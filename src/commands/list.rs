use crate::config::Settings;
use crate::error::Result;
use colored::Colorize;
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
            .filter_map(std::result::Result::ok)
            .filter(|e| e.path().is_file())
            .collect();

        if files.is_empty() {
            println!("   {}", "(empty)".bright_black());
        } else {
            files.sort_by_key(std::fs::DirEntry::path);

            for entry in files {
                let file_path = entry.path();
                let file_name = match file_path.file_name().and_then(|n| n.to_str()) {
                    Some(name) => name,
                    None => continue, // Skip files with invalid names
                };

                if let Ok(metadata) = entry.metadata() {
                    let size_kb = metadata.len() as f64 / 1024.0;

                    let mut status_info = format!("{size_kb:.2} KB");

                    if file_name.ends_with(".enc") {
                        if let Ok(file_data) = fs::read(&file_path) {
                            if let Ok(package) =
                                crate::crypto::encrypt::EncryptedPackage::from_bytes(&file_data)
                            {
                                if package.is_expired() {
                                    status_info.push_str(&format!(" {}", "[EXPIRED]".red().bold()));
                                } else if package.expires_at > 0 {
                                    if let Ok(duration) = std::time::SystemTime::now()
                                        .duration_since(std::time::UNIX_EPOCH)
                                    {
                                        let now = duration.as_secs();
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
