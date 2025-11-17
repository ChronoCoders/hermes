use crate::crypto;
use crate::error::{HermesError, Result};
use crate::ui;
use std::fs;
use std::path::PathBuf;

pub fn execute(name: &str, output_path: Option<&str>) -> Result<()> {
    ui::print_box_start("EXPORT_KYBER_KEY");

    let keys_dir = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
        .join(".hermes")
        .join("keys");

    let kyber_pubkey_path = keys_dir.join(format!("{}_kyber.pub", name));

    if !kyber_pubkey_path.exists() {
        return Err(HermesError::ConfigError(format!(
            "Kyber public key not found for: {}. Generate with --pqc flag",
            name
        )));
    }

    ui::print_box_line(&format!(">> Exporting Kyber public key for: {}", name));

    let kyber_key = crypto::load_kyber_public_key(kyber_pubkey_path.to_str().unwrap())?;
    let fingerprint = crypto::get_kyber_fingerprint(&kyber_key);

    let dest_path = if let Some(path) = output_path {
        PathBuf::from(path)
    } else {
        PathBuf::from(format!("{}_kyber.pub", name))
    };

    fs::copy(&kyber_pubkey_path, &dest_path)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("KYBER PUBLIC KEY EXPORTED");
    ui::print_info("Name", name);
    ui::print_info("Fingerprint", &fingerprint);
    ui::print_info("Exported to", dest_path.to_str().unwrap());
    println!();

    println!("Share this file along with your RSA public key for hybrid encryption");
    println!();

    Ok(())
}
