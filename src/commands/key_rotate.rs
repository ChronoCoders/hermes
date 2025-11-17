use crate::crypto;
use crate::error::{HermesError, Result};
use crate::ui;
use chrono::Utc;
use colored::Colorize;
use rsa::pkcs8::{EncodePrivateKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::fs;
use std::path::PathBuf;

pub fn execute(name: &str, archive: bool, generate_pqc: bool, generate_sign: bool) -> Result<()> {
    ui::print_box_start("KEY_ROTATE");

    let keys_dir = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Could not find home directory".to_string()))?
        .join(".hermes")
        .join("keys");

    if !keys_dir.exists() {
        return Err(HermesError::ConfigError(
            "Keys directory not found. Run keygen first.".to_string(),
        ));
    }

    // Check if current key exists
    let current_private = keys_dir.join(format!("{}.pem", name));
    let current_public = keys_dir.join(format!("{}.pub", name));

    if !current_private.exists() || !current_public.exists() {
        return Err(HermesError::ConfigError(format!(
            "Key '{}' not found. Generate it first with keygen.",
            name
        )));
    }

    ui::print_box_line(&format!(">> Rotating key: {}", name));

    // Archive old keys if requested
    if archive {
        let archive_dir = keys_dir.join("archive");
        if !archive_dir.exists() {
            fs::create_dir_all(&archive_dir)?;
        }

        let timestamp = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let archive_name = format!("{}_{}", name, timestamp);

        ui::print_box_line(">> Archiving old RSA keys...");
        archive_key_file(&current_private, &archive_dir, &archive_name, "pem")?;
        archive_key_file(&current_public, &archive_dir, &archive_name, "pub")?;

        // Archive PQC keys if they exist
        let kyber_private = keys_dir.join(format!("{}_kyber.pem", name));
        let kyber_public = keys_dir.join(format!("{}_kyber.pub", name));
        if kyber_private.exists() {
            ui::print_box_line(">> Archiving old Kyber keys...");
            archive_key_file(&kyber_private, &archive_dir, &archive_name, "kyber.pem")?;
            archive_key_file(&kyber_public, &archive_dir, &archive_name, "kyber.pub")?;
        }

        // Archive Dilithium keys if they exist
        let dilithium_private = keys_dir.join(format!("{}_dilithium.pem", name));
        let dilithium_public = keys_dir.join(format!("{}_dilithium.pub", name));
        if dilithium_private.exists() {
            ui::print_box_line(">> Archiving old Dilithium keys...");
            archive_key_file(&dilithium_private, &archive_dir, &archive_name, "dilithium.pem")?;
            archive_key_file(&dilithium_public, &archive_dir, &archive_name, "dilithium.pub")?;
        }

        ui::print_box_line(&format!(">> Archived to: {}/", archive_dir.display()));
    }

    // Generate new RSA keypair
    ui::print_box_line(">> Generating new RSA-4096 keypair...");
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 4096)
        .map_err(|e| HermesError::KeyGenerationFailed(e.to_string()))?;
    let public_key = RsaPublicKey::from(&private_key);

    // Save new RSA keys
    let private_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| HermesError::KeyGenerationFailed(e.to_string()))?;
    let public_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .map_err(|e| HermesError::KeyGenerationFailed(e.to_string()))?;

    fs::write(&current_private, private_pem.as_bytes())?;
    fs::write(&current_public, public_pem.as_bytes())?;

    let rsa_fingerprint = crypto::get_key_fingerprint(&public_key)?;
    ui::print_box_line(&format!(">> New RSA fingerprint: {}", rsa_fingerprint));

    // Generate new PQC keys if requested
    if generate_pqc {
        ui::print_box_line(">> Generating new Kyber-1024 keypair...");
        let (kyber_public, kyber_secret) = crypto::generate_kyber_keypair()?;

        let kyber_private_path = keys_dir.join(format!("{}_kyber.pem", name));
        let kyber_public_path = keys_dir.join(format!("{}_kyber.pub", name));

        crypto::save_kyber_secret_key(&kyber_secret, &kyber_private_path)?;
        crypto::save_kyber_public_key(&kyber_public, &kyber_public_path)?;

        let kyber_fingerprint = crypto::get_kyber_fingerprint(&kyber_public);
        ui::print_box_line(&format!(">> New Kyber fingerprint: {}", kyber_fingerprint));
    }

    // Generate new signing keys if requested
    if generate_sign {
        ui::print_box_line(">> Generating new Dilithium-5 keypair...");
        let (dilithium_public, dilithium_secret) = crypto::generate_dilithium_keypair()?;

        let dilithium_private_path = keys_dir.join(format!("{}_dilithium.pem", name));
        let dilithium_public_path = keys_dir.join(format!("{}_dilithium.pub", name));

        crypto::save_dilithium_secret_key(&dilithium_secret, &dilithium_private_path)?;
        crypto::save_dilithium_public_key(&dilithium_public, &dilithium_public_path)?;

        let dilithium_fingerprint = crypto::get_dilithium_fingerprint(&dilithium_public);
        ui::print_box_line(&format!(">> New Dilithium fingerprint: {}", dilithium_fingerprint));
    }

    // Create rotation metadata
    let metadata_path = keys_dir.join(format!("{}.rotation", name));
    let rotation_info = format!(
        "Last rotated: {}\nRSA fingerprint: {}\n",
        Utc::now().to_rfc3339(),
        rsa_fingerprint
    );
    fs::write(&metadata_path, rotation_info)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("KEY ROTATION COMPLETE");
    ui::print_info("Key name", name);
    ui::print_info("RSA fingerprint", &rsa_fingerprint);
    if archive {
        ui::print_info("Old keys archived", "Yes");
    }
    if generate_pqc {
        ui::print_info("New Kyber keys", "Generated");
    }
    if generate_sign {
        ui::print_info("New Dilithium keys", "Generated");
    }
    println!();

    println!(
        "{}",
        "WARNING: Remember to distribute your new public keys to recipients!"
            .bright_yellow()
            .bold()
    );

    Ok(())
}

fn archive_key_file(
    source: &PathBuf,
    archive_dir: &PathBuf,
    base_name: &str,
    extension: &str,
) -> Result<()> {
    let dest = archive_dir.join(format!("{}.{}", base_name, extension));
    fs::copy(source, dest)?;
    Ok(())
}
