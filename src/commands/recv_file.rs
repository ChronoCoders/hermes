use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::transfer::SftpClient;
use crate::ui;
use std::fs::File;
use std::io::Write;

pub fn execute(remote_file: &str, password: &str, output_path: Option<&str>) -> Result<()> {
    ui::print_box_start("FILE_DECRYPT");

    let config = Settings::load()?;

    ui::print_box_line(">> Connecting to SFTP server...");
    let client = SftpClient::connect(&config)?;

    let remote_path = if remote_file.starts_with('/') || remote_file.contains(':') {
        remote_file.to_string()
    } else {
        format!("{}/{}", config.paths.files, remote_file)
    };

    ui::print_box_line(">> Downloading encrypted file...");
    let encrypted = client.download(&remote_path)?;

    let package = crate::crypto::encrypt::EncryptedPackage::from_bytes(&encrypted)?;

    if package.is_expired() {
        ui::print_box_line("");
        ui::print_box_end();
        println!();
        ui::print_error("FILE EXPIRED");
        ui::print_info("Status", "Self-destructed ⚠️");
        println!();
        return Err(HermesError::DecryptionFailed);
    }

    ui::print_box_line(">> Decrypting and decompressing...");
    let decrypted = crypto::decrypt_data(&encrypted, password)?;

    ui::print_box_line(">> Verifying file integrity...");

    let filename = package
        .filename
        .clone()
        .unwrap_or_else(|| "decrypted_file".to_string());

    let output = output_path.unwrap_or(&filename);

    ui::print_box_line(">> Writing to disk...");
    let mut file = File::create(output)?;
    file.write_all(&decrypted)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("FILE DECRYPTED & VERIFIED");
    ui::print_info("Output", output);
    ui::print_info(
        "Size",
        &format!("{:.2} MB", decrypted.len() as f64 / 1024.0 / 1024.0),
    );
    ui::print_info("Integrity", "VERIFIED ✓");
    if package.compressed() {
        ui::print_info("Decompressed", "Yes");
    }
    if package.expires_at > 0 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let remaining = (package.expires_at as i64 - now as i64) / 3600;
        if remaining > 0 {
            ui::print_info("Expires", &format!("in {} hours", remaining));
        }
    }
    ui::print_status("UNLOCKED");
    println!();

    Ok(())
}
