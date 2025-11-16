use crate::error::Result;
use crate::shamir::Share;
use crate::ui;
use std::fs;

pub fn execute(share_path: &str) -> Result<()> {
    ui::print_box_start("SHARE_VERIFY");
    ui::print_box_line(&format!(">> Share: {}", share_path));
    ui::print_box_line("");

    let json = fs::read_to_string(share_path)?;
    let share = Share::from_json(&json)?;

    ui::print_box_line(&format!(">> Share ID: {}", share.id));
    ui::print_box_line(&format!(">> Threshold: {}/{}", share.threshold, share.total_shares));
    ui::print_box_line(&format!(">> Data size: {} bytes", share.y.len()));
    ui::print_box_line("");

    if share.verify() {
        ui::print_box_line("   VERIFICATION: PASSED");
    } else {
        ui::print_box_line("   VERIFICATION: FAILED");
    }

    ui::print_box_end();

    println!();
    if share.verify() {
        ui::print_success("SHARE VALID");
    } else {
        println!("ERROR: Share verification failed");
        println!("Share may be corrupted or tampered with");
        return Err(crate::error::HermesError::ConfigError(
            "Share verification failed".to_string(),
        ));
    }
    ui::print_info("Share ID", &share.id.to_string());
    ui::print_info("Threshold", &format!("{}/{}", share.threshold, share.total_shares));
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
