use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::progress;
use crate::transfer::SftpClient;
use crate::ui;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn execute(
    remote_files: Vec<String>,
    password: Option<&str>,
    output_dir: Option<&str>,
    recipient_name: Option<&str>,
) -> Result<()> {
    if remote_files.is_empty() {
        return Err(HermesError::ConfigError(
            "No files specified for batch operation".to_string(),
        ));
    }

    ui::print_box_start("BATCH_DECRYPT");
    ui::print_box_line(&format!(">> Files to decrypt: {}", remote_files.len()));
    ui::print_box_line("");

    // Create output directory if specified
    let output_path = if let Some(dir) = output_dir {
        let path = PathBuf::from(dir);
        std::fs::create_dir_all(&path)?;
        Some(path)
    } else {
        None
    };

    // Load config and connect once
    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;

    let mut successful = 0;
    let mut failed = 0;
    let mut results = Vec::new();

    for (index, remote_file) in remote_files.iter().enumerate() {
        ui::print_box_line(&format!(
            ">> [{}/{}] Processing: {}",
            index + 1,
            remote_files.len(),
            remote_file
        ));

        match process_single_file(
            remote_file,
            password,
            recipient_name,
            &output_path,
            &config,
            &client,
        ) {
            Ok(output_file) => {
                ui::print_box_line(&format!("   ✓ Saved: {output_file}"));
                successful += 1;
                results.push((remote_file.clone(), true, Some(output_file)));
            }
            Err(e) => {
                ui::print_box_line(&format!("   ✗ Failed: {e}"));
                failed += 1;
                results.push((remote_file.clone(), false, None));
            }
        }
        ui::print_box_line("");
    }

    ui::print_box_end();

    println!();
    ui::print_success("BATCH DECRYPTION COMPLETE");
    ui::print_info("Total Files", &remote_files.len().to_string());
    ui::print_info("Successful", &successful.to_string());
    ui::print_info("Failed", &failed.to_string());

    if failed > 0 {
        println!("\n❌ Failed files:");
        for (path, success, _) in &results {
            if !success {
                println!("   • {path}");
            }
        }
    }

    ui::print_status("COMPLETE");
    println!();

    if failed > 0 {
        Err(HermesError::ConfigError(format!(
            "{failed} files failed to decrypt"
        )))
    } else {
        Ok(())
    }
}

fn process_single_file(
    remote_file: &str,
    password: Option<&str>,
    recipient_name: Option<&str>,
    output_dir: &Option<PathBuf>,
    config: &Settings,
    client: &SftpClient,
) -> Result<String> {
    // Download
    let remote_path = if remote_file.starts_with('/') || remote_file.contains(':') {
        remote_file.to_string()
    } else {
        format!("{}/{}", config.paths.files, remote_file)
    };

    let encrypted = client.download(&remote_path)?;

    let package = crate::crypto::encrypt::EncryptedPackage::from_bytes(&encrypted)?;

    if package.is_expired() {
        return Err(HermesError::DecryptionFailed);
    }

    // Decrypt
    let decrypted = if package.is_multi_recipient() {
        if let Some(name) = recipient_name {
            let spinner = progress::ProgressTracker::new_spinner("🔓 Decrypting");
            spinner.set_message("Processing...".to_string());

            let result = crypto::decrypt::decrypt_data_multi(&encrypted, name)?;

            spinner.finish_and_clear();
            result
        } else {
            return Err(HermesError::ConfigError(
                "Recipient name required for multi-recipient file".to_string(),
            ));
        }
    } else if let Some(pwd) = password {
        let spinner = progress::ProgressTracker::new_spinner("🔓 Decrypting");
        spinner.set_message("Processing...".to_string());

        let result = crypto::decrypt_data(&encrypted, pwd)?;

        spinner.finish_and_clear();
        result
    } else {
        return Err(HermesError::ConfigError(
            "Password required for password-encrypted file".to_string(),
        ));
    };

    // Save to file
    let filename = package
        .filename
        .clone()
        .unwrap_or_else(|| "decrypted_file".to_string());

    let output_path = if let Some(dir) = output_dir {
        dir.join(&filename)
    } else {
        PathBuf::from(&filename)
    };

    let mut file = File::create(&output_path)?;

    if decrypted.len() > 1024 * 1024 {
        let progress = progress::ProgressTracker::new(decrypted.len() as u64, "💾 Writing");

        let chunk_size = 8192;
        for (i, chunk) in decrypted.chunks(chunk_size).enumerate() {
            file.write_all(chunk)?;
            progress.set_position((i * chunk_size) as u64);
        }

        progress.finish_and_clear();
    } else {
        file.write_all(&decrypted)?;
    }

    Ok(output_path.to_string_lossy().to_string())
}
