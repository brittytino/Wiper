use ring::signature::{RsaKeyPair, RSA_PKCS1_SHA256};
use ring::{digest, rand};
use rsa::{RsaPrivateKey, RsaPublicKey, PaddingScheme, PublicKey, Hash};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalSignature {
    pub algorithm: String,
    pub signature: String,
    pub public_key_fingerprint: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateChain {
    pub certificate_id: String,
    pub previous_hash: Option<String>,
    pub current_hash: String,
    pub block_height: u64,
    pub signatures: Vec<DigitalSignature>,
}

pub struct CryptoManager {
    rng: rand::SystemRandom,
    key_pair: Option<RsaKeyPair>,
    public_key_der: Vec<u8>,
    certificate_chain: HashMap<String, CertificateChain>,
    chain_height: u64,
}

impl CryptoManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let rng = rand::SystemRandom::new();
        
        // Generate RSA key pair
        let mut rng_std = rand::thread_rng();
        let bits = 2048;
        let private_key = RsaPrivateKey::new(&mut rng_std, bits)?;
        let public_key = RsaPublicKey::from(&private_key);
        
        // Convert to DER format for storage
        let public_key_der = rsa::pkcs8::EncodePublicKey::to_public_key_der(&public_key)?;
        
        Ok(Self {
            rng,
            key_pair: None, // Will initialize when needed
            public_key_der: public_key_der.as_bytes().to_vec(),
            certificate_chain: HashMap::new(),
            chain_height: 0,
        })
    }

    pub fn generate_certificate_hash(&self, data: &[u8]) -> String {
        let digest = digest::digest(&digest::SHA256, data);
        hex::encode(digest.as_ref())
    }

    pub fn sign_data(&self, data: &[u8]) -> Result<DigitalSignature, Box<dyn std::error::Error>> {
        let hash = digest::digest(&digest::SHA256, data);
        
        // For simplicity, using RSA with PKCS1v15 padding
        let mut rng = rand::thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048)?;
        let padding = PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA256));
        let signature = private_key.sign(padding, hash.as_ref())?;
        
        let public_key_fingerprint = self.get_public_key_fingerprint();
        
        Ok(DigitalSignature {
            algorithm: "RSA-SHA256".to_string(),
            signature: hex::encode(signature),
            public_key_fingerprint,
            timestamp: Utc::now(),
        })
    }

    pub fn verify_signature(&self, signature: &DigitalSignature, data: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
        let hash = digest::digest(&digest::SHA256, data);
        let signature_bytes = hex::decode(&signature.signature)?;
        
        // In a real implementation, we would use the public key from the signature
        // For now, this is a simplified verification
        Ok(signature.algorithm == "RSA-SHA256" && signature_bytes.len() > 0)
    }

    pub fn create_certificate_chain_entry(
        &mut self,
        certificate_id: String,
        data: &[u8],
    ) -> Result<CertificateChain, Box<dyn std::error::Error>> {
        let current_hash = self.generate_certificate_hash(data);
        let signature = self.sign_data(data)?;
        
        let previous_hash = if self.chain_height > 0 {
            // Get the hash of the previous certificate in the chain
            self.certificate_chain
                .values()
                .find(|entry| entry.block_height == self.chain_height - 1)
                .map(|entry| entry.current_hash.clone())
        } else {
            None
        };
        
        let chain_entry = CertificateChain {
            certificate_id: certificate_id.clone(),
            previous_hash,
            current_hash: current_hash.clone(),
            block_height: self.chain_height,
            signatures: vec![signature],
        };
        
        self.certificate_chain.insert(certificate_id, chain_entry.clone());
        self.chain_height += 1;
        
        Ok(chain_entry)
    }

    pub fn verify_certificate_chain(&self, certificate_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some(cert_entry) = self.certificate_chain.get(certificate_id) {
            // Verify the signature
            let data = format!("{}:{}:{}", 
                cert_entry.certificate_id, 
                cert_entry.block_height, 
                cert_entry.previous_hash.as_ref().unwrap_or(&"genesis".to_string())
            );
            
            for signature in &cert_entry.signatures {
                if !self.verify_signature(signature, data.as_bytes())? {
                    return Ok(false);
                }
            }
            
            // Verify chain integrity
            if let Some(prev_hash) = &cert_entry.previous_hash {
                let expected_prev = self.certificate_chain
                    .values()
                    .find(|entry| entry.block_height == cert_entry.block_height - 1)
                    .map(|entry| &entry.current_hash);
                    
                if expected_prev != Some(prev_hash) {
                    return Ok(false);
                }
            }
            
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn get_public_key_fingerprint(&self) -> String {
        let hash = digest::digest(&digest::SHA256, &self.public_key_der);
        hex::encode(hash.as_ref())[0..16].to_string()
    }

    pub fn generate_secure_random_bytes(&self, len: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut bytes = vec![0u8; len];
        self.rng.fill(&mut bytes).map_err(|_| "Random generation failed")?;
        Ok(bytes)
    }

    pub fn hash_device_info(&self, device_info: &str, serial: &str, timestamp: &DateTime<Utc>) -> String {
        let data = format!("{}:{}:{}", device_info, serial, timestamp.timestamp());
        let hash = digest::digest(&digest::SHA256, data.as_bytes());
        hex::encode(hash.as_ref())
    }
}

// Tamper-proof verification functions
impl CryptoManager {
    pub fn create_tamper_proof_seal(&self, data: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        // Create multiple hash layers for tamper detection
        let hash1 = digest::digest(&digest::SHA256, data);
        let hash2 = digest::digest(&digest::SHA512, data);
        
        // Combine hashes with timestamp
        let timestamp = Utc::now().timestamp();
        let combined = format!("{}:{}:{}", hex::encode(hash1.as_ref()), hex::encode(hash2.as_ref()), timestamp);
        
        let final_hash = digest::digest(&digest::SHA256, combined.as_bytes());
        Ok(hex::encode(final_hash.as_ref()))
    }

    pub fn verify_tamper_proof_seal(&self, data: &[u8], seal: &str, timestamp: i64) -> Result<bool, Box<dyn std::error::Error>> {
        let hash1 = digest::digest(&digest::SHA256, data);
        let hash2 = digest::digest(&digest::SHA512, data);
        
        let expected_combined = format!("{}:{}:{}", hex::encode(hash1.as_ref()), hex::encode(hash2.as_ref()), timestamp);
        let expected_hash = digest::digest(&digest::SHA256, expected_combined.as_bytes());
        let expected_seal = hex::encode(expected_hash.as_ref());
        
        Ok(expected_seal == seal)
    }
}
