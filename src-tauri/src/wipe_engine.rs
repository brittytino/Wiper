use serde::{Deserialize, Serialize};
use std::fs::{OpenOptions};
use std::io::{Seek, SeekFrom, Write, Read};
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc::UnboundedSender;
use std::sync::{Arc, Mutex};
use ring::rand::{SystemRandom, SecureRandom};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WipeMethod {
    SinglePass,
    DoD522022M,
    Gutmann,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeProgress {
    pub current_pass: u32,
    pub total_passes: u32,
    pub progress_percent: f64,
    pub bytes_written: u64,
    pub total_size: u64,
    pub current_pattern: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct WipeResult {
    pub success: bool,
    pub total_time_seconds: u64,
    pub total_passes: u32,
    pub bytes_written: u64,
    pub error_message: Option<String>,
    // More fields as necessary for certificate
    pub wipe_certificate_id: String,
}

pub struct SecureWipeEngine {
    progress_sender: Option<UnboundedSender<WipeProgress>>,
    cancel_flag: Arc<Mutex<bool>>,
    rng: SystemRandom,
}

impl SecureWipeEngine {
    pub fn new() -> Self {
        Self {
            progress_sender: None,
            cancel_flag: Arc::new(Mutex::new(false)),
            rng: SystemRandom::new(),
        }
    }

    pub fn set_progress_callback(&mut self, sender: UnboundedSender<WipeProgress>) {
        self.progress_sender = Some(sender);
    }

    pub fn cancel_operation(&self) {
        let mut flag = self.cancel_flag.lock().unwrap();
        *flag = true;
    }

    fn is_cancelled(&self) -> bool {
        *self.cancel_flag.lock().unwrap()
    }

    pub async fn wipe_device(&self, device_path: &str, device_id: &str, method: WipeMethod) -> Result<WipeResult, std::io::Error> {
        use std::time::Instant;
        let start = Instant::now();

        let patterns = match method {
            WipeMethod::SinglePass => vec![vec![0x00]],
            WipeMethod::DoD522022M => vec![vec![0x01], vec![0xFE], vec![255]], // simplified patterns
            WipeMethod::Gutmann => vec![vec![0x55], vec![0xAA], vec![0x92], vec![0x49], vec![0x24]], // simplified subset
        };

        let total_passes = patterns.len() as u32;
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(device_path)?;

        file.seek(SeekFrom::End(0))?;
        let size = file.stream_position()?;
        file.seek(SeekFrom::Start(0))?;

        let mut bytes_written_total = 0u64;

        for (pass_idx, pattern_vec) in patterns.iter().enumerate() {
            if self.is_cancelled() {
                return Ok(WipeResult {
                    success: false,
                    total_time_seconds: start.elapsed().as_secs(),
                    total_passes,
                    bytes_written: bytes_written_total,
                    error_message: Some("Wipe cancelled".to_string()),
                    wipe_certificate_id: "".to_string(),
                });
            }

            let pattern = pattern_vec[0];
            let chunk_size = 1024 * 1024;
            let buffer = vec![pattern; chunk_size];
            let mut bytes_written_pass = 0u64;

            file.seek(SeekFrom::Start(0))?;

            while bytes_written_pass < size {
                let bytes_to_write = std::cmp::min(chunk_size as u64, size - bytes_written_pass) as usize;
                file.write_all(&buffer[..bytes_to_write])?;
                bytes_written_pass += bytes_to_write as u64;
                bytes_written_total += bytes_to_write as u64;

                if let Some(sender) = &self.progress_sender {
                    let _ = sender.send(WipeProgress {
                        current_pass: (pass_idx + 1) as u32,
                        total_passes,
                        progress_percent: (bytes_written_total as f64 / (size * total_passes as u64) as f64) * 100.0,
                        bytes_written: bytes_written_total,
                        total_size: size * total_passes as u64,
                        current_pattern: format!("Pattern 0x{:X}", pattern),
                        status: "In Progress".to_string(),
                    });
                }

                sleep(Duration::from_millis(10)).await;
            }
            file.sync_all()?;
        }

        Ok(WipeResult {
            success: true,
            total_time_seconds: start.elapsed().as_secs(),
            total_passes,
            bytes_written: bytes_written_total,
            error_message: None,
            wipe_certificate_id: uuid::Uuid::new_v4().to_string(),
        })
    }
}
