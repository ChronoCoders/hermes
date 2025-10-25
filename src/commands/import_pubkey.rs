use crate::crypto;
use crate::error::Result;
use crate::ui;
use std::fs;

pub fn execute(name: &str, pubkey_path: &str) -> Result<()> {
    ui::print_box_start("IMPORT_PUBKEY");

    let recipients_dir = dirs::home_dir()
        .ok_or_else(|| {
            crate::error::HermesError::ConfigError("Could not find home directory".to_string())
        })?
        .join(".hermes")
        .join("recipients");

    std::fs::create_dir_all(&recipients_dir)?;

    ui::print_box_line(&format!(">> Importing public key for: {}", name));

    let public_key = crypto::load_public_key(pubkey_path)?;
    let fingerprint = crypto::get_key_fingerprint(&public_key)?;

    let dest_path = recipients_dir.join(format!("{}.pub", name));
    fs::copy(pubkey_path, &dest_path)?;

    ui::print_box_line(">> Public key imported successfully");
    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("PUBLIC KEY IMPORTED");
    ui::print_info("Recipient", name);
    ui::print_info("Stored At", dest_path.to_str().unwrap());
    ui::print_info("Fingerprint", &fingerprint);
    println!();

    println!(
        "✅ You can now encrypt files for {} using --recipients flag",
        name
    );
    println!();

    Ok(())
}
