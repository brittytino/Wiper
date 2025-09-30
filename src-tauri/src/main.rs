#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod device;
mod cert;

use tauri::{Window};
use tauri::Emitter;
use chrono::Utc;
use std::time::Duration;

use device::DeviceInfo;
use cert::{WipeCertificate, SignedCertificate, sign_certificate};

#[tauri::command]
async fn get_devices() -> Result<Vec<DeviceInfo>, String> {
    device::enumerate_devices().map_err(|e| e.to_string())
}

#[tauri::command]
async fn wipe_device(
    window: Window,
    device_path: String,
    method: String,
    operator: Option<String>
) -> Result<serde_json::Value, String> {
    let operator = operator.unwrap_or_else(|| "unknown".to_string());
    let devices = device::enumerate_devices().map_err(|e| e.to_string())?;
    let device = devices.into_iter().find(|d| d.path == device_path)
        .unwrap_or(DeviceInfo {
            path: device_path.clone(),
            name: device_path.clone(),
            size_bytes: 0,
            serial: None,
            bus: None,
            kind: None,
        });

    // Simulate wipe flow
    for (i, (pct, msg)) in vec![
        (5, "Preparing device"),
        (12, "Removing HPA/DCO (simulated)"),
        (30, "Issuing secure erase / sanitize (simulated)"),
        (60, "Waiting for device to complete (simulated)"),
        (85, "Verifying erase (simulated)"),
        (98, "Finalizing"),
    ].into_iter().enumerate() {
        let payload = serde_json::json!({ "pct": pct, "msg": msg });
        let _ = window.emit("wipe-progress", payload);
        tokio::time::sleep(Duration::from_secs(1 + (i as u64))).await;
    }

    let started_at = Utc::now();
    let finished_at = Utc::now();

    let cert = WipeCertificate {
        version: "1.0".to_string(),
        tool_version: env!("CARGO_PKG_VERSION").to_string(),
        device: cert::Device {
            model: device.name.clone(),
            serial: device.serial.clone().unwrap_or_default(),
            path: device.path.clone(),
            size_bytes: device.size_bytes,
            bus: device.bus.clone().unwrap_or_default(),
            kind: device.kind.clone().unwrap_or_default(),
        },
        operation: cert::Operation {
            nist_level: if method.to_lowercase() == "purge" { "Purge".to_string() } else { "Clear".to_string() },
            primitive: format!("SIMULATED_{}", method.to_uppercase()),
            started_at: started_at.to_rfc3339(),
            finished_at: finished_at.to_rfc3339(),
            duration_secs: (finished_at - started_at).num_seconds() as u64,
            verify: cert::Verify { method: "simulated".to_string(), result: true },
        },
        host: cert::Host {
            os: std::env::consts::OS.to_string(),
            hostname: hostname::get().ok().and_then(|h| h.into_string().ok()).unwrap_or_default(),
            host_fingerprint: "".to_string(),
        },
        transcript_blake3: "".to_string(),
        pass: true,
    };

    let sig: SignedCertificate = sign_certificate(&cert).map_err(|e| e.to_string())?;
    let mut json_path = std::env::temp_dir();
    let filename = format!("securewipe_cert_{}.json", Utc::now().timestamp());
    json_path.push(&filename);
    let signed_json = serde_json::to_vec_pretty(&sig).map_err(|e| e.to_string())?;
    std::fs::write(&json_path, &signed_json).map_err(|e| e.to_string())?;

    let _ = window.emit("wipe-progress", serde_json::json!({ "pct": 100, "msg": "Completed" }));

    Ok(serde_json::json!({
        "json": json_path.to_string_lossy().to_string(),
        "pdf": "", // PDF generation can be integrated via 'printpdf' or 'wkhtmltopdf' crate
    }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_devices, wipe_device])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
