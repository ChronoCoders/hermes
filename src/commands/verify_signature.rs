use crate::crypto;
use crate::error::{HermesError, Result};
use crate::ui;
use std::fs;
use std::path::Path;

pub fn execute(signed_file: &str, signer_name: &str, output: Option<&str>) -> Result<()> {
    ui::print_box_start("VERIFY_SIGNATURE");

    let path = Path::new(signed_file);
    if !path.exists() {
        return Err(HermesError::FileNotFound(signed_file.to_string()));
    }

    ui::print_box_line(&format!(">> Signed file: {}", signed_file));
    ui::print_box_line(&format!(">> Signer: {}", signer_name));

    let recipients_dir = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
        .join(".hermes")
        .join("recipients");

    let dilithium_key_path = recipients_dir.join(format!("{}_dilithium.pub", signer_name));
    if !dilithium_key_path.exists() {
        // Try keys directory (own key)
        let keys_dir = dirs::home_dir()
            .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
            .join(".hermes")
            .join("keys");
        let own_key_path = keys_dir.join(format!("{}_dilithium.pub", signer_name));

        if !own_key_path.exists() {
            return Err(HermesError::ConfigError(format!(
                "Dilithium public key not found for: {}",
                signer_name
            )));
        }
    }

    let public_key_path = if recipients_dir
        .join(format!("{}_dilithium.pub", signer_name))
        .exists()
    {
        recipients_dir.join(format!("{}_dilithium.pub", signer_name))
    } else {
        dirs::home_dir()
            .unwrap()
            .join(".hermes")
            .join("keys")
            .join(format!("{}_dilithium.pub", signer_name))
    };

    ui::print_box_line(">> Loading verification key...");
    let public_key = crypto::load_dilithium_public_key(public_key_path.to_str().unwrap())?;

    ui::print_box_line(">> Reading signed data...");
    let signed_data = fs::read(path)?;

    ui::print_box_line(">> Verifying signature...");
    let original_data = crypto::verify_signature(&signed_data, &public_key)?;

    if let Some(out) = output {
        fs::write(out, &original_data)?;
        ui::print_box_line(&format!(">> Extracted to: {}", out));
    }

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("SIGNATURE VALID");
    ui::print_info("Signer", signer_name);
    ui::print_info("Algorithm", "Dilithium-5 (NIST PQC)");
    ui::print_info("Original size", &format!("{} bytes", original_data.len()));
    if let Some(out) = output {
        ui::print_info("Extracted to", out);
    }
    println!();

    Ok(())
}
