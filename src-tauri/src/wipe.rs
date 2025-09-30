use serde::{Serialize, Deserialize};
use sysinfo::{SystemExt, DiskExt};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
    pub serial: Option<String>,
    pub bus: Option<String>,
    pub kind: Option<String>,
}

pub fn enumerate_devices() -> anyhow::Result<Vec<DeviceInfo>> {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_disks_list();
    let disks = sys.disks();

    let mut out = Vec::new();
    for d in disks {
        let name = d.name().to_string_lossy().to_string();
        let mount = d.mount_point().to_string_lossy().to_string();
        let total = d.total_space();
        out.push(DeviceInfo {
            path: mount.clone(),
            name: format!("{} ({})", name, d.file_system().iter().map(|c| *c as char).collect::<String>()),
            size_bytes: total,
            serial: None,
            bus: None,
            kind: None,
        });
    }

    // If no disks found via sysinfo, include an example/dev placeholder
    if out.is_empty() {
        out.push(DeviceInfo {
            path: "/dev/sdX".to_string(),
            name: "Example Disk".to_string(),
            size_bytes: 128_000_000_000u64,
            serial: Some("EXAMPLE1234".to_string()),
            bus: Some("ata".to_string()),
            kind: Some("ssd".to_string()),
        });
    }
    Ok(out)
}
