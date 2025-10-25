use crate::config::Settings;
use crate::error::Result;
use colored::*;
use std::fs;

pub fn execute() -> Result<()> {
    let config = Settings::load()?;

    println!("\n{}", "═".repeat(60).bright_cyan());
    println!("{}", "📁 HERMES VAULT INVENTORY".bright_white().bold());
    println!("{}", "═".repeat(60).bright_cyan());

    print_directory(&config.paths.outbox, "OUTBOX")?;
    print_directory(&config.paths.inbox, "INBOX")?;
    print_directory(&config.paths.files, "FILES")?;

    println!("{}\n", "═".repeat(60).bright_cyan());

    Ok(())
}

fn print_directory(path: &str, label: &str) -> Result<()> {
    println!("\n📁 {}", label.bright_yellow().bold());
    println!("   Path: {}", path.bright_black());

    if let Ok(entries) = fs::read_dir(path) {
        let mut files: Vec<_> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .collect();

        if files.is_empty() {
            println!("   {}", "(empty)".bright_black());
        } else {
            files.sort_by_key(|e| e.path());

            for entry in files {
                let file_path = entry.path();
                let file_name = file_path.file_name().unwrap().to_str().unwrap();

                if let Ok(metadata) = entry.metadata() {
                    let size_kb = metadata.len() as f64 / 1024.0;

                    let mut status_info = format!("{:.2} KB", size_kb);

                    if file_name.ends_with(".enc") {
                        if let Ok(file_data) = fs::read(&file_path) {
                            if let Ok(package) =
                                crate::crypto::encrypt::EncryptedPackage::from_bytes(&file_data)
                            {
                                if package.is_expired() {
                                    status_info.push_str(&format!(" {}", "[EXPIRED]".red().bold()));
                                } else if package.expires_at > 0 {
                                    let now = std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs();
                                    let remaining = (package.expires_at as i64 - now as i64) / 3600;
                                    if remaining > 0 {
                                        status_info.push_str(&format!(
                                            " {}{}h",
                                            "[⏰ ".yellow(),
                                            remaining
                                        ));
                                        status_info.push_str(&"]".yellow().to_string());
                                    }
                                }
                            }
                        }
                    }

                    println!("   • {} ({})", file_name.bright_green(), status_info);
                }
            }
        }
    } else {
        println!("   {}", "(directory not accessible)".bright_red());
    }

    Ok(())
}
