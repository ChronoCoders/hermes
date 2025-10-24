use crate::config::Settings;
use crate::error::Result;
use crate::transfer::SftpClient;
use crate::ui;
use colored::*;

pub fn execute() -> Result<()> {
    ui::print_box_start("LIST");

    let config = Settings::load()?;

    ui::print_box_line(">> Connecting to SFTP server...");
    let client = SftpClient::connect(&config)?;

    ui::print_box_line(">> Scanning vault directories...");

    let sftp = client.session.sftp().map_err(|e| {
        crate::error::HermesError::SftpOperationFailed(format!("SFTP init failed: {}", e))
    })?;

    println!();
    ui::print_box_end();

    println!();
    println!(
        "{}",
        "╔══════════════════════════════════════════════════════════╗".cyan()
    );
    println!(
        "{}",
        "║                    ENCRYPTED VAULT                       ║".cyan()
    );
    println!(
        "{}",
        "╚══════════════════════════════════════════════════════════╝".cyan()
    );
    println!();

    let directories = vec![
        ("OUTBOX", &config.paths.outbox),
        ("INBOX", &config.paths.inbox),
        ("FILES", &config.paths.files),
    ];

    for (name, path) in directories {
        println!("{}", format!("📁 {}", name).bright_cyan().bold());
        println!("{}", format!("   Path: {}", path).bright_black());

        match sftp.readdir(std::path::Path::new(path)) {
            Ok(entries) => {
                if entries.is_empty() {
                    println!("{}", "   (empty)".bright_black());
                } else {
                    for (file_path, stat) in entries {
                        if let Some(filename) = file_path.file_name() {
                            let size = stat.size.unwrap_or(0);
                            let size_mb = size as f64 / 1024.0 / 1024.0;

                            if size_mb > 1.0 {
                                println!(
                                    "   {} {} {}",
                                    "•".bright_green(),
                                    filename.to_string_lossy().bright_magenta(),
                                    format!("({:.2} MB)", size_mb).bright_yellow()
                                );
                            } else {
                                let size_kb = size as f64 / 1024.0;
                                println!(
                                    "   {} {} {}",
                                    "•".bright_green(),
                                    filename.to_string_lossy().bright_magenta(),
                                    format!("({:.2} KB)", size_kb).bright_yellow()
                                );
                            }
                        }
                    }
                }
            }
            Err(_) => {
                println!("{}", "   ⚠ Directory not accessible".bright_red());
            }
        }
        println!();
    }

    Ok(())
}
