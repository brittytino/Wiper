use serde::{Serialize, Deserialize};
use sysinfo::{System, SystemExt, DiskExt};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeviceInfo {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
    pub serial: Option<String>,
    pub bus: Option<String>,
    pub kind: Option<String>,
}

pub fn enumerate_devices() -> Result<Vec<DeviceInfo>, String> {
    let mut sys = System::new_all();
    sys.refresh_disks_list();
    let mut devices = Vec::new();
    for disk in sys.disks() {
        devices.push(DeviceInfo {
            path: disk.mount_point().to_string_lossy().to_string(),
            name: disk.name().to_string_lossy().to_string(),
            size_bytes: disk.total_space(),
            serial: None,
            bus: None,
            kind: Some(format!("{:?}", disk.kind())),
        });
    }
    Ok(devices)
}
