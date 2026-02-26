use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::progress;
use crate::transfer::SftpClient;
use crate::ui;
use chrono::Local;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};

pub fn execute(
    dir_path: &str,
    password: Option<&str>,
    ttl_hours: Option<u64>,
    recipients: Option<Vec<String>>,
    recursive: bool,
) -> Result<()> {
    let path = Path::new(dir_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(dir_path.to_string()));
    }

    if !path.is_dir() {
        return Err(HermesError::ConfigError(format!(
            "Not a directory: {dir_path}"
        )));
    }

    ui::print_box_start("DIRECTORY_ENCRYPT");
    ui::print_box_line(&format!(">> Directory: {dir_path}"));
    ui::print_box_line(&format!(">> Recursive: {recursive}"));
    ui::print_box_line("");

    // Collect all files
    let files = collect_files(path, recursive)?;
    ui::print_box_line(&format!(">> Found {} files", files.len()));
    ui::print_box_line("");

    if files.is_empty() {
        ui::print_box_end();
        println!();
        ui::print_error("NO FILES FOUND");
        return Ok(());
    }

    // Load config and connect once
    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;

    let mut successful = 0;
    let mut failed = 0;
    let mut results = Vec::new();

    for (index, file_path) in files.iter().enumerate() {
        let relative_path = file_path
            .strip_prefix(path)
            .unwrap_or(file_path)
            .to_string_lossy();

        ui::print_box_line(&format!(
            ">> [{}/{}] {}",
            index + 1,
            files.len(),
            relative_path
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
                ui::print_box_line(&format!("   ✓ {remote_path}"));
                successful += 1;
                results.push((file_path.clone(), true, Some(remote_path)));
            }
            Err(e) => {
                ui::print_box_line(&format!("   ✗ {e}"));
                failed += 1;
                results.push((file_path.clone(), false, None));
            }
        }
    }

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("DIRECTORY ENCRYPTION COMPLETE");
    ui::print_info("Total Files", &files.len().to_string());
    ui::print_info("Successful", &successful.to_string());
    ui::print_info("Failed", &failed.to_string());

    if failed > 0 {
        println!("\n❌ Failed files:");
        for (path, success, _) in &results {
            if !success {
                println!("   • {}", path.display());
            }
        }
    }

    ui::print_status("COMPLETE");
    println!();

    if failed > 0 {
        Err(HermesError::ConfigError(format!(
            "{failed} files failed to encrypt"
        )))
    } else {
        Ok(())
    }
}

fn collect_files(path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    if recursive {
        collect_files_recursive(path, &mut files)?;
    } else {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            }
        }
    }

    Ok(files)
}

fn collect_files_recursive(path: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            files.push(path);
        } else if path.is_dir() {
            collect_files_recursive(&path, files)?;
        }
    }
    Ok(())
}

fn process_single_file(
    file_path: &Path,
    password: Option<&str>,
    ttl_hours: Option<u64>,
    recipients: &Option<Vec<String>>,
    config: &Settings,
    client: &SftpClient,
) -> Result<String> {
    let filename = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| HermesError::FileNotFound("Invalid filename".to_string()))?;

    // Read file
    let mut file = File::open(file_path)?;
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
        progress.finish_and_clear();
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
        file_path.file_stem().unwrap().to_str().unwrap(),
        timestamp
    );

    client.upload(&encrypted, &final_path)?;

    Ok(final_path)
}
