use crate::error::{HermesError, Result};
use crate::ui;
use std::collections::HashMap;
use std::fs;

pub fn execute() -> Result<()> {
    ui::print_box_start("ARCHIVED_KEYS");

    let archive_dir = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
        .join(".hermes")
        .join("keys")
        .join("archive");

    if !archive_dir.exists() {
        ui::print_box_line(">> No archived keys found");
        ui::print_box_end();
        return Ok(());
    }

    let entries = fs::read_dir(&archive_dir)?;
    let mut archives: HashMap<String, Vec<String>> = HashMap::new();

    for entry in entries.flatten() {
        let filename = entry.file_name().to_string_lossy().to_string();
        if filename.ends_with(".pem") || filename.ends_with(".pub") {
            // Parse name_timestamp.extension
            if let Some(base) = filename.rsplit('.').nth(1) {
                // Find the timestamp separator (last underscore before date)
                if let Some(pos) = base.rfind('_') {
                    let parts: Vec<&str> = base.splitn(2, '_').collect();
                    if parts.len() >= 2 {
                        let key_name = parts[0];
                        let timestamp = &base[pos + 1..];
                        let archive_id = format!("{}_{}", key_name, timestamp);
                        archives
                            .entry(archive_id)
                            .or_default()
                            .push(filename);
                    }
                }
            }
        }
    }

    if archives.is_empty() {
        ui::print_box_line(">> No archived keys found");
    } else {
        let mut sorted_keys: Vec<_> = archives.keys().collect();
        sorted_keys.sort();
        sorted_keys.reverse(); // Most recent first

        for archive_id in sorted_keys {
            let files = archives.get(archive_id).unwrap();
            ui::print_box_line(&format!(">> {}", archive_id));
            for file in files {
                ui::print_box_line(&format!("   - {}", file));
            }
        }
    }

    ui::print_box_end();

    println!();
    ui::print_info("Archive location", archive_dir.to_str().unwrap_or("unknown"));
    println!();

    Ok(())
}
