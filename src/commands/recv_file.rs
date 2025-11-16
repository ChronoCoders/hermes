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

    ui::print_box_line(">> Connecting to SFTP server...");
    let client = SftpClient::connect(&config)?;

    let remote_path = if remote_file.starts_with('/') || remote_file.contains(':') {
        remote_file.to_string()
    } else {
        format!("{}/{}", config.paths.files, remote_file)
    };

    // Download with progress
    ui::print_box_line(">> Downloading encrypted file...");
    let encrypted = client.download(&remote_path)?;

    if encrypted.len() > 1024 * 1024 {
        let progress = progress::create_download_progress(encrypted.len() as u64);
        progress.finish_with_message("✓ Download complete".to_string());
    }

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

    // Decrypt with progress
    ui::print_box_line(">> Decrypting and decompressing...");

    let decrypted = if package.is_multi_recipient() {
        if let Some(name) = recipient_name {
            ui::print_box_line(&format!(">> Using recipient key: {name}"));

            if encrypted.len() > 1024 * 1024 {
                let spinner = progress::ProgressTracker::new_spinner("🔓 Decrypting");
                spinner.set_message("Processing...".to_string());

                let result = crypto::decrypt::decrypt_data_multi(&encrypted, name)?;

                spinner.finish_with_message("✓ Decryption complete".to_string());
                result
            } else {
                crypto::decrypt::decrypt_data_multi(&encrypted, name)?
            }
        } else {
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
            return Err(HermesError::ConfigError(
                "Recipient name required for multi-recipient file".to_string(),
            ));
        }
    } else if let Some(pwd) = password {
        if encrypted.len() > 1024 * 1024 {
            let spinner = progress::ProgressTracker::new_spinner("🔓 Decrypting");
            spinner.set_message("Processing...".to_string());

            let result = crypto::decrypt_data(&encrypted, pwd)?;

            spinner.finish_with_message("✓ Decryption complete".to_string());
            result
        } else {
            crypto::decrypt_data(&encrypted, pwd)?
        }
    } else {
        return Err(HermesError::ConfigError(
            "Password required for password-encrypted file".to_string(),
        ));
    };

    ui::print_box_line(">> Verifying file integrity...");

    let filename = package
        .filename
        .clone()
        .unwrap_or_else(|| "decrypted_file".to_string());

    let output = output_path.unwrap_or(&filename);

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
        file.write_all(&decrypted)?;
    }

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

    Ok(())
}
