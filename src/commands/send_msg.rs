use crate::config::Settings;
use crate::crypto;
use crate::error::Result;
use crate::transfer::SftpClient;
use crate::ui;
use chrono::Local;

pub fn execute(message: &str, password: &str, remote_path: Option<&str>) -> Result<()> {
    ui::print_box_start("ENCRYPT");
    ui::print_box_line(">> Encrypting message...");

    let encrypted = crypto::encrypt_data(message.as_bytes(), password, None)?;

    ui::print_box_line(&format!(">> Encrypted size: {} bytes", encrypted.len()));

    let config = Settings::load()?;

    ui::print_box_line(">> Establishing SFTP connection...");
    let client = SftpClient::connect(&config)?;

    let final_path = if let Some(custom_path) = remote_path {
        custom_path.to_string()
    } else {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        format!("{}/msg_{}.enc", config.paths.outbox, timestamp)
    };

    ui::print_box_line(">> Uploading encrypted payload...");
    client.upload(&encrypted, &final_path)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("MESSAGE TRANSMITTED");
    ui::print_info("Remote", &final_path);
    ui::print_info("Cipher", "AES-256-GCM");
    ui::print_status("SECURE");
    println!();

    Ok(())
}
