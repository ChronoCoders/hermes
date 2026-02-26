use crate::dms::{get_registry_path, DmsRegistry};
use crate::error::{HermesError, Result};
use crate::ui;

pub fn execute(file_path: &str) -> Result<()> {
    ui::print_box_start("CHECKIN");
    ui::print_box_line(&format!(">> File: {}", file_path));
    ui::print_box_line("");

    let registry_path = get_registry_path()?;
    let mut registry = DmsRegistry::load_from_file(&registry_path)?;

    let switch = registry
        .get_mut(file_path)
        .ok_or_else(|| HermesError::ConfigError(format!("No DMS found for file: {}", file_path)))?;

    if !switch.enabled {
        ui::print_box_line("   DMS is disabled for this file");
        ui::print_box_end();
        return Ok(());
    }

    let timeout = switch.timeout_hours;

    switch.checkin();

    let time_left = switch.time_until_deletion();

    registry.save_to_file(&registry_path)?;

    ui::print_box_line("   Check-in successful");
    ui::print_box_line("");
    ui::print_box_line(&format!(">> Next check-in required in: {} hours", timeout));
    ui::print_box_line(&format!(
        ">> Time until deletion: {}",
        crate::dms::format_duration(time_left)
    ));

    ui::print_box_end();

    println!();
    ui::print_success("CHECK-IN COMPLETE");
    ui::print_info("File", file_path);
    ui::print_info("Status", "Protected");
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
