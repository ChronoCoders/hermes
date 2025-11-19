use crate::crypto;
use crate::error::Result;
use crate::progress;
use crate::ui;
use std::path::PathBuf;

pub fn execute(name: &str, output_dir: Option<&str>, use_pqc: bool, use_sign: bool) -> Result<()> {
    let title = if use_pqc && use_sign {
        "FULL_PQC_KEYGEN"
    } else if use_pqc {
        "HYBRID_KEYGEN"
    } else if use_sign {
        "SIGNING_KEYGEN"
    } else {
        "RSA_KEYGEN"
    };
    ui::print_box_start(title);

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

    spinner.finish_with_message("RSA keypair generated".to_string());

    let public_key = crypto::load_public_key(public_key_path.to_str().unwrap())?;
    let fingerprint = crypto::get_key_fingerprint(&public_key)?;

    // Generate Kyber (Post-Quantum) keypair if PQC is enabled
    let (kyber_private_path, kyber_public_path, kyber_fingerprint) = if use_pqc {
        let kyber_private_path = key_dir.join(format!("{name}_kyber.pem"));
        let kyber_public_path = key_dir.join(format!("{name}_kyber.pub"));

        ui::print_box_line("");
        ui::print_box_line(">> Generating Kyber-1024 keypair (Post-Quantum)...");

        let pq_spinner = progress::create_keygen_spinner();
        pq_spinner.set_message("Generating Kyber lattice...".to_string());

        let (kyber_public, kyber_secret) = crypto::generate_kyber_keypair()?;

        crypto::save_kyber_secret_key(&kyber_secret, &kyber_private_path)?;
        crypto::save_kyber_public_key(&kyber_public, &kyber_public_path)?;

        let kyber_fp = crypto::get_kyber_fingerprint(&kyber_public);

        pq_spinner.finish_with_message("Kyber keypair generated".to_string());

        (
            Some(kyber_private_path),
            Some(kyber_public_path),
            Some(kyber_fp),
        )
    } else {
        (None, None, None)
    };

    // Generate Dilithium (Post-Quantum Signatures) keypair if signing is enabled
    let (dilithium_private_path, dilithium_public_path, dilithium_fingerprint) = if use_sign {
        let dilithium_private_path = key_dir.join(format!("{name}_dilithium.pem"));
        let dilithium_public_path = key_dir.join(format!("{name}_dilithium.pub"));

        ui::print_box_line("");
        ui::print_box_line(">> Generating Dilithium-5 keypair (Signatures)...");

        let sign_spinner = progress::create_keygen_spinner();
        sign_spinner.set_message("Generating Dilithium lattice...".to_string());

        let (dilithium_public, dilithium_secret) = crypto::generate_dilithium_keypair()?;

        crypto::save_dilithium_secret_key(&dilithium_secret, &dilithium_private_path)?;
        crypto::save_dilithium_public_key(&dilithium_public, &dilithium_public_path)?;

        let dilithium_fp = crypto::get_dilithium_fingerprint(&dilithium_public);

        sign_spinner.finish_with_message("Dilithium keypair generated".to_string());

        (
            Some(dilithium_private_path),
            Some(dilithium_public_path),
            Some(dilithium_fp),
        )
    } else {
        (None, None, None)
    };

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    if use_pqc && use_sign {
        ui::print_success("FULL PQC KEYPAIR GENERATED (RSA + Kyber + Dilithium)");
    } else if use_pqc {
        ui::print_success("HYBRID KEYPAIR GENERATED (RSA + Kyber)");
    } else if use_sign {
        ui::print_success("SIGNING KEYPAIR GENERATED (RSA + Dilithium)");
    } else {
        ui::print_success("RSA KEYPAIR GENERATED");
    }
    ui::print_info("Name", name);
    ui::print_info("RSA Private Key", private_key_path.to_str().unwrap());
    ui::print_info("RSA Public Key", public_key_path.to_str().unwrap());
    ui::print_info("RSA Fingerprint", &fingerprint);
    ui::print_info("RSA Key Size", "4096 bits");

    if use_pqc {
        println!();
        ui::print_info(
            "Kyber Private Key",
            kyber_private_path.unwrap().to_str().unwrap(),
        );
        ui::print_info(
            "Kyber Public Key",
            kyber_public_path.unwrap().to_str().unwrap(),
        );
        ui::print_info("Kyber Fingerprint", &kyber_fingerprint.unwrap());
        ui::print_info("Kyber Security", "ML-KEM 1024 (256-bit equiv)");
    }

    if use_sign {
        println!();
        ui::print_info(
            "Dilithium Private Key",
            dilithium_private_path.unwrap().to_str().unwrap(),
        );
        ui::print_info(
            "Dilithium Public Key",
            dilithium_public_path.unwrap().to_str().unwrap(),
        );
        ui::print_info("Dilithium Fingerprint", &dilithium_fingerprint.unwrap());
        ui::print_info("Dilithium Security", "NIST Level 5 (256-bit equiv)");
    }
    println!();

    println!("Keep your private key(s) secure!");
    if use_pqc && use_sign {
        println!("Share ALL public keys (.pub, _kyber.pub, _dilithium.pub) for full PQC support");
    } else if use_pqc {
        println!("Share BOTH public keys (.pub and _kyber.pub) for quantum-safe encryption");
    } else if use_sign {
        println!("Share BOTH public keys (.pub and _dilithium.pub) for digital signatures");
    } else {
        println!("Share your public key with others to receive encrypted files");
    }
    println!();

    Ok(())
}
