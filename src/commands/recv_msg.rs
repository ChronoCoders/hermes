use crate::config::Settings;
use crate::crypto;
use crate::error::{HermesError, Result};
use crate::transfer::SftpClient;
use crate::ui;

pub fn execute(
    remote_file: &str,
    password: Option<&str>,
    recipient_name: Option<&str>,
) -> Result<()> {
    ui::print_box_start("MESSAGE_DECRYPT");

    let config = Settings::load()?;

    ui::print_box_line(">> Connecting to SFTP server...");
    let client = SftpClient::connect(&config)?;

    let remote_path = if remote_file.starts_with('/') || remote_file.contains(':') {
        remote_file.to_string()
    } else {
        format!("{}/{}", config.paths.outbox, remote_file)
    };

    ui::print_box_line(">> Downloading encrypted message...");
    let encrypted = client.download(&remote_path)?;

    let package = crate::crypto::encrypt::EncryptedPackage::from_bytes(&encrypted)?;

    if package.is_expired() {
        ui::print_box_line("");
        ui::print_box_end();
        println!();
        ui::print_error("MESSAGE EXPIRED");
        ui::print_info("Status", "Self-destructed ⚠️");
        println!();
        return Err(HermesError::DecryptionFailed);
    }

    ui::print_box_line(">> Decrypting message...");

    let decrypted = if package.is_multi_recipient() {
        if let Some(name) = recipient_name {
            ui::print_box_line(&format!(">> Using recipient key: {}", name));
            crypto::decrypt::decrypt_data_multi(&encrypted, name)?
        } else {
            ui::print_box_line("");
            ui::print_box_end();
            println!();
            ui::print_error("MULTI-RECIPIENT MESSAGE");
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
            println!("Use: hermes recv-msg {} --recipient <name>", remote_file);
            println!();
            return Err(HermesError::ConfigError(
                "Recipient name required for multi-recipient message".to_string(),
            ));
        }
    } else if let Some(pwd) = password {
        crypto::decrypt_data(&encrypted, pwd)?
    } else {
        return Err(HermesError::ConfigError(
            "Password required for password-encrypted message".to_string(),
        ));
    };

    ui::print_box_line(">> Verifying integrity...");

    let message = String::from_utf8(decrypted).map_err(|_| HermesError::DecryptionFailed)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("MESSAGE DECRYPTED");
    ui::print_info("Content", &message);
    ui::print_info("Length", &format!("{} chars", message.len()));
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
            ui::print_info("Expires", &format!("in {} hours", remaining));
        }
    }
    ui::print_status("UNLOCKED");
    println!();

    Ok(())
}
