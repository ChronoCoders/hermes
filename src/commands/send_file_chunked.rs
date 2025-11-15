use crate::chunking::split_file_into_chunks;
use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::progress::ProgressTracker;
use crate::transfer::SftpClient;
use crate::ui;
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

pub fn execute(
    file_path: &str,
    password: Option<&str>,
    ttl_hours: Option<u64>,
    recipients: Option<Vec<String>>,
) -> Result<()> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(file_path.to_string()));
    }

    let file_size = path.metadata()?.len();
    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| HermesError::FileNotFound("Invalid filename".to_string()))?;

    ui::print_box_start("CHUNKED_ENCRYPT");
    ui::print_box_line(&format!(">> File: {}", filename));
    ui::print_box_line(&format!(
        ">> Size: {} bytes ({:.2} MB)",
        file_size,
        file_size as f64 / 1024.0 / 1024.0
    ));
    ui::print_box_line("");

    let temp_dir = PathBuf::from("./hermes_chunks_temp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    ui::print_box_line(">> Step 1/4: Splitting file into chunks...");
    let split_progress = ProgressTracker::new_spinner("Splitting");
    let manifest = split_file_into_chunks(path, &temp_dir)?;
    split_progress.finish_and_clear();
    ui::print_box_line(&format!("   ✓ Created {} chunks", manifest.total_chunks));
    ui::print_box_line("");

    ui::print_box_line(">> Step 2/4: Encrypting chunks...");
    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let base_remote_path = format!("{}/{}_{}", config.paths.files, filename, timestamp);

    let mut encrypted_manifest = manifest.clone();
    encrypted_manifest.chunks.clear();

    for (index, chunk) in manifest.chunks.iter().enumerate() {
        let progress_msg = format!(
            "[{}/{}] Chunk {}",
            index + 1,
            manifest.total_chunks,
            chunk.index + 1
        );
        ui::print_box_line(&format!("   {}", progress_msg));

        let chunk_path = temp_dir.join(&chunk.encrypted_path);
        let chunk_data = fs::read(&chunk_path)?;

        let encrypted = if let Some(recips) = &recipients {
            crypto::encrypt::encrypt_data_multi(
                &chunk_data,
                None,
                Some(chunk.encrypted_path.clone()),
                ttl_hours,
                Some(recips.clone()),
            )?
        } else if let Some(pwd) = password {
            crypto::encrypt_data(
                &chunk_data,
                pwd,
                Some(chunk.encrypted_path.clone()),
                ttl_hours,
            )?
        } else {
            return Err(HermesError::ConfigError(
                "Either password or recipients required".to_string(),
            ));
        };

        let remote_chunk_path = format!("{}.chunk.{:03}.enc", base_remote_path, chunk.index + 1);
        client.upload(&encrypted, &remote_chunk_path)?;

        let mut encrypted_chunk = chunk.clone();
        encrypted_chunk.encrypted_path = remote_chunk_path;
        encrypted_manifest.add_chunk(encrypted_chunk);

        fs::remove_file(&chunk_path)?;
    }

    ui::print_box_line("");
    ui::print_box_line(">> Step 3/4: Creating manifest...");

    let manifest_json = encrypted_manifest.to_json()?;
    let encrypted_manifest_data = if let Some(recips) = &recipients {
        crypto::encrypt::encrypt_data_multi(
            manifest_json.as_bytes(),
            None,
            Some(format!("{}.manifest", filename)),
            ttl_hours,
            Some(recips.clone()),
        )?
    } else if let Some(pwd) = password {
        crypto::encrypt_data(
            manifest_json.as_bytes(),
            pwd,
            Some(format!("{}.manifest", filename)),
            ttl_hours,
        )?
    } else {
        return Err(HermesError::ConfigError(
            "Either password or recipients required".to_string(),
        ));
    };

    let remote_manifest_path = format!("{}.manifest.enc", base_remote_path);
    client.upload(&encrypted_manifest_data, &remote_manifest_path)?;
    ui::print_box_line(&format!("   ✓ Manifest: {}", remote_manifest_path));

    ui::print_box_line("");
    ui::print_box_line(">> Step 4/4: Cleanup...");
    fs::remove_dir_all(&temp_dir)?;
    ui::print_box_line("   ✓ Temporary files removed");

    ui::print_box_end();

    println!();
    ui::print_success("CHUNKED ENCRYPTION COMPLETE");
    ui::print_info("Original File", filename);
    ui::print_info("Total Chunks", &encrypted_manifest.total_chunks.to_string());
    ui::print_info("File Hash", &encrypted_manifest.file_hash);
    ui::print_info("Manifest", &remote_manifest_path);
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
