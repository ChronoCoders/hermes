use crate::config::Settings;
use crate::error::{HermesError, Result};
use crate::transfer::SftpClient;
use crate::ui;
use std::path::Path;

pub fn execute(test_connection: bool) -> Result<()> {
    ui::print_box_start("CONFIG_VALIDATION");

    // Load config
    ui::print_box_line(">> Loading configuration...");
    let config = match Settings::load() {
        Ok(c) => {
            ui::print_box_line("✓ Configuration loaded successfully");
            c
        }
        Err(e) => {
            ui::print_box_line(&format!("✗ Failed to load config: {e}"));
            ui::print_box_end();
            return Err(e);
        }
    };

    // Validate SFTP settings
    ui::print_box_line(">> Validating SFTP settings...");

    if config.sftp.host.is_empty() {
        ui::print_box_line("✗ SFTP host is empty");
        ui::print_box_end();
        return Err(HermesError::ConfigError(
            "SFTP host cannot be empty".to_string(),
        ));
    }
    ui::print_box_line(&format!("✓ Host: {}", config.sftp.host));

    if config.sftp.port == 0 {
        ui::print_box_line(&format!("✗ Invalid port: {}", config.sftp.port));
        ui::print_box_end();
        return Err(HermesError::ConfigError(format!(
            "Invalid port: {}",
            config.sftp.port
        )));
    }
    ui::print_box_line(&format!("✓ Port: {}", config.sftp.port));

    if config.sftp.username.is_empty() {
        ui::print_box_line("✗ SFTP username is empty");
        ui::print_box_end();
        return Err(HermesError::ConfigError(
            "SFTP username cannot be empty".to_string(),
        ));
    }
    ui::print_box_line(&format!("✓ Username: {}", config.sftp.username));

    // Validate SSH key if provided
    if let Some(ref key_file) = config.sftp.key_file {
        if !key_file.is_empty() {
            if Path::new(key_file).exists() {
                ui::print_box_line(&format!("✓ SSH key found: {key_file}"));
            } else {
                ui::print_box_line(&format!("⚠ SSH key not found: {key_file}"));
            }
        }
    }

    // Validate paths
    ui::print_box_line(">> Validating remote paths...");

    if config.paths.inbox.is_empty() {
        ui::print_box_line("✗ Inbox path is empty");
        ui::print_box_end();
        return Err(HermesError::ConfigError(
            "Inbox path cannot be empty".to_string(),
        ));
    }
    ui::print_box_line(&format!("✓ Inbox: {}", config.paths.inbox));

    if config.paths.outbox.is_empty() {
        ui::print_box_line("✗ Outbox path is empty");
        ui::print_box_end();
        return Err(HermesError::ConfigError(
            "Outbox path cannot be empty".to_string(),
        ));
    }
    ui::print_box_line(&format!("✓ Outbox: {}", config.paths.outbox));

    if config.paths.files.is_empty() {
        ui::print_box_line("✗ Files path is empty");
        ui::print_box_end();
        return Err(HermesError::ConfigError(
            "Files path cannot be empty".to_string(),
        ));
    }
    ui::print_box_line(&format!("✓ Files: {}", config.paths.files));

    // Test connection if requested
    if test_connection {
        ui::print_box_line(">> Testing SFTP connection...");

        match SftpClient::connect(&config) {
            Ok(_) => {
                ui::print_box_line("✓ Connection successful");
            }
            Err(e) => {
                ui::print_box_line(&format!("✗ Connection failed: {e}"));
                ui::print_box_end();
                return Err(e);
            }
        }
    }

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("CONFIGURATION VALID");
    ui::print_info(
        "Host",
        &format!("{}:{}", config.sftp.host, config.sftp.port),
    );
    ui::print_info("Username", &config.sftp.username);
    ui::print_info("Inbox", &config.paths.inbox);
    ui::print_info("Outbox", &config.paths.outbox);
    ui::print_info("Files", &config.paths.files);

    if test_connection {
        ui::print_status("CONNECTION OK");
    } else {
        ui::print_status("VALIDATED");
    }
    println!();

    Ok(())
}
