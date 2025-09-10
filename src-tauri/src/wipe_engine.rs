use std::fs::OpenOptions;
use std::io::{Write, Seek, SeekFrom};
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};
use ring::rand::{SystemRandom, SecureRandom};

#[derive(Clone, Debug)]
pub enum WipeMethod {
    SinglePass,
    DoD522022M,
}

pub struct WipeResult {
    pub success: bool,
    pub total_time_seconds: u64,
    pub total_passes: u32,
    pub bytes_written: u64,
    pub error_message: Option<String>,
    pub wipe_certificate_id: String,
}

pub struct SecureWipeEngine {
    cancel_flag: Arc<Mutex<bool>>,
    rng: SystemRandom,
}

impl SecureWipeEngine {
    pub fn new() -> Self {
        Self {
            cancel_flag: Arc::new(Mutex::new(false)),
            rng: SystemRandom::new(),
        }
    }

    pub fn cancel_operation(&self) {
        let mut cancel = self.cancel_flag.lock().unwrap();
        *cancel = true;
    }

    fn is_cancelled(&self) -> bool {
        *self.cancel_flag.lock().unwrap()
    }

    pub async fn wipe_device(
        &self,
        device_path: &str,
        _device_id: &str,
        method: WipeMethod,
    ) -> Result<WipeResult, Box<dyn std::error::Error>> {
        use std::time::Instant;

        let start_time = Instant::now();

        let passes = match method {
            WipeMethod::SinglePass => vec![0u8],  // Zero pass
            WipeMethod::DoD522022M => vec![0xFF, 0x00, 0xFF], // Example three passes
        };

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(device_path)?;

        // Size detection (simplified here)
        let size = file.seek(SeekFrom::End(0))?;
        file.seek(SeekFrom::Start(0))?;

        let chunk_size = 1024 * 1024; // 1MB buffer
        let mut buffer = vec![0u8; chunk_size];

        let mut total_bytes_written = 0u64;

        for (pass_idx, &val) in passes.iter().enumerate() {
            if self.is_cancelled() {
                return Ok(WipeResult {
                    success: false,
                    total_time_seconds: start_time.elapsed().as_secs(),
                    total_passes: pass_idx as u32,
                    bytes_written: total_bytes_written,
                    error_message: Some("Cancelled".to_string()),
                    wipe_certificate_id: "".to_string(),
                });
            }

            buffer.fill(val);

            file.seek(SeekFrom::Start(0))?;

            let mut bytes_written = 0u64;
            while bytes_written < size {
                let bytes_to_write = std::cmp::min(chunk_size as u64, size - bytes_written) as usize;
                file.write_all(&buffer[..bytes_to_write])?;
                bytes_written += bytes_to_write as u64;
                total_bytes_written += bytes_to_write as u64;

                sleep(Duration::from_millis(1)).await;
            }

            file.sync_all()?;
        }

        Ok(WipeResult {
            success: true,
            total_time_seconds: start_time.elapsed().as_secs(),
            total_passes: passes.len() as u32,
            bytes_written: total_bytes_written,
            error_message: None,
            wipe_certificate_id: uuid::Uuid::new_v4().to_string(),
        })
    }
}
