use crate::crypto;
use crate::error::{HermesError, Result};
use crate::ui;
use std::fs;
use std::path::Path;

pub fn execute(file_path: &str, key_name: &str, output: Option<&str>) -> Result<()> {
    ui::print_box_start("SIGN_FILE");

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(file_path.to_string()));
    }

    let filename = path.file_name().unwrap().to_str().unwrap();
    ui::print_box_line(&format!(">> File: {}", filename));
    ui::print_box_line(&format!(">> Signer: {}", key_name));

    let keys_dir = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
        .join(".hermes")
        .join("keys");

    let dilithium_key_path = keys_dir.join(format!("{}_dilithium.pem", key_name));
    if !dilithium_key_path.exists() {
        return Err(HermesError::ConfigError(format!(
            "Dilithium signing key not found for: {}. Generate with --sign flag",
            key_name
        )));
    }

    ui::print_box_line(">> Loading signing key...");
    let secret_key = crypto::load_dilithium_secret_key(dilithium_key_path.to_str().unwrap())?;

    ui::print_box_line(">> Reading file...");
    let file_data = fs::read(path)?;

    ui::print_box_line(">> Signing with Dilithium-5...");
    let signed_data = crypto::sign_message(&file_data, &secret_key);

    let output_path = if let Some(out) = output {
        out.to_string()
    } else {
        format!("{}.sig", file_path)
    };

    fs::write(&output_path, &signed_data)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("FILE SIGNED");
    ui::print_info("Input", file_path);
    ui::print_info("Signature", &output_path);
    ui::print_info("Algorithm", "Dilithium-5 (NIST PQC)");
    ui::print_info("Size", &format!("{} bytes", signed_data.len()));
    println!();

    Ok(())
}
