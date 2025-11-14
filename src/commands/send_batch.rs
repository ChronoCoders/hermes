use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::progress;
use crate::transfer::SftpClient;
use crate::ui;
use chrono::Local;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn execute(
    file_paths: Vec<String>,
    password: Option<&str>,
    ttl_hours: Option<u64>,
    recipients: Option<Vec<String>>,
) -> Result<()> {
    if file_paths.is_empty() {
        return Err(HermesError::ConfigError(
            "No files specified for batch operation".to_string(),
        ));
    }

    ui::print_box_start("BATCH_ENCRYPT");
    ui::print_box_line(&format!(">> Files to encrypt: {}", file_paths.len()));
    ui::print_box_line("");

    // Load config and connect once
    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;

    let mut successful = 0;
    let mut failed = 0;
    let mut results = Vec::new();

    for (index, file_path) in file_paths.iter().enumerate() {
        ui::print_box_line(&format!(
            ">> [{}/{}] Processing: {}",
            index + 1,
            file_paths.len(),
            file_path
        ));

        match process_single_file(
            file_path,
            password,
            ttl_hours,
            &recipients,
            &config,
            &client,
        ) {
            Ok(remote_path) => {
                ui::print_box_line(&format!("   ✓ Success: {}", remote_path));
                successful += 1;
                results.push((file_path.clone(), true, Some(remote_path)));
            }
            Err(e) => {
                ui::print_box_line(&format!("   ✗ Failed: {}", e));
                failed += 1;
                results.push((file_path.clone(), false, None));
            }
        }
        ui::print_box_line("");
    }

    ui::print_box_end();

    println!();
    ui::print_success("BATCH OPERATION COMPLETE");
    ui::print_info("Total Files", &file_paths.len().to_string());
    ui::print_info("Successful", &successful.to_string());
    ui::print_info("Failed", &failed.to_string());

    if failed > 0 {
        println!("\n❌ Failed files:");
        for (path, success, _) in &results {
            if !success {
                println!("   • {}", path);
            }
        }
    }

    ui::print_status("COMPLETE");
    println!();

    if failed > 0 {
        Err(HermesError::ConfigError(format!(
            "{} files failed to encrypt",
            failed
        )))
    } else {
        Ok(())
    }
}

fn process_single_file(
    file_path: &str,
    password: Option<&str>,
    ttl_hours: Option<u64>,
    recipients: &Option<Vec<String>>,
    config: &Settings,
    client: &SftpClient,
) -> Result<String> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(file_path.to_string()));
    }

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| HermesError::FileNotFound("Invalid filename".to_string()))?;

    // Read file
    let mut file = File::open(path)?;
    let file_size = file.metadata()?.len();
    let mut plaintext = Vec::new();

    if file_size > 1024 * 1024 {
        // Show progress for files > 1MB
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

    // Encrypt
    let encrypted = if let Some(recips) = recipients {
        crypto::encrypt::encrypt_data_multi(
            &plaintext,
            None,
            Some(filename.to_string()),
            ttl_hours,
            Some(recips.clone()),
        )?
    } else if let Some(pwd) = password {
        crypto::encrypt_data(&plaintext, pwd, Some(filename.to_string()), ttl_hours)?
    } else {
        return Err(HermesError::ConfigError(
            "Either password or recipients required".to_string(),
        ));
    };

    // Upload
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let final_path = format!(
        "{}/{}_{}.enc",
        config.paths.files,
        path.file_stem().unwrap().to_str().unwrap(),
        timestamp
    );

    client.upload(&encrypted, &final_path)?;

    Ok(final_path)
}
