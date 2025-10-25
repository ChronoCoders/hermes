use crate::crypto;
use crate::error::Result;
use crate::ui;
use std::fs;
use std::path::PathBuf;

pub fn execute(name: &str, output_path: Option<&str>) -> Result<()> {
    ui::print_box_start("EXPORT_PUBKEY");

    let key_dir = dirs::home_dir()
        .ok_or_else(|| {
            crate::error::HermesError::ConfigError("Could not find home directory".to_string())
        })?
        .join(".hermes")
        .join("keys");

    let public_key_path = key_dir.join(format!("{}.pub", name));

    if !public_key_path.exists() {
        return Err(crate::error::HermesError::FileNotFound(format!(
            "Public key not found: {}",
            name
        )));
    }

    ui::print_box_line(&format!(">> Exporting public key: {}", name));

    let public_key = crypto::load_public_key(public_key_path.to_str().unwrap())?;
    let fingerprint = crypto::get_key_fingerprint(&public_key)?;

    let dest = if let Some(path) = output_path {
        PathBuf::from(path)
    } else {
        PathBuf::from(format!("{}_public.pem", name))
    };

    fs::copy(&public_key_path, &dest)?;

    ui::print_box_line(">> Public key exported");
    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("PUBLIC KEY EXPORTED");
    ui::print_info("Output", dest.to_str().unwrap());
    ui::print_info("Fingerprint", &fingerprint);
    println!();

    println!("📤 Share this file with others so they can encrypt files for you");
    println!();

    Ok(())
}
