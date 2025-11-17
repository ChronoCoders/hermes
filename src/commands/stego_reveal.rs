use crate::crypto::decrypt::{decrypt_data, decrypt_data_multi};
use crate::error::{HermesError, Result};
use crate::steganography;
use crate::ui;
use std::fs;

pub fn execute(
    stego_image: &str,
    output_path: &str,
    password: Option<&str>,
    recipient: Option<&str>,
) -> Result<()> {
    ui::print_box_start("STEGO_REVEAL");

    if !std::path::Path::new(stego_image).exists() {
        return Err(HermesError::FileNotFound(stego_image.to_string()));
    }

    ui::print_box_line(&format!(">> Stego image: {}", stego_image));
    ui::print_box_line(&format!(">> Output: {}", output_path));

    // Extract hidden data
    ui::print_box_line(">> Extracting hidden data...");
    let encrypted_data = steganography::extract_data(stego_image)?;

    ui::print_box_line(&format!(">> Extracted: {} bytes", encrypted_data.len()));

    // Decrypt the data
    ui::print_box_line(">> Decrypting data...");
    let decrypted = if let Some(pwd) = password {
        decrypt_data(&encrypted_data, pwd)?
    } else if let Some(recip) = recipient {
        decrypt_data_multi(&encrypted_data, recip)?
    } else {
        return Err(HermesError::ConfigError(
            "Either password or recipient must be specified".to_string(),
        ));
    };

    ui::print_box_line(&format!(">> Decrypted size: {} bytes", decrypted.len()));

    // Write output file
    ui::print_box_line(">> Writing output file...");
    fs::write(output_path, &decrypted)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("DATA REVEALED FROM IMAGE");
    ui::print_info("Stego image", stego_image);
    ui::print_info("Output file", output_path);
    ui::print_info("Encrypted size", &format!("{} bytes", encrypted_data.len()));
    ui::print_info("Decrypted size", &format!("{} bytes", decrypted.len()));
    println!();

    Ok(())
}
