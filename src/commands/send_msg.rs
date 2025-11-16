use crate::config::Settings;
use crate::crypto;
use crate::error::Result;
use crate::transfer::SftpClient;
use crate::ui;
use chrono::Local;

pub fn execute(
    message: &str,
    password: Option<&str>,
    remote_path: Option<&str>,
    ttl_hours: Option<u64>,
    recipients: Option<Vec<String>>,
) -> Result<()> {
    ui::print_box_start("MESSAGE_ENCRYPT");

    ui::print_box_line(&format!(">> Length: {} chars", message.len()));

    let encrypted = if let Some(recips) = recipients {
        ui::print_box_line(&format!(">> Recipients: {}", recips.join(", ")));
        ui::print_box_line(">> Encrypting with RSA hybrid encryption...");
        crypto::encrypt::encrypt_data_multi(
            message.as_bytes(),
            None,
            None,
            ttl_hours,
            Some(recips),
        )?
    } else if let Some(pwd) = password {
        ui::print_box_line(">> Encrypting message...");
        crypto::encrypt_data(message.as_bytes(), pwd, None, ttl_hours)?
    } else {
        return Err(crate::error::HermesError::ConfigError(
            "Either password or recipients required".to_string(),
        ));
    };

    if let Some(hours) = ttl_hours {
        ui::print_box_line(&format!(">> Self-destruct: {hours} hours"));
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
        ui::print_info("Expires", &format!("in {hours} hours"));
    } else {
        ui::print_info("Expires", "Never");
    }
    ui::print_status("LOCKED");
    println!();

    Ok(())
}
