//! Steganography module for hiding encrypted data in images
//!
//! Uses Least Significant Bit (LSB) steganography to embed data in PNG images.
//! The technique modifies the least significant bits of pixel color channels,
//! which is imperceptible to the human eye.

use crate::error::{HermesError, Result};
use image::{GenericImageView, ImageBuffer, RgbaImage};

/// Magic bytes to identify steganographic data
const STEGO_MAGIC: &[u8] = b"HRMSSTEG";

/// Embed encrypted data into a PNG image using LSB steganography
///
/// # Arguments
/// * `cover_image_path` - Path to the cover image (PNG)
/// * `data` - Encrypted data to hide
/// * `output_path` - Path for the output stego-image
///
/// # Returns
/// * `Result<()>` - Success or error
pub fn embed_data(cover_image_path: &str, data: &[u8], output_path: &str) -> Result<()> {
    let img = image::open(cover_image_path)
        .map_err(|e| HermesError::SteganographyError(format!("Failed to open image: {}", e)))?;

    let (width, height) = img.dimensions();
    let rgba_img = img.to_rgba8();

    // Calculate maximum capacity (3 bits per pixel for RGB, leaving alpha unchanged)
    let max_capacity = (width * height * 3 / 8) as usize;

    // Prepare data with magic header and length prefix
    let mut payload = Vec::new();
    payload.extend_from_slice(STEGO_MAGIC);
    payload.extend_from_slice(&(data.len() as u32).to_be_bytes());
    payload.extend_from_slice(data);

    if payload.len() > max_capacity {
        return Err(HermesError::SteganographyError(format!(
            "Data too large for image. Max capacity: {} bytes, data size: {} bytes",
            max_capacity - 12, // Subtract header and length
            data.len()
        )));
    }

    let mut output_img: RgbaImage = ImageBuffer::new(width, height);
    let mut bit_index = 0;
    let total_bits = payload.len() * 8;

    for (x, y, pixel) in rgba_img.enumerate_pixels() {
        let mut new_pixel = *pixel;

        // Modify R, G, B channels (leave Alpha unchanged)
        for channel in 0..3 {
            if bit_index < total_bits {
                let byte_index = bit_index / 8;
                let bit_offset = 7 - (bit_index % 8);
                let bit = (payload[byte_index] >> bit_offset) & 1;

                // Clear LSB and set to our bit
                new_pixel[channel] = (new_pixel[channel] & 0xFE) | bit;
                bit_index += 1;
            }
        }

        output_img.put_pixel(x, y, new_pixel);
    }

    output_img
        .save(output_path)
        .map_err(|e| HermesError::SteganographyError(format!("Failed to save image: {}", e)))?;

    Ok(())
}

/// Extract hidden data from a steganographic image
///
/// # Arguments
/// * `stego_image_path` - Path to the stego-image
///
/// # Returns
/// * `Result<Vec<u8>>` - Extracted encrypted data
pub fn extract_data(stego_image_path: &str) -> Result<Vec<u8>> {
    let img = image::open(stego_image_path)
        .map_err(|e| HermesError::SteganographyError(format!("Failed to open image: {}", e)))?;

    let rgba_img = img.to_rgba8();
    let (width, height) = img.dimensions();

    // Extract all LSBs first
    let mut bits = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let pixel = rgba_img.get_pixel(x, y);
            for channel in 0..3 {
                bits.push(pixel[channel] & 1);
            }
        }
    }

    // Convert bits to bytes
    let mut bytes = Vec::new();
    for chunk in bits.chunks(8) {
        if chunk.len() == 8 {
            let mut byte = 0u8;
            for (i, &bit) in chunk.iter().enumerate() {
                byte |= bit << (7 - i);
            }
            bytes.push(byte);
        }
    }

    // Verify magic header
    if bytes.len() < 12 {
        return Err(HermesError::SteganographyError(
            "Image does not contain steganographic data".to_string(),
        ));
    }

    if &bytes[0..8] != STEGO_MAGIC {
        return Err(HermesError::SteganographyError(
            "Invalid steganographic data (magic mismatch)".to_string(),
        ));
    }

    // Extract length
    let length = u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]) as usize;

    if bytes.len() < 12 + length {
        return Err(HermesError::SteganographyError(
            "Incomplete steganographic data".to_string(),
        ));
    }

    // Extract data
    let data = bytes[12..12 + length].to_vec();

    Ok(data)
}

