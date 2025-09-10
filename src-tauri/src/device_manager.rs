use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub device_type: String,
    pub is_removable: bool,
    pub is_mounted: bool,
}

pub struct DeviceManager {
    pub devices: HashMap<String, StorageDevice>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub async fn scan_devices(&mut self) -> Result<Vec<StorageDevice>, Box<dyn std::error::Error>> {
        // For demo purposes, mock devices or add real platform-specific detection logic
        self.devices.clear();

        // Example mock device
        let dev = StorageDevice {
            id: "device1".to_string(),
            name: "Demo USB".to_string(),
            path: "/dev/sdb".to_string(),
            size_bytes: 16_000_000_000,
            size_human: "14.9 GB".to_string(),
            device_type: "USB".to_string(),
            is_removable: true,
            is_mounted: true,
        };

        self.devices.insert(dev.id.clone(), dev.clone());

        Ok(self.devices.values().cloned().collect())
    }
}
