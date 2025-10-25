use crate::config::Settings;
use crate::error::Result;
use crate::ui;
use std::fs;

pub fn execute() -> Result<()> {
    ui::print_box_start("INITIALIZE");

    ui::print_box_line(">> Creating configuration...");

    let config = Settings::default_config();
    let config_dir = dirs::config_dir()
        .ok_or_else(|| {
            crate::error::HermesError::ConfigError("Could not find config directory".to_string())
        })?
        .join("hermes");

    fs::create_dir_all(&config_dir)?;

    let config_path = config_dir.join("config.toml");
    let toml_string = toml::to_string_pretty(&config)
        .map_err(|e| crate::error::HermesError::ConfigError(e.to_string()))?;

    fs::write(&config_path, toml_string)?;

    ui::print_box_line(">> Configuration saved");
    ui::print_box_line("");
    ui::print_box_end();

    println!();
    ui::print_success("HERMES INITIALIZED");
    ui::print_info("Config", &config_path.display().to_string());
    ui::print_info("Status", "Ready for secure transfers");
    println!();

    Ok(())
}
