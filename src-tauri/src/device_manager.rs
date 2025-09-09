use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDevice {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub device_type: DeviceType,
    pub is_removable: bool,
    pub is_mounted: bool,
    pub file_system: Option<String>,
    pub serial_number: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub health_status: HealthStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceType {
    HDD,
    SSD,
    USB,
    SD,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HealthStatus {
    Healthy,
    Warning,
    Critical,
    Unknown,
}

pub struct DeviceManager {
    devices: HashMap<String, StorageDevice>,
}

impl DeviceManager {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }

    pub async fn scan_devices(&mut self) -> Result<Vec<StorageDevice>, Box<dyn std::error::Error>> {
        self.devices.clear();
        
        #[cfg(target_os = "windows")]
        let devices = self.scan_windows_devices().await?;
        
        #[cfg(target_os = "linux")]
        let devices = self.scan_linux_devices().await?;
        
        #[cfg(target_os = "macos")]
        let devices = self.scan_macos_devices().await?;
        
        for device in &devices {
            self.devices.insert(device.id.clone(), device.clone());
        }
        
        Ok(devices)
    }

    #[cfg(target_os = "windows")]
    async fn scan_windows_devices(&self) -> Result<Vec<StorageDevice>, Box<dyn std::error::Error>> {
        let mut devices = Vec::new();
        
        // Use WMI to get physical drives
        let output = Command::new("wmic")
            .args(&["diskdrive", "get", "size,model,deviceid,mediatype", "/format:csv"])
            .output()?;
            
        let output_str = String::from_utf8_lossy(&output.stdout);
        
        for line in output_str.lines().skip(2) { // Skip headers
            if line.trim().is_empty() { continue; }
            
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() >= 5 {
                let device_id = parts[1].trim();
                let model = parts[2].trim();
                let size_str = parts[4].trim();
                
                if let Ok(size) = size_str.parse::<u64>() {
                    let device = StorageDevice {
                        id: Uuid::new_v4().to_string(),
                        name: format!("{} ({})", model, device_id),
                        path: device_id.to_string(),
                        size_bytes: size,
                        size_human: format_bytes(size),
                        device_type: self.detect_device_type(model, device_id),
                        is_removable: self.is_removable_windows(device_id).unwrap_or(false),
                        is_mounted: true,
                        file_system: None,
                        serial_number: None,
                        model: Some(model.to_string()),
                        vendor: None,
                        health_status: HealthStatus::Unknown,
                    };
                    devices.push(device);
                }
            }
        }
        
        Ok(devices)
    }

    #[cfg(target_os = "linux")]
    async fn scan_linux_devices(&self) -> Result<Vec<StorageDevice>, Box<dyn std::error::Error>> {
        let mut devices = Vec::new();
        
        // Use lsblk to get block devices
        let output = Command::new("lsblk")
            .args(&["-J", "-o", "NAME,SIZE,TYPE,MOUNTPOINT,FSTYPE,MODEL,SERIAL,TRAN"])
            .output()?;
            
        let output_str = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&output_str)?;
        
        if let Some(blockdevices) = json["blockdevices"].as_array() {
            for block in blockdevices {
                if let Some(device_type) = block["type"].as_str() {
                    if device_type == "disk" {
                        let name = block["name"].as_str().unwrap_or("unknown");
                        let size_str = block["size"].as_str().unwrap_or("0");
                        let model = block["model"].as_str();
                        let serial = block["serial"].as_str();
                        let transport = block["tran"].as_str();
                        
                        let size_bytes = parse_linux_size(size_str);
                        
                        let device = StorageDevice {
                            id: Uuid::new_v4().to_string(),
                            name: format!("/dev/{}", name),
                            path: format!("/dev/{}", name),
                            size_bytes,
                            size_human: format_bytes(size_bytes),
                            device_type: self.detect_device_type_linux(transport, model),
                            is_removable: transport == Some("usb"),
                            is_mounted: block["mountpoint"].as_str().is_some(),
                            file_system: block["fstype"].as_str().map(|s| s.to_string()),
                            serial_number: serial.map(|s| s.to_string()),
                            model: model.map(|s| s.to_string()),
                            vendor: None,
                            health_status: HealthStatus::Unknown,
                        };
                        devices.push(device);
                    }
                }
            }
        }
        
        Ok(devices)
    }

    fn detect_device_type(&self, model: &str, path: &str) -> DeviceType {
        let model_lower = model.to_lowercase();
        let path_lower = path.to_lowercase();
        
        if path_lower.contains("usb") || model_lower.contains("usb") {
            DeviceType::USB
        } else if model_lower.contains("ssd") || model_lower.contains("solid state") {
            DeviceType::SSD
        } else if model_lower.contains("sd") || model_lower.contains("secure digital") {
            DeviceType::SD
        } else if model_lower.contains("hdd") || model_lower.contains("hard disk") {
            DeviceType::HDD
        } else {
            DeviceType::Unknown
        }
    }

    fn detect_device_type_linux(&self, transport: Option<&str>, model: Option<&str>) -> DeviceType {
        match transport {
            Some("usb") => DeviceType::USB,
            Some("sata") | Some("ata") => {
                if let Some(model) = model {
                    if model.to_lowercase().contains("ssd") {
                        DeviceType::SSD
                    } else {
                        DeviceType::HDD
                    }
                } else {
                    DeviceType::Unknown
                }
            },
            Some("nvme") => DeviceType::SSD,
            _ => DeviceType::Unknown,
        }
    }

    pub async fn get_device_health(&self, device_id: &str) -> Result<HealthStatus, Box<dyn std::error::Error>> {
        // Implement SMART data reading for health status
        #[cfg(target_os = "linux")]
        {
            if let Some(device) = self.devices.get(device_id) {
                let output = Command::new("smartctl")
                    .args(&["-H", &device.path])
                    .output();
                    
                match output {
                    Ok(output) => {
                        let output_str = String::from_utf8_lossy(&output.stdout);
                        if output_str.contains("PASSED") {
                            Ok(HealthStatus::Healthy)
                        } else if output_str.contains("FAILED") {
                            Ok(HealthStatus::Critical)
                        } else {
                            Ok(HealthStatus::Warning)
                        }
                    },
                    Err(_) => Ok(HealthStatus::Unknown),
                }
            } else {
                Ok(HealthStatus::Unknown)
            }
        }
        
        #[cfg(not(target_os = "linux"))]
        Ok(HealthStatus::Unknown)
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

fn parse_linux_size(size_str: &str) -> u64 {
    if size_str.ends_with('K') {
        size_str.trim_end_matches('K').parse::<u64>().unwrap_or(0) * 1024
    } else if size_str.ends_with('M') {
        size_str.trim_end_matches('M').parse::<u64>().unwrap_or(0) * 1024 * 1024
    } else if size_str.ends_with('G') {
        size_str.trim_end_matches('G').parse::<u64>().unwrap_or(0) * 1024 * 1024 * 1024
    } else if size_str.ends_with('T') {
        size_str.trim_end_matches('T').parse::<u64>().unwrap_or(0) * 1024 * 1024 * 1024 * 1024
    } else {
        size_str.parse::<u64>().unwrap_or(0)
    }
}
