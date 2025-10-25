use crate::config::Settings;
use crate::crypto;
use crate::error::Result;
use crate::transfer::SftpClient;
use crate::ui;
use chrono::Local;

pub fn execute(
    message: &str,
    password: &str,
    remote_path: Option<&str>,
    ttl_hours: Option<u64>,
) -> Result<()> {
    ui::print_box_start("MESSAGE_ENCRYPT");

    ui::print_box_line(&format!(">> Length: {} chars", message.len()));
    ui::print_box_line(">> Encrypting message...");

    let encrypted = crypto::encrypt_data(message.as_bytes(), password, None, ttl_hours)?;

    if let Some(hours) = ttl_hours {
        ui::print_box_line(&format!(">> Self-destruct: {} hours", hours));
    }

    ui::print_box_line(">> Uploading to SFTP vault...");

    let config = Settings::load()?;
    let client = SftpClient::connect(&config)?;

    let final_path = if let Some(custom_path) = remote_path {
        custom_path.to_string()
    } else {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        format!("{}/msg_{}.enc", config.paths.outbox, timestamp)
    };

    client.upload(&encrypted, &final_path)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("MESSAGE SECURED");
    ui::print_info("Remote", &final_path);
    ui::print_info("Size", &format!("{} bytes", encrypted.len()));
    if let Some(hours) = ttl_hours {
        ui::print_info("Expires", &format!("in {} hours", hours));
    } else {
        ui::print_info("Expires", "Never");
    }
    ui::print_status("LOCKED");
    println!();

    Ok(())
}
