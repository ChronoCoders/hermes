use crate::error::{HermesError, Result};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeadManSwitch {
    pub file_path: String,
    pub timeout_hours: u64,
    pub last_checkin: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub enabled: bool,
    pub grace_period_hours: u64,
    pub notification_email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DmsRegistry {
    pub switches: HashMap<String, DeadManSwitch>,
}

impl DeadManSwitch {
    pub fn new(file_path: String, timeout_hours: u64) -> Self {
        let now = Utc::now();
        Self {
            file_path,
            timeout_hours,
            last_checkin: now,
            created_at: now,
            enabled: true,
            grace_period_hours: timeout_hours / 4,
            notification_email: None,
        }
    }

    pub fn checkin(&mut self) {
        self.last_checkin = Utc::now();
    }

    pub fn time_until_deletion(&self) -> Duration {
        let deadline = self.last_checkin + Duration::hours(self.timeout_hours as i64);
        deadline - Utc::now()
    }

    pub fn is_expired(&self) -> bool {
        self.time_until_deletion() <= Duration::zero()
    }

    pub fn is_in_grace_period(&self) -> bool {
        let time_left = self.time_until_deletion();
        time_left > Duration::zero() && time_left <= Duration::hours(self.grace_period_hours as i64)
    }

    pub fn time_until_warning(&self) -> Duration {
        let warning_time = self.last_checkin
            + Duration::hours((self.timeout_hours - self.grace_period_hours) as i64);
        warning_time - Utc::now()
    }

    pub fn needs_warning(&self) -> bool {
        self.is_in_grace_period() && self.enabled
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }
}

impl DmsRegistry {
    pub fn new() -> Self {
        Self {
            switches: HashMap::new(),
        }
    }

    pub fn add(&mut self, switch: DeadManSwitch) {
        self.switches.insert(switch.file_path.clone(), switch);
    }

    pub fn remove(&mut self, file_path: &str) -> Option<DeadManSwitch> {
        self.switches.remove(file_path)
    }

    pub fn get(&self, file_path: &str) -> Option<&DeadManSwitch> {
        self.switches.get(file_path)
    }

    pub fn get_mut(&mut self, file_path: &str) -> Option<&mut DeadManSwitch> {
        self.switches.get_mut(file_path)
    }

    pub fn get_expired(&self) -> Vec<&DeadManSwitch> {
        self.switches
            .values()
            .filter(|s| s.enabled && s.is_expired())
            .collect()
    }

    pub fn get_warning_needed(&self) -> Vec<&DeadManSwitch> {
        self.switches
            .values()
            .filter(|s| s.needs_warning())
            .collect()
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Ok(Self::new());
        }

        let mut file = File::open(path)?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;

        serde_json::from_str(&json).map_err(HermesError::SerializationError)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(self).map_err(HermesError::SerializationError)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

impl Default for DmsRegistry {
    fn default() -> Self {
        Self {
            switches: HashMap::new(),
        }
    }
}

pub fn get_registry_path() -> Result<std::path::PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| HermesError::ConfigError("Cannot find home directory".to_string()))?;

    let config_dir = home.join(".hermes");
    std::fs::create_dir_all(&config_dir)?;

    Ok(config_dir.join("dms_registry.json"))
}

pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds();

    if total_seconds < 0 {
        return "EXPIRED".to_string();
    }

    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}
