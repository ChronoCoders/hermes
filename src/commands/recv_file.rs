use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::progress;
use crate::transfer::SftpClient;
use crate::ui;
use std::fs::File;
use std::io::Write;

pub fn execute(
    remote_file: &str,
    password: Option<&str>,
    output_path: Option<&str>,
    recipient_name: Option<&str>,
) -> Result<()> {
    ui::print_box_start("FILE_DECRYPT");

    let config = Settings::load()?;
    let encrypted = download_encrypted_file(&config, remote_file)?;
    let package = crate::crypto::encrypt::EncryptedPackage::from_bytes(&encrypted)?;

    check_expiration(&package)?;

    ui::print_box_line(">> Decrypting and decompressing...");
    let decrypted = decrypt_file_data(&encrypted, &package, password, recipient_name, remote_file)?;

    ui::print_box_line(">> Verifying file integrity...");

    let filename = package
        .filename
        .clone()
        .unwrap_or_else(|| "decrypted_file".to_string());
    let output = output_path.unwrap_or(&filename);

    write_decrypted_file(output, &decrypted)?;

    ui::print_box_line("");
    ui::print_box_end();

    display_success_info(output, &decrypted, &package);

    Ok(())
}

fn download_encrypted_file(config: &Settings, remote_file: &str) -> Result<Vec<u8>> {
    ui::print_box_line(">> Connecting to SFTP server...");
    let client = SftpClient::connect(config)?;

    let remote_path = if remote_file.starts_with('/') || remote_file.contains(':') {
        remote_file.to_string()
    } else {
        format!("{}/{}", config.paths.files, remote_file)
    };

    ui::print_box_line(">> Downloading encrypted file...");
    let encrypted = client.download(&remote_path)?;

    if encrypted.len() > 1024 * 1024 {
        let progress = progress::create_download_progress(encrypted.len() as u64);
        progress.finish_with_message("✓ Download complete".to_string());
    }

    Ok(encrypted)
}

fn check_expiration(package: &crate::crypto::encrypt::EncryptedPackage) -> Result<()> {
    if package.is_expired() {
        ui::print_box_line("");
        ui::print_box_end();
        println!();
        ui::print_error("FILE EXPIRED");
        ui::print_info("Status", "Self-destructed ⚠️");
        println!();
        return Err(HermesError::DecryptionFailed);
    }
    Ok(())
}

fn decrypt_file_data(
    encrypted: &[u8],
    package: &crate::crypto::encrypt::EncryptedPackage,
    password: Option<&str>,
    recipient_name: Option<&str>,
    remote_file: &str,
) -> Result<Vec<u8>> {
    if package.is_multi_recipient() {
        decrypt_multi_recipient(encrypted, package, recipient_name, remote_file)
    } else {
        decrypt_with_password(encrypted, password)
    }
}

fn decrypt_multi_recipient(
    encrypted: &[u8],
    package: &crate::crypto::encrypt::EncryptedPackage,
    recipient_name: Option<&str>,
    remote_file: &str,
) -> Result<Vec<u8>> {
    let name = recipient_name.ok_or_else(|| {
        ui::print_box_line("");
        ui::print_box_end();
        println!();
        ui::print_error("MULTI-RECIPIENT FILE");
        ui::print_info(
            "Recipients",
            &package
                .recipients
                .iter()
                .map(|r| r.name.clone())
                .collect::<Vec<_>>()
                .join(", "),
        );
        println!();
        println!("Use: hermes recv-file {remote_file} --recipient <n>");
        println!();
        HermesError::ConfigError("Recipient name required for multi-recipient file".to_string())
    })?;

    ui::print_box_line(&format!(">> Using recipient key: {name}"));
    decrypt_with_progress(encrypted, |data| crypto::decrypt::decrypt_data_multi(data, name))
}

fn decrypt_with_password(encrypted: &[u8], password: Option<&str>) -> Result<Vec<u8>> {
    let pwd = password.ok_or_else(|| {
        HermesError::ConfigError("Password required for password-encrypted file".to_string())
    })?;

    decrypt_with_progress(encrypted, |data| crypto::decrypt_data(data, pwd))
}

fn decrypt_with_progress<F>(encrypted: &[u8], decrypt_fn: F) -> Result<Vec<u8>>
where
    F: FnOnce(&[u8]) -> Result<Vec<u8>>,
{
    if encrypted.len() > 1024 * 1024 {
        let spinner = progress::ProgressTracker::new_spinner("🔓 Decrypting");
        spinner.set_message("Processing...".to_string());
        let result = decrypt_fn(encrypted)?;
        spinner.finish_with_message("✓ Decryption complete".to_string());
        Ok(result)
    } else {
        decrypt_fn(encrypted)
    }
}

fn write_decrypted_file(output: &str, decrypted: &[u8]) -> Result<()> {
    ui::print_box_line(">> Writing to disk...");
    let mut file = File::create(output)?;

    if decrypted.len() > 1024 * 1024 {
        let progress = progress::ProgressTracker::new(decrypted.len() as u64, "💾 Writing");
        let chunk_size = 8192;

        for (i, chunk) in decrypted.chunks(chunk_size).enumerate() {
            file.write_all(chunk)?;
            progress.set_position((i * chunk_size) as u64);
        }

        progress.finish_with_message("✓ File saved".to_string());
    } else {
        file.write_all(decrypted)?;
    }

    Ok(())
}

fn display_success_info(output: &str, decrypted: &[u8], package: &crate::crypto::encrypt::EncryptedPackage) {
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

    if package.is_multi_recipient() {
        ui::print_info("Type", "Multi-recipient");
    }

    if package.expires_at > 0 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let remaining = (package.expires_at as i64 - now as i64) / 3600;
        if remaining > 0 {
            ui::print_info("Expires", &format!("in {remaining} hours"));
        }
    }

    ui::print_status("UNLOCKED");
    println!();
}
