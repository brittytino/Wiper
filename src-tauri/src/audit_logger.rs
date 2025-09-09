use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use tokio::fs::{OpenOptions};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEvent {
    WipeStarted {
        device_id: String,
        device_path: String,
        method: String,
        certificate_id: String,
        timestamp: DateTime<Utc>,
    },
    WipeCompleted {
        device_id: String,
        certificate_id: String,
        success: bool,
        duration_seconds: u64,
        timestamp: DateTime<Utc>,
    },
    WipeFailed {
        device_id: String,
        certificate_id: String,
        error: String,
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub event_type: String,
    pub details: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone)]
pub struct AuditLogger {
    file_path: String,
    cache: Arc<Mutex<Vec<AuditLogEntry>>>,
}

impl AuditLogger {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
            cache: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn log_event(&self, event: AuditEvent) -> tokio::io::Result<()> {
        let entry = AuditLogEntry {
            event_type: format!("{:?}", event),
            details: serde_json::to_string(&event).unwrap_or_default(),
            timestamp: Utc::now(),
        };

        {
            let mut cache = self.cache.lock().unwrap();
            cache.push(entry.clone());
        }

        let serialized = serde_json::to_string(&entry).unwrap_or_default();
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .await?;
        file.write_all(serialized.as_bytes()).await?;
        file.write_all(b"\n").await?;
        file.flush().await?;

        Ok(())
    }

    pub async fn get_logs(&self) -> Vec<AuditLogEntry> {
        let cache = self.cache.lock().unwrap();
        if !cache.is_empty() {
            return cache.clone();
        }
        drop(cache);

        if let Ok(content) = tokio::fs::read_to_string(&self.file_path).await {
            return content
                .lines()
                .filter_map(|line| serde_json::from_str(line).ok())
                .collect();
        }

        Vec::new()
    }
}
