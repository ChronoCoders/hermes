use crate::chunking::{reassemble_chunks_from_manifest, ChunkManifest};
use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::progress::ProgressTracker;
use crate::transfer::SftpClient;
use crate::ui;
use std::fs;
use std::path::PathBuf;

pub fn execute(
    remote_manifest: &str,
    password: Option<&str>,
    output: Option<&str>,
    recipient_name: Option<&str>,
) -> Result<()> {
    ui::print_box_start("CHUNKED_DECRYPT");
    ui::print_box_line(&format!(">> Manifest: {}", remote_manifest));
    ui::print_box_line("");

    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;

    let remote_path = if remote_manifest.starts_with('/') || remote_manifest.contains(':') {
        remote_manifest.to_string()
    } else {
        format!("{}/{}", config.paths.files, remote_manifest)
    };

    ui::print_box_line(">> Step 1/4: Downloading manifest...");
    let encrypted_manifest = client.download(&remote_path)?;

    let manifest_data = if let Some(name) = recipient_name {
        crypto::decrypt::decrypt_data_multi(&encrypted_manifest, name)?
    } else if let Some(pwd) = password {
        crypto::decrypt_data(&encrypted_manifest, pwd)?
    } else {
        return Err(HermesError::ConfigError(
            "Password or recipient name required".to_string(),
        ));
    };

    let manifest_json =
        String::from_utf8(manifest_data).map_err(|_| HermesError::DecryptionFailed)?;

    let manifest = ChunkManifest::from_json(&manifest_json)?;
    ui::print_box_line(&format!(
        "   ✓ Manifest loaded: {} chunks",
        manifest.total_chunks
    ));
    ui::print_box_line(&format!(
        "   ✓ Original file: {}",
        manifest.original_filename
    ));
    ui::print_box_line("");

    let temp_dir = PathBuf::from("./hermes_chunks_temp");
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    ui::print_box_line(">> Step 2/4: Downloading and decrypting chunks...");

    for (index, chunk) in manifest.chunks.iter().enumerate() {
        let progress_msg = format!(
            "[{}/{}] Chunk {}",
            index + 1,
            manifest.total_chunks,
            chunk.index + 1
        );
        ui::print_box_line(&format!("   {}", progress_msg));

        let encrypted_chunk = client.download(&chunk.encrypted_path)?;

        let decrypted_chunk = if let Some(name) = recipient_name {
            crypto::decrypt::decrypt_data_multi(&encrypted_chunk, name)?
        } else if let Some(pwd) = password {
            crypto::decrypt_data(&encrypted_chunk, pwd)?
        } else {
            return Err(HermesError::ConfigError(
                "Password or recipient name required".to_string(),
            ));
        };

        let local_chunk_path = temp_dir.join(format!(
            "{}.chunk.{:03}",
            manifest.original_filename,
            chunk.index + 1
        ));
        fs::write(&local_chunk_path, &decrypted_chunk)?;
    }

    ui::print_box_line("");
    ui::print_box_line(">> Step 3/4: Reassembling file...");
    let reassemble_progress = ProgressTracker::new_spinner("Reassembling");

    let output_path = if let Some(out) = output {
        PathBuf::from(out)
    } else {
        PathBuf::from(&manifest.original_filename)
    };

    reassemble_chunks_from_manifest(&manifest, &temp_dir, &output_path)?;
    reassemble_progress.finish_and_clear();
    ui::print_box_line(&format!("   ✓ File reassembled: {}", output_path.display()));

    ui::print_box_line("");
    ui::print_box_line(">> Step 4/4: Cleanup...");
    fs::remove_dir_all(&temp_dir)?;
    ui::print_box_line("   ✓ Temporary files removed");

    ui::print_box_end();

    println!();
    ui::print_success("CHUNKED DECRYPTION COMPLETE");
    ui::print_info("Output File", &output_path.display().to_string());
    ui::print_info("File Hash", &manifest.file_hash);
    ui::print_info("Total Size", &format!("{} bytes", manifest.total_size));
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
