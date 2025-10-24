use crate::config::Settings;
use crate::crypto;
use crate::transfer::SftpClient;
use crate::error::{HermesError, Result};
use crate::ui;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use chrono::Local;

pub fn execute(file_path: &str, password: &str) -> Result<()> {
    ui::print_box_start("FILE_ENCRYPT");
    
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(file_path.to_string()));
    }
    
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| HermesError::FileNotFound("Invalid filename".to_string()))?;
    
    ui::print_box_line(&format!(">> Target: {}", filename));
    
    let mut file = File::open(path)?;
    let mut plaintext = Vec::new();
    file.read_to_end(&mut plaintext)?;
    
    let original_size = plaintext.len();
    
    ui::print_box_line(&format!(">> Size: {:.2} MB", original_size as f64 / 1024.0 / 1024.0));
    ui::print_box_line(">> Compressing and encrypting...");
    
    let encrypted = crypto::encrypt_data(&plaintext, password, Some(filename.to_string()))?;
    
    let package: crypto::EncryptedPackage = serde_json::from_slice(&encrypted)?;
    
    if package.compressed {
        let ratio = (1.0 - (encrypted.len() as f64 / original_size as f64)) * 100.0;
        ui::print_box_line(&format!(">> Compressed: {:.1}% reduction", ratio));
    }
    
    ui::print_box_line(">> Uploading to SFTP vault...");
    
    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;
    
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let remote_path = format!("{}/{}_{}.enc", config.paths.files, 
        path.file_stem().unwrap().to_str().unwrap(), timestamp);
    
    client.upload(&encrypted, &remote_path)?;
    
    ui::print_box_line("");
    ui::print_box_end();
    
    println!();
    ui::print_success("FILE SECURED");
    ui::print_info("Remote", &remote_path);
    ui::print_info("Original", &format!("{:.2} MB", original_size as f64 / 1024.0 / 1024.0));
    ui::print_info("Encrypted", &format!("{:.2} MB", encrypted.len() as f64 / 1024.0 / 1024.0));
    if package.compressed {
        ui::print_info("Compression", "GZIP");
    }
    ui::print_status("LOCKED");
    println!();
    
    Ok(())
}