use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AuditLogger {
    log_file_path: String,
    cache: Arc<Mutex<Vec<AuditLogEntry>>>,
}

impl AuditLogger {
    pub fn new(path: &str) -> Self {
        Self {
            log_file_path: path.to_owned(),
            cache: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn log(&self, message: String) -> Result<(), std::io::Error> {
        let entry = AuditLogEntry {
            message,
            timestamp: Utc::now(),
        };

        {
            let mut cache = self.cache.lock().unwrap();
            cache.push(entry.clone());
        }

        let serialized = serde_json::to_string(&entry).unwrap_or_else(|_| "{}".to_string());

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file_path)
            .await?;

        file.write_all(serialized.as_bytes()).await?;
        file.write_all(b"\n").await?;
        file.flush().await?;

        Ok(())
    }

    pub async fn get_logs(&self) -> Vec<AuditLogEntry> {
        {
            let cache = self.cache.lock().unwrap();
            if !cache.is_empty() {
                return cache.clone();
            }
        }

        if let Ok(content) = tokio::fs::read_to_string(&self.log_file_path).await {
            return content.lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect();
        }

        Vec::new()
    }
}
