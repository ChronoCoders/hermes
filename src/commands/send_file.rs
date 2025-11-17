use crate::config::Settings;
use crate::crypto;
use crate::dms::{get_registry_path, DeadManSwitch, DmsRegistry};
use crate::error::{HermesError, Result};
use crate::progress;
use crate::transfer::SftpClient;
use crate::ui;
use chrono::Local;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn execute(
    file_path: &str,
    password: Option<&str>,
    remote_path: Option<&str>,
    ttl_hours: Option<u64>,
    recipients: Option<Vec<String>>,
    dms_hours: Option<u64>,
    use_pqc: bool,
) -> Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(file_path.to_string()));
    }

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| HermesError::FileNotFound("Invalid filename".to_string()))?;

    let title = if use_pqc { "ENCRYPT_PQC" } else { "ENCRYPT" };
    ui::print_box_start(title);
    ui::print_box_line(&format!(">> File: {}", filename));
    if use_pqc {
        ui::print_box_line(">> Mode: Hybrid RSA + Kyber (Post-Quantum)");
    }
    if let Some(hours) = dms_hours {
        ui::print_box_line(&format!(">> Dead Man's Switch: {} hours", hours));
    }
    ui::print_box_line("");

    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let mut plaintext = Vec::new();

    if file_size > 1024 * 1024 {
        let progress = progress::create_encryption_progress(file_size);
        let mut buffer = vec![0u8; 8192];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            plaintext.extend_from_slice(&buffer[..bytes_read]);
            progress.inc(bytes_read as u64);
        }
        progress.finish_with_message("✓ Read complete".to_string());
    } else {
        file.read_to_end(&mut plaintext)?;
    }

    let spinner = progress::ProgressTracker::new_spinner("Encrypting");
    spinner.set_message("Processing...".to_string());

    let encrypted = if let Some(recips) = recipients {
        if use_pqc {
            spinner.set_message("Encrypting with RSA + Kyber...".to_string());
        }
        crypto::encrypt::encrypt_data_multi(
            &plaintext,
            None,
            Some(filename.to_string()),
            ttl_hours,
            Some(recips),
            use_pqc,
        )?
    } else if let Some(pwd) = password {
        if use_pqc {
            return Err(HermesError::ConfigError(
                "PQC mode requires recipients, not password".to_string(),
            ));
        }
        crypto::encrypt_data(&plaintext, pwd, Some(filename.to_string()), ttl_hours)?
    } else {
        return Err(HermesError::ConfigError(
            "Either password or recipients required".to_string(),
        ));
    };

    spinner.finish_and_clear();

    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;

    let final_path = if let Some(custom_path) = remote_path {
        custom_path.to_string()
    } else {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        format!(
            "{}/{}_{}.enc",
            config.paths.files,
            path.file_stem().unwrap().to_str().unwrap(),
            timestamp
        )
    };

    let upload_progress = progress::create_upload_progress(encrypted.len() as u64);
    client.upload(&encrypted, &final_path)?;
    upload_progress.finish_with_message("✓ Upload complete".to_string());

    if let Some(hours) = dms_hours {
        let registry_path = get_registry_path()?;
        let mut registry = DmsRegistry::load_from_file(&registry_path)?;

        let dms = DeadManSwitch::new(final_path.clone(), hours);
        registry.add(dms);
        registry.save_to_file(&registry_path)?;

        ui::print_box_line("");
        ui::print_box_line(&format!(
            ">> DMS enabled: Check-in required every {} hours",
            hours
        ));
    }

    ui::print_box_end();

    println!();
    ui::print_success("ENCRYPTION COMPLETE");
    ui::print_info("Remote Path", &final_path);
    if let Some(hours) = dms_hours {
        ui::print_info("DMS Timeout", &format!("{} hours", hours));
    }
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
