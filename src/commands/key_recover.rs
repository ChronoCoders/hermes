use crate::error::{HermesError, Result};
use crate::shamir::{recover_secret, Share};
use crate::ui;
use rsa::pkcs8::{DecodePrivateKey, EncodePrivateKey, LineEnding};
use rsa::RsaPrivateKey;
use std::fs;

pub fn execute(share_paths: Vec<String>, output_name: &str) -> Result<()> {
    ui::print_box_start("KEY_RECOVER");
    ui::print_box_line(&format!(">> Loading {} shares...", share_paths.len()));
    ui::print_box_line("");

    let mut shares = Vec::new();

    for (idx, path) in share_paths.iter().enumerate() {
        let json = fs::read_to_string(path).map_err(|e| {
            HermesError::FileNotFound(format!("Failed to read share {}: {}", path, e))
        })?;

        let share = Share::from_json(&json)?;

        if !share.verify() {
            return Err(HermesError::ConfigError(format!(
                "Share {} verification failed",
                idx + 1
            )));
        }

        ui::print_box_line(&format!("   Share {}: OK", share.id));
        shares.push(share);
    }

    ui::print_box_line("");
    ui::print_box_line(">> Recovering private key...");

    let recovered_bytes = recover_secret(&shares)?;

    let private_key = RsaPrivateKey::from_pkcs8_der(&recovered_bytes).map_err(|_e| {
        HermesError::DecryptionFailed
    })?;

    ui::print_box_line(">> Saving recovered key...");

    // Save the recovered private key
    let private_pem = private_key
        .to_pkcs8_pem(LineEnding::LF)
        .map_err(|_e| HermesError::DecryptionFailed)?;

    let key_path = format!("{}.pem", output_name);
    fs::write(&key_path, private_pem.as_bytes())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&key_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&key_path, perms)?;
    }

    ui::print_box_end();

    println!();
    ui::print_success("KEY RECOVERY COMPLETE");
    ui::print_info("Shares Used", &shares.len().to_string());
    ui::print_info("Key Name", output_name);
    ui::print_info("Threshold", &format!("{}/{}", shares[0].threshold, shares[0].total_shares));
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
