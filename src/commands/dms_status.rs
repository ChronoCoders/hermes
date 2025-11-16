use crate::dms::{get_registry_path, DmsRegistry};
use crate::error::Result;
use crate::ui;
use colored::Colorize;

pub fn execute() -> Result<()> {
    ui::print_box_start("DMS_STATUS");
    ui::print_box_line(">> Dead Man's Switch Status");
    ui::print_box_line("");

    let registry_path = get_registry_path()?;
    let registry = DmsRegistry::load_from_file(&registry_path)?;

    if registry.switches.is_empty() {
        ui::print_box_line("   No active Dead Man's Switches");
        ui::print_box_end();
        println!();
        ui::print_info("Total", "0");
        return Ok(());
    }

    let mut active = 0;
    let mut warning = 0;
    let mut expired = 0;

    for (path, switch) in &registry.switches {
        if !switch.enabled {
            continue;
        }

        active += 1;

        let status = if switch.is_expired() {
            expired += 1;
            "EXPIRED".red().to_string()
        } else if switch.is_in_grace_period() {
            warning += 1;
            "WARNING".yellow().to_string()
        } else {
            "ACTIVE".green().to_string()
        };

        ui::print_box_line(&format!(">> File: {}", path));
        ui::print_box_line(&format!("   Status: {}", status));
        ui::print_box_line(&format!(
            "   Time Left: {}",
            crate::dms::format_duration(switch.time_until_deletion())
        ));
        ui::print_box_line(&format!("   Timeout: {} hours", switch.timeout_hours));
        ui::print_box_line("");
    }

    ui::print_box_end();

    println!();
    ui::print_success("DMS STATUS");
    ui::print_info("Total Active", &active.to_string());
    ui::print_info("Warnings", &warning.to_string());
    ui::print_info("Expired", &expired.to_string());
    ui::print_status("COMPLETE");
    println!();

    Ok(())
}
