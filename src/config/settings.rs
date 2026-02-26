use crate::error::{HermesError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const ENV_USERNAME: &str = "USERNAME";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SftpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub key_file: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PathsConfig {
    pub inbox: String,
    pub outbox: String,
    pub files: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub sftp: SftpConfig,
    pub paths: PathsConfig,
}

impl Settings {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            return Err(HermesError::ConfigError(
                "Configuration file not found. Run 'hermes init' first.".to_string(),
            ));
        }

        let contents = fs::read_to_string(&config_path)
            .map_err(|e| HermesError::ConfigError(format!("Failed to read config: {e}")))?;

        toml::from_str(&contents)
            .map_err(|e| HermesError::ConfigError(format!("Invalid config format: {e}")))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                HermesError::ConfigError(format!("Failed to create config directory: {e}"))
            })?;
        }

        let contents = toml::to_string_pretty(self)
            .map_err(|e| HermesError::ConfigError(format!("Failed to serialize config: {e}")))?;

        fs::write(&config_path, contents)
            .map_err(|e| HermesError::ConfigError(format!("Failed to write config: {e}")))?;

        Ok(())
    }

    fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir().ok_or_else(|| {
            HermesError::ConfigError("Could not determine config directory".to_string())
        })?;

        Ok(config_dir.join("hermes").join("config.toml"))
    }

    #[must_use]
    pub fn default_config() -> Self {
        let username = std::env::var(ENV_USERNAME).unwrap_or_else(|_| "user".to_string());
        let key_path = format!("C:\\Users\\{username}\\.ssh\\hermes_key");

        Self {
            sftp: SftpConfig {
                host: "localhost".to_string(),
                port: 22,
                username,
                key_file: Some(key_path),
            },
            paths: PathsConfig {
                inbox: "C:\\hermes_vault\\inbox".to_string(),
                outbox: "C:\\hermes_vault\\outbox".to_string(),
                files: "C:\\hermes_vault\\files".to_string(),
            },
        }
    }
}
