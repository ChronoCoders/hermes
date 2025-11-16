use crate::crypto::rsa::load_private_key;
use crate::error::Result;
use crate::shamir::split_secret;
use crate::ui;
use std::fs;
use std::path::Path;

pub fn execute(name: &str, threshold: u8, total_shares: u8, output_dir: Option<&str>) -> Result<()> {
    ui::print_box_start("KEY_SPLIT");
    ui::print_box_line(&format!(">> Key: {}", name));
    ui::print_box_line(&format!(">> Threshold: {}/{}", threshold, total_shares));
    ui::print_box_line("");

    let private_key = load_private_key(name)?;
    let key_bytes = private_key.to_pkcs8_der()?.as_bytes().to_vec();

    ui::print_box_line(&format!(">> Key size: {} bytes", key_bytes.len()));
    ui::print_box_line(">> Splitting into shares...");

    let shares = split_secret(&key_bytes, threshold, total_shares)?;

    let output_path = if let Some(dir) = output_dir {
        Path::new(dir).to_path_buf()
    } else {
        Path::new(".").join(format!("{}_shares", name))
    };

    fs::create_dir_all(&output_path)?;

    for share in &shares {
        let share_filename = format!("share_{}_of_{}.json", share.id, total_shares);
        let share_path = output_path.join(&share_filename);
        
        let json = share.to_json()?;
        fs::write(&share_path, json)?;
        
        ui::print_box_line(&format!("   Share {}: {}", share.id, share_filename));
    }

    ui::print_box_line("");
    ui::print_box_line(&format!(">> Output directory: {}", output_path.display()));

    ui::print_box_end();

    println!();
    ui::print_success("KEY SPLIT COMPLETE");
    ui::print_info("Key Name", name);
    ui::print_info("Total Shares", &total_shares.to_string());
    ui::print_info("Threshold", &threshold.to_string());
    ui::print_info("Output", &output_path.display().to_string());
    ui::print_status("COMPLETE");
    println!();

    println!("IMPORTANT:");
    println!("- Distribute shares to {} different trusted parties", total_shares);
    println!("- Need any {} shares to recover the key", threshold);
    println!("- Keep shares secure and separate");
    println!();

    Ok(())
}
