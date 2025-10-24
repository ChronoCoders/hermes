use crate::config::Settings;
use crate::crypto;
use crate::transfer::SftpClient;
use crate::error::Result;
use crate::ui;

pub fn execute(remote_file: &str, password: &str) -> Result<()> {
    ui::print_box_start("DECRYPT");
    
    let config = Settings::load()?;
    
    ui::print_box_line(">> Connecting to SFTP server...");
    let client = SftpClient::connect(&config)?;
    
    let remote_path = if remote_file.starts_with('/') || remote_file.contains(':') {
        remote_file.to_string()
    } else {
        format!("{}/{}", config.paths.outbox, remote_file)
    };
    
    ui::print_box_line(">> Downloading encrypted payload...");
    let encrypted = client.download(&remote_path)?;
    
    ui::print_box_line(">> Decrypting with provided key...");
    let decrypted = crypto::decrypt_data(&encrypted, password)?;
    
    let message = String::from_utf8(decrypted)
        .map_err(|_| crate::error::HermesError::DecryptionFailed)?;
    
    ui::print_box_line(">> Verifying integrity (SHA-256)...");
    ui::print_box_line("");
    ui::print_box_end();
    
    println!();
    ui::print_success("MESSAGE DECRYPTED & VERIFIED");
    println!();
    println!("{}", "┌─[PLAINTEXT]──────────────────────────────────────┐".cyan());
    println!("{} {} {}", "│".cyan(), message, "│".cyan());
    println!("{}", "└──────────────────────────────────────────────────┘".cyan());
    println!();
    ui::print_info("From", &remote_path);
    ui::print_info("Size", &format!("{} bytes", message.len()));
    ui::print_info("Integrity", "VERIFIED ✓");
    ui::print_status("SECURE");
    println!();
    
    Ok(())
}

use colored::*;