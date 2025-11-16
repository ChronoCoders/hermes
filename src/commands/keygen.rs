use crate::crypto;
use crate::error::Result;
use crate::progress;
use crate::ui;
use std::path::PathBuf;

pub fn execute(name: &str, output_dir: Option<&str>) -> Result<()> {
    ui::print_box_start("RSA_KEYGEN");

    let key_dir = if let Some(dir) = output_dir {
        PathBuf::from(dir)
    } else {
        dirs::home_dir()
            .ok_or_else(|| {
                crate::error::HermesError::ConfigError("Could not find home directory".to_string())
            })?
            .join(".hermes")
            .join("keys")
    };

    std::fs::create_dir_all(&key_dir)?;

    let private_key_path = key_dir.join(format!("{name}.pem"));
    let public_key_path = key_dir.join(format!("{name}.pub"));

    ui::print_box_line(&format!(">> Generating RSA-4096 keypair for: {name}"));
    ui::print_box_line(">> This may take a moment...");

    let spinner = progress::create_keygen_spinner();
    spinner.set_message("Generating prime numbers...".to_string());

    crypto::generate_keypair(
        private_key_path.to_str().unwrap(),
        public_key_path.to_str().unwrap(),
    )?;

    spinner.finish_with_message("✓ Keypair generated".to_string());

    let public_key = crypto::load_public_key(public_key_path.to_str().unwrap())?;
    let fingerprint = crypto::get_key_fingerprint(&public_key)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("RSA KEYPAIR GENERATED");
    ui::print_info("Name", name);
    ui::print_info("Private Key", private_key_path.to_str().unwrap());
    ui::print_info("Public Key", public_key_path.to_str().unwrap());
    ui::print_info("Fingerprint", &fingerprint);
    ui::print_info("Key Size", "4096 bits");
    println!();

    println!("⚠️  Keep your private key secure!");
    println!("📤 Share your public key with others to receive encrypted files");
    println!();

    Ok(())
}