/// Calculate maximum data capacity for an image
pub fn get_capacity(image_path: &str) -> Result<usize> {
    let img = image::open(image_path)
        .map_err(|e| HermesError::SteganographyError(format!("Failed to open image: {}", e)))?;

    let (width, height) = img.dimensions();
    // 3 bits per pixel (RGB), divided by 8 to get bytes
    // Subtract 12 bytes for magic header and length prefix
    let capacity = ((width * height * 3 / 8) as usize).saturating_sub(12);

    Ok(capacity)
}

/// Analyze an image for potential steganographic content
pub fn analyze_image(image_path: &str) -> Result<StegoAnalysis> {
    let img = image::open(image_path)
        .map_err(|e| HermesError::SteganographyError(format!("Failed to open image: {}", e)))?;

    let (width, height) = img.dimensions();
    let rgba_img = img.to_rgba8();

    // Count LSB distribution
    let mut lsb_zeros = 0u64;
    let mut lsb_ones = 0u64;

    for pixel in rgba_img.pixels() {
        for channel in 0..3 {
            if pixel[channel] & 1 == 0 {
                lsb_zeros += 1;
            } else {
                lsb_ones += 1;
            }
        }
    }

    let total = lsb_zeros + lsb_ones;
    let ratio = lsb_ones as f64 / total as f64;

    // Check for magic header
    let has_data = extract_data(image_path).is_ok();

    Ok(StegoAnalysis {
        width,
        height,
        capacity: get_capacity(image_path)?,
        lsb_ratio: ratio,
        likely_contains_data: has_data,
    })
}

/// Analysis results for steganographic inspection
#[derive(Debug)]
pub struct StegoAnalysis {
    pub width: u32,
    pub height: u32,
    pub capacity: usize,
    pub lsb_ratio: f64,
    pub likely_contains_data: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;
    use tempfile::tempdir;

    #[test]
    fn test_embed_extract_roundtrip() {
        let dir = tempdir().unwrap();

        // Create a simple test image
        let cover_path = dir.path().join("cover.png");
        let stego_path = dir.path().join("stego.png");

        let img: RgbaImage = ImageBuffer::from_fn(100, 100, |_, _| Rgba([128, 128, 128, 255]));
        img.save(&cover_path).unwrap();

        // Test data
        let test_data = b"Secret encrypted message for steganography test!";

        // Embed
        embed_data(
            cover_path.to_str().unwrap(),
            test_data,
            stego_path.to_str().unwrap(),
        )
        .unwrap();

        // Extract
        let extracted = extract_data(stego_path.to_str().unwrap()).unwrap();

        assert_eq!(test_data.to_vec(), extracted);
    }

    #[test]
    fn test_capacity_calculation() {
        let dir = tempdir().unwrap();
        let img_path = dir.path().join("test.png");

        // 100x100 image = 10000 pixels = 30000 bits = 3750 bytes - 12 header = 3738 bytes
        let img: RgbaImage = ImageBuffer::from_fn(100, 100, |_, _| Rgba([128, 128, 128, 255]));
        img.save(&img_path).unwrap();

        let capacity = get_capacity(img_path.to_str().unwrap()).unwrap();
        assert_eq!(capacity, 3738);
    }

    #[test]
    fn test_data_too_large() {
        let dir = tempdir().unwrap();
        let cover_path = dir.path().join("small.png");
        let stego_path = dir.path().join("stego.png");

        // Very small image
        let img: RgbaImage = ImageBuffer::from_fn(10, 10, |_, _| Rgba([128, 128, 128, 255]));
        img.save(&cover_path).unwrap();

        // Try to embed more data than capacity
        let large_data = vec![0u8; 1000];

        let result = embed_data(
            cover_path.to_str().unwrap(),
            &large_data,
            stego_path.to_str().unwrap(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_no_stego_data() {
        let dir = tempdir().unwrap();
        let img_path = dir.path().join("clean.png");

        let img: RgbaImage = ImageBuffer::from_fn(50, 50, |_, _| Rgba([128, 128, 128, 255]));
        img.save(&img_path).unwrap();

        let result = extract_data(img_path.to_str().unwrap());
        assert!(result.is_err());
    }
}
