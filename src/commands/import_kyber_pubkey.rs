use crate::crypto;
use crate::error::{HermesError, Result};
use crate::ui;
use std::fs;
use std::path::PathBuf;

pub fn execute(name: &str, pubkey_path: &str) -> Result<()> {
    ui::print_box_start("IMPORT_KYBER_KEY");

    let source_path = PathBuf::from(pubkey_path);
    if !source_path.exists() {
        return Err(HermesError::FileNotFound(pubkey_path.to_string()));
    }

    ui::print_box_line(&format!(">> Importing Kyber public key for: {}", name));
    ui::print_box_line(&format!(">> Source: {}", pubkey_path));

    // Validate the key by loading it
    let kyber_key = crypto::load_kyber_public_key(pubkey_path)?;
    let fingerprint = crypto::get_kyber_fingerprint(&kyber_key);

    let recipients_dir = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
        .join(".hermes")
        .join("recipients");

    fs::create_dir_all(&recipients_dir)?;

    let dest_path = recipients_dir.join(format!("{}_kyber.pub", name));

    fs::copy(&source_path, &dest_path)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("KYBER PUBLIC KEY IMPORTED");
    ui::print_info("Recipient", name);
    ui::print_info("Fingerprint", &fingerprint);
    ui::print_info("Saved to", dest_path.to_str().unwrap());
    println!();

    println!("You can now send PQC-encrypted files to this recipient with --pqc flag");
    println!();

    Ok(())
}
