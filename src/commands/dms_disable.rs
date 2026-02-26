use crate::dms::{get_registry_path, DmsRegistry};
use crate::error::{HermesError, Result};
use crate::ui;

pub fn execute(file_path: &str) -> Result<()> {
    ui::print_box_start("DMS_DISABLE");
    ui::print_box_line(&format!(">> File: {}", file_path));
    ui::print_box_line("");

    let registry_path = get_registry_path()?;
    let mut registry = DmsRegistry::load_from_file(&registry_path)?;

    let switch = registry
        .get_mut(file_path)
        .ok_or_else(|| HermesError::ConfigError(format!("No DMS found for file: {}", file_path)))?;

    if !switch.enabled {
        ui::print_box_line("   ⚠ DMS already disabled");
        ui::print_box_end();
        return Ok(());
    }

    switch.disable();
    registry.save_to_file(&registry_path)?;

    ui::print_box_line("   ✓ DMS disabled successfully");
    ui::print_box_line("   ℹ File will NOT be auto-deleted");

    ui::print_box_end();

    println!();
    ui::print_success("DMS DISABLED");
    ui::print_info("File", file_path);
    ui::print_info("Status", "Manual control");
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
