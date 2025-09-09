#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod device_manager;
mod wipe_engine;
mod certificate_generator;
mod audit_logger;
mod crypto;
mod config;

use tauri::{Manager};
use device_manager::{DeviceManager, StorageDevice};
use wipe_engine::{SecureWipeEngine, WipeMethod};
use certificate_generator::{CertificateGenerator, WipeCertificate};
use audit_logger::{AuditLogger, AuditLogEntry, AuditEvent};
use once_cell::sync::OnceCell;
use std::sync::{Mutex, Arc};
use tokio::sync::Mutex as AsyncMutex;

static DEVICE_MANAGER: OnceCell<Mutex<DeviceManager>> = OnceCell::new();
static AUDIT_LOGGER: OnceCell<AuditLogger> = OnceCell::new();
static WIPE_ENGINE: OnceCell<Arc<AsyncMutex<SecureWipeEngine>>> = OnceCell::new();
static CERTIFICATE_GENERATOR: OnceCell<Mutex<CertificateGenerator>> = OnceCell::new();

#[tauri::command]
async fn scan_storage_devices() -> Result<Vec<StorageDevice>, String> {
    let mut manager = DEVICE_MANAGER.get_or_init(|| Mutex::new(DeviceManager::new())).lock().unwrap();
    manager.scan_devices().await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_audit_logs() -> Result<Vec<AuditLogEntry>, String> {
    let logger = AUDIT_LOGGER.get_or_init(|| AuditLogger::new("audit_log.json"));
    Ok(logger.get_logs().await)
}

#[tauri::command]
async fn start_wipe(
    device_path: String,
    device_id: String,
    method: Option<String>,
) -> Result<WipeCertificate, String> {

    let method = match method.as_deref() {
        Some("dod") => WipeMethod::DoD522022M,
        Some("single") => WipeMethod::SinglePass,
        _ => WipeMethod::DoD522022M,
    };

    let audit_logger = AUDIT_LOGGER.get_or_init(|| AuditLogger::new("audit_log.json"));
    let engine_arc = WIPE_ENGINE.get_or_init(|| {
        Arc::new(AsyncMutex::new(SecureWipeEngine::new(audit_logger.clone())))
    }).clone();

    let mut engine = engine_arc.lock().await;
    let result = engine.wipe_device(&device_path, &device_id, method).await.map_err(|e| e.to_string())?;

    if !result.success {
        return Err(format!("Wipe failed: {:?}", result.error_message));
    }

    let device = {
        let dm = DEVICE_MANAGER.get_or_init(|| Mutex::new(DeviceManager::new())).lock().unwrap();
        dm.devices.get(&device_id).cloned()
    }.ok_or_else(|| "Device not found".to_string())?;

    let mut generator = CERTIFICATE_GENERATOR.get_or_init(|| {
        let crypto = crypto::CryptoManager::new().expect("Failed to initialize CryptoManager");
        Mutex::new(CertificateGenerator::new(crypto))
    }).lock().unwrap();

    generator.generate_certificate(&device, &result).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cancel_wipe() -> Result<(), String> {
    if let Some(engine_arc) = WIPE_ENGINE.get() {
        let engine = engine_arc.lock().await;
        engine.cancel_operation();
        Ok(())
    } else {
        Err("No wipe operation to cancel".to_string())
    }
}

fn main() {
    // Initialize statics correctly without arguments
    DEVICE_MANAGER.set(Mutex::new(DeviceManager::new()))
        .expect("Failed to initialize DEVICE_MANAGER");

    AUDIT_LOGGER.set(AuditLogger::new("audit_log.json"))
        .expect("Failed to initialize AUDIT_LOGGER");

    // Lazy initialization for WIPE_ENGINE and CERTIFICATE_GENERATOR via get_or_init()

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_storage_devices,
            get_audit_logs,
            start_wipe,
            cancel_wipe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
