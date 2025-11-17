use crate::error::{HermesError, Result};
use crate::steganography;
use crate::ui;

pub fn execute(image_path: &str, analyze: bool) -> Result<()> {
    ui::print_box_start("STEGO_CAPACITY");

    if !std::path::Path::new(image_path).exists() {
        return Err(HermesError::FileNotFound(image_path.to_string()));
    }

    ui::print_box_line(&format!(">> Image: {}", image_path));

    if analyze {
        ui::print_box_line(">> Analyzing image...");
        let analysis = steganography::analyze_image(image_path)?;

        ui::print_box_line(&format!(">> Dimensions: {}x{}", analysis.width, analysis.height));
        ui::print_box_line(&format!(">> Max capacity: {} bytes", analysis.capacity));
        ui::print_box_line(&format!(">> LSB ratio: {:.4}", analysis.lsb_ratio));
        ui::print_box_line(&format!(
            ">> Contains hidden data: {}",
            if analysis.likely_contains_data {
                "YES"
            } else {
                "NO"
            }
        ));
    } else {
        let capacity = steganography::get_capacity(image_path)?;
        ui::print_box_line(&format!(">> Max capacity: {} bytes", capacity));
        ui::print_box_line(&format!(">> Max capacity: {:.2} KB", capacity as f64 / 1024.0));
        ui::print_box_line(&format!(
            ">> Max capacity: {:.2} MB",
            capacity as f64 / 1024.0 / 1024.0
        ));
    }

    ui::print_box_line("");
    ui::print_box_end();

    println!();
    if analyze {
        ui::print_info("Analysis", "Complete");
    } else {
        ui::print_info("Capacity", "Calculated");
    }
    println!();

    Ok(())
}
