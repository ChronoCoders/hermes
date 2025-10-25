use crate::config::Settings;
use crate::error::Result;
use crate::ui;

pub fn execute() -> Result<()> {
    let config = Settings::load()?;

    println!();
    ui::print_box_start("CONFIGURATION");

    ui::print_box_line("SFTP Settings:");
    ui::print_box_line(&format!("  Host: {}", config.sftp.host));
    ui::print_box_line(&format!("  Port: {}", config.sftp.port));
    ui::print_box_line(&format!("  Username: {}", config.sftp.username));
    ui::print_box_line(&format!(
        "  Key File: {}",
        config.sftp.key_file.unwrap_or_else(|| "None".to_string())
    ));
    ui::print_box_line("");

    ui::print_box_line("Vault Paths:");
    ui::print_box_line(&format!("  Inbox: {}", config.paths.inbox));
    ui::print_box_line(&format!("  Outbox: {}", config.paths.outbox));
    ui::print_box_line(&format!("  Files: {}", config.paths.files));
    ui::print_box_line("");

    ui::print_box_end();
    println!();

    Ok(())
}
