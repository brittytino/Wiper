use crate::wipe_engine::WipeResult;
use crate::device_manager::StorageDevice;
use chrono::{Utc, DateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct WipeCertificate {
    pub certificate_id: String,
    pub device_name: String,
    pub wiped_at: DateTime<Utc>,
    pub total_bytes_erased: u64,
    pub total_passes: u32,
    pub success: bool,
}

pub struct CertificateGenerator {}

impl CertificateGenerator {
    pub fn new() -> Self {
        CertificateGenerator {}
    }

    pub async fn generate_certificate(&mut self, device: &StorageDevice, wipe_result: &WipeResult) -> Result<WipeCertificate, Box<dyn std::error::Error>> {
        Ok(WipeCertificate {
            certificate_id: wipe_result.wipe_certificate_id.clone(),
            device_name: device.name.clone(),
            wiped_at: Utc::now(),
            total_bytes_erased: wipe_result.bytes_written,
            total_passes: wipe_result.total_passes,
            success: wipe_result.success,
        })
    }
}
