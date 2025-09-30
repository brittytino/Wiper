use serde::{Serialize, Deserialize};
use ed25519_dalek::{Keypair, Signature, Signer};
use rand::rngs::OsRng;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeCertificate {
    pub version: String,
    pub tool_version: String,
    pub device: Device,
    pub operation: Operation,
    pub host: Host,
    pub transcript_blake3: String,
    pub pass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub model: String,
    pub serial: String,
    pub path: String,
    pub size_bytes: u64,
    pub bus: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub nist_level: String,
    pub primitive: String,
    pub started_at: String,
    pub finished_at: String,
    pub duration_secs: u64,
    pub verify: Verify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Verify {
    pub method: String,
    pub result: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host {
    pub os: String,
    pub hostname: String,
    pub host_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedCertificate {
    pub cert: WipeCertificate,
    pub pubkey: String,
    pub sig: String,
    pub signed_at: String,
}

pub fn sign_certificate(cert: &WipeCertificate) -> Result<SignedCertificate> {
    let json = serde_json::to_vec(cert)?;
    let mut csprng = OsRng {};
    let kp: Keypair = Keypair::generate(&mut csprng);
    let signature: Signature = kp.sign(&json);
    let sig_b64 = general_purpose::STANDARD.encode(signature.to_bytes());
    let pub_b64 = general_purpose::STANDARD.encode(kp.public.to_bytes());
    Ok(SignedCertificate {
        cert: cert.clone(),
        pubkey: pub_b64,
        sig: sig_b64,
        signed_at: Utc::now().to_rfc3339(),
    })
}
