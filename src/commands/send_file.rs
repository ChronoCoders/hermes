use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
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
) -> Result<()> {
    ui::print_box_start("FILE_ENCRYPT");

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(file_path.to_string()));
    }

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| HermesError::FileNotFound("Invalid filename".to_string()))?;

    ui::print_box_line(&format!(">> Target: {}", filename));

    let mut file = File::open(path)?;
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext)?;

    let original_size = plaintext.len();

    ui::print_box_line(&format!(
        ">> Size: {:.2} MB",
        original_size as f64 / 1024.0 / 1024.0
    ));

    let encrypted = if let Some(recips) = recipients {
        ui::print_box_line(&format!(">> Recipients: {}", recips.join(", ")));
        ui::print_box_line(">> Compressing and encrypting with RSA hybrid...");
        crypto::encrypt::encrypt_data_multi(
            &plaintext,
            None,
            Some(filename.to_string()),
            ttl_hours,
            Some(recips),
        )?
    } else if let Some(pwd) = password {
        ui::print_box_line(">> Compressing and encrypting...");
        crypto::encrypt_data(&plaintext, pwd, Some(filename.to_string()), ttl_hours)?
    } else {
        return Err(HermesError::ConfigError(
            "Either password or recipients required".to_string(),
        ));
    };

    let package = crypto::encrypt::EncryptedPackage::from_bytes(&encrypted)?;

    if package.compressed() {
        let ratio = (1.0 - (encrypted.len() as f64 / original_size as f64)) * 100.0;
        ui::print_box_line(&format!(">> Compressed: {:.1}% reduction", ratio));
    }

    if let Some(hours) = ttl_hours {
        ui::print_box_line(&format!(">> Self-destruct: {} hours", hours));
    }

    ui::print_box_line(">> Uploading to SFTP vault...");

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

    client.upload(&encrypted, &final_path)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("FILE SECURED");
    ui::print_info("Remote", &final_path);
    ui::print_info(
        "Original",
        &format!("{:.2} MB", original_size as f64 / 1024.0 / 1024.0),
    );
    ui::print_info(
        "Encrypted",
        &format!("{:.2} MB", encrypted.len() as f64 / 1024.0 / 1024.0),
    );
    if package.compressed() {
        ui::print_info("Compression", "GZIP");
    }
    if package.is_multi_recipient() {
        ui::print_info("Type", "Multi-recipient (RSA+AES)");
    }
    if let Some(hours) = ttl_hours {
        ui::print_info("Expires", &format!("in {} hours", hours));
    } else {
        ui::print_info("Expires", "Never");
    }
    ui::print_status("LOCKED");
    println!();

    Ok(())
}
