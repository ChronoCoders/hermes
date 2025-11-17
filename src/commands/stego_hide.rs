use crate::crypto::encrypt::{encrypt_data, encrypt_data_multi};
use crate::error::{HermesError, Result};
use crate::steganography;
use crate::ui;
use std::fs;
use std::path::Path;

pub fn execute(
    file_path: &str,
    cover_image: &str,
    output_image: &str,
    password: Option<&str>,
    recipients: Option<Vec<String>>,
) -> Result<()> {
    ui::print_box_start("STEGO_HIDE");

    let path = Path::new(file_path);
    if !path.exists() {
        return Err(HermesError::FileNotFound(file_path.to_string()));
    }

    let cover_path = Path::new(cover_image);
    if !cover_path.exists() {
        return Err(HermesError::FileNotFound(cover_image.to_string()));
    }

    let filename = path.file_name().unwrap().to_str().unwrap();
    ui::print_box_line(&format!(">> File: {}", filename));
    ui::print_box_line(&format!(">> Cover image: {}", cover_image));
    ui::print_box_line(&format!(">> Output: {}", output_image));

    // Check capacity
    let capacity = steganography::get_capacity(cover_image)?;
    let file_size = fs::metadata(path)?.len() as usize;
    ui::print_box_line(&format!(">> Image capacity: {} bytes", capacity));

    // Read and compress file
    ui::print_box_line(">> Reading file...");
    let file_data = fs::read(path)?;

    // Encrypt the data
    ui::print_box_line(">> Encrypting data...");
    let encrypted = if let Some(pwd) = password {
        encrypt_data(&file_data, pwd, None, None)?
    } else if let Some(recips) = recipients {
        encrypt_data_multi(&file_data, None, None, None, Some(recips), false)?
    } else {
        return Err(HermesError::ConfigError(
            "Either password or recipients must be specified".to_string(),
        ));
    };

    if encrypted.len() > capacity {
        return Err(HermesError::SteganographyError(format!(
            "Encrypted data ({} bytes) exceeds image capacity ({} bytes). Use a larger image.",
            encrypted.len(),
            capacity
        )));
    }

    ui::print_box_line(&format!(">> Encrypted size: {} bytes", encrypted.len()));

    // Embed in image
    ui::print_box_line(">> Embedding data in image...");
    steganography::embed_data(cover_image, &encrypted, output_image)?;

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("DATA HIDDEN IN IMAGE");
    ui::print_info("Original file", file_path);
    ui::print_info("Cover image", cover_image);
    ui::print_info("Stego image", output_image);
    ui::print_info("Original size", &format!("{} bytes", file_size));
    ui::print_info("Encrypted size", &format!("{} bytes", encrypted.len()));
    ui::print_info(
        "Capacity used",
        &format!("{:.1}%", (encrypted.len() as f64 / capacity as f64) * 100.0),
    );
    println!();

    Ok(())
}
