use crate::crypto;
use crate::error::Result;
use colored::*;
use std::fs;

pub fn execute() -> Result<()> {
    let key_dir = dirs::home_dir()
        .ok_or_else(|| {
            crate::error::HermesError::ConfigError("Could not find home directory".to_string())
        })?
        .join(".hermes")
        .join("keys");

    let recipients_dir = dirs::home_dir()
        .ok_or_else(|| {
            crate::error::HermesError::ConfigError("Could not find home directory".to_string())
        })?
        .join(".hermes")
        .join("recipients");

    println!("\n{}", "═".repeat(60).bright_cyan());
    println!("{}", "🔑 RSA KEY MANAGEMENT".bright_white().bold());
    println!("{}", "═".repeat(60).bright_cyan());

    println!("\n🔐 {}", "Your Keys".bright_yellow().bold());
    println!("   Path: {}", key_dir.display().to_string().bright_black());

    if key_dir.exists() {
        if let Ok(entries) = fs::read_dir(&key_dir) {
            let mut found_keys = false;

            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pub") {
                    found_keys = true;
                    let name = path.file_stem().unwrap().to_str().unwrap();

                    if let Ok(public_key) = crypto::load_public_key(path.to_str().unwrap()) {
                        if let Ok(fingerprint) = crypto::get_key_fingerprint(&public_key) {
                            println!(
                                "   • {} ({})",
                                name.bright_green(),
                                fingerprint.bright_black()
                            );
                        }
                    }
                }
            }

            if !found_keys {
                println!("   {}", "(no keys found)".bright_black());
                println!(
                    "   Use: {} to generate a keypair",
                    "hermes keygen <name>".bright_cyan()
                );
            }
        }
    } else {
        println!("   {}", "(no keys found)".bright_black());
        println!(
            "   Use: {} to generate a keypair",
            "hermes keygen <name>".bright_cyan()
        );
    }

    println!("\n📤 {}", "Recipients".bright_yellow().bold());
    println!(
        "   Path: {}",
        recipients_dir.display().to_string().bright_black()
    );

    if recipients_dir.exists() {
        if let Ok(entries) = fs::read_dir(&recipients_dir) {
            let mut found_recipients = false;

            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("pub") {
                    found_recipients = true;
                    let name = path.file_stem().unwrap().to_str().unwrap();

                    if let Ok(public_key) = crypto::load_public_key(path.to_str().unwrap()) {
                        if let Ok(fingerprint) = crypto::get_key_fingerprint(&public_key) {
                            println!(
                                "   • {} ({})",
                                name.bright_green(),
                                fingerprint.bright_black()
                            );
                        }
                    }
                }
            }

            if !found_recipients {
                println!("   {}", "(no recipients found)".bright_black());
                println!(
                    "   Use: {} to add a recipient",
                    "hermes import-pubkey <name> <file>".bright_cyan()
                );
            }
        }
    } else {
        println!("   {}", "(no recipients found)".bright_black());
        println!(
            "   Use: {} to add a recipient",
            "hermes import-pubkey <name> <file>".bright_cyan()
        );
    }

    println!("{}\n", "═".repeat(60).bright_cyan());

    Ok(())
}
