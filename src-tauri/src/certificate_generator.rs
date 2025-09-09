use printpdf::*;
use qrcode::QrCode;
use image::{ImageBuffer, Rgb, DynamicImage};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::crypto::{CryptoManager, DigitalSignature, CertificateChain};
use crate::device_manager::StorageDevice;
use crate::wipe_engine::{WipeResult, WipeMethod};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeCertificate {
    pub certificate_id: String,
    pub version: String,
    pub device_info: DeviceCertificateInfo,
    pub wipe_details: WipeDetails,
    pub verification_data: VerificationData,
    pub digital_signatures: Vec<DigitalSignature>,
    pub certificate_chain: Option<CertificateChain>,
    pub qr_code_data: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCertificateInfo {
    pub name: String,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub manufacturer: Option<String>,
    pub capacity_bytes: u64,
    pub capacity_human: String,
    pub device_type: String,
    pub device_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WipeDetails {
    pub method_used: String,
    pub total_passes: u32,
    pub bytes_processed: u64,
    pub duration_seconds: u64,
    pub verification_passed: bool,
    pub bad_sectors_count: u64,
    pub standards_compliance: Vec<String>,
    pub wipe_patterns_used: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationData {
    pub verification_hash: String,
    pub tamper_proof_seal: String,
    pub verification_url: String,
    pub chain_verification_hash: Option<String>,
    pub public_key_fingerprint: String,
}

pub struct CertificateGenerator {
    crypto_manager: CryptoManager,
}

impl CertificateGenerator {
    pub fn new(crypto_manager: CryptoManager) -> Self {
        Self { crypto_manager }
    }

    pub async fn generate_certificate(
        &mut self,
        device: &StorageDevice,
        wipe_result: &WipeResult,
    ) -> Result<WipeCertificate, Box<dyn std::error::Error>> {
        let certificate_id = wipe_result.wipe_certificate_id.clone();
        let created_at = Utc::now();

        // Create device info
        let device_info = DeviceCertificateInfo {
            name: device.name.clone(),
            model: device.model.clone(),
            serial_number: device.serial_number.clone(),
            manufacturer: device.vendor.clone(),
            capacity_bytes: device.size_bytes,
            capacity_human: device.size_human.clone(),
            device_type: format!("{:?}", device.device_type),
            device_path: device.path.clone(),
        };

        // Create wipe details
        let wipe_details = WipeDetails {
            method_used: format!("{:?}", wipe_result.method_used),
            total_passes: wipe_result.total_passes,
            bytes_processed: wipe_result.bytes_processed,
            duration_seconds: wipe_result.total_time.as_secs(),
            verification_passed: wipe_result.verification_passed,
            bad_sectors_count: wipe_result.bad_sectors_count,
            standards_compliance: self.get_compliance_standards(&wipe_result.method_used),
            wipe_patterns_used: self.get_pattern_descriptions(&wipe_result.method_used),
        };

        // Create verification data
        let certificate_data = self.create_certificate_data(&device_info, &wipe_details, &created_at);
        let verification_hash = self.crypto_manager.generate_certificate_hash(&certificate_data);
        let tamper_proof_seal = self.crypto_manager.create_tamper_proof_seal(&certificate_data)?;
        
        let verification_url = format!(
            "https://verify.securewipe.app/certificate/{}?hash={}", 
            certificate_id, 
            verification_hash
        );

        let verification_data = VerificationData {
            verification_hash: verification_hash.clone(),
            tamper_proof_seal,
            verification_url: verification_url.clone(),
            chain_verification_hash: None,
            public_key_fingerprint: "".to_string(), // Will be set after signing
        };

        // Create digital signature
        let signature_data = format!("{}:{}:{}", certificate_id, verification_hash, created_at.timestamp());
        let digital_signature = self.crypto_manager.sign_data(signature_data.as_bytes())?;

        // Create certificate chain entry
        let certificate_chain = self.crypto_manager.create_certificate_chain_entry(
            certificate_id.clone(),
            &certificate_data,
        )?;

        // Generate QR code
        let qr_code_data = self.generate_qr_code(&verification_url)?;

        let certificate = WipeCertificate {
            certificate_id,
            version: "1.0".to_string(),
            device_info,
            wipe_details,
            verification_data,
            digital_signatures: vec![digital_signature],
            certificate_chain: Some(certificate_chain),
            qr_code_data,
            created_at,
            expires_at: Some(created_at + chrono::Duration::days(365 * 5)), // 5 years
        };

        Ok(certificate)
    }

    pub fn generate_pdf_certificate(&self, certificate: &WipeCertificate) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let (doc, page1, layer1) = PdfDocument::new("Secure Data Wipe Certificate", Mm(210.0), Mm(297.0), "Certificate");
        let current_layer = doc.get_page(page1).get_layer(layer1);

        // Load fonts
        let font_bold = doc.add_builtin_font(BuiltinFont::HelveticaBold)?;
        let font_regular = doc.add_builtin_font(BuiltinFont::Helvetica)?;
        let font_mono = doc.add_builtin_font(BuiltinFont::Courier)?;

        // Header
        current_layer.use_text("SECURE DATA WIPE CERTIFICATE", 20, Mm(20.0), Mm(270.0), &font_bold);
        current_layer.use_text("Tamper-Proof Digital Certificate", 14, Mm(20.0), Mm(260.0), &font_regular);

        // Certificate details section
        let mut y_pos = 240.0;
        
        // Certificate ID
        current_layer.use_text("Certificate ID:", 12, Mm(20.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&certificate.certificate_id, 10, Mm(60.0), Mm(y_pos), &font_mono);
        y_pos -= 10.0;

        // Issue date
        current_layer.use_text("Issued:", 12, Mm(20.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&certificate.created_at.format("%Y-%m-%d %H:%M:%S UTC").to_string(), 10, Mm(60.0), Mm(y_pos), &font_regular);
        y_pos -= 15.0;

        // Device Information Section
        current_layer.use_text("DEVICE INFORMATION", 14, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 10.0;

        current_layer.use_text("Device:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&certificate.device_info.name, 10, Mm(60.0), Mm(y_pos), &font_regular);
        y_pos -= 8.0;

        if let Some(model) = &certificate.device_info.model {
            current_layer.use_text("Model:", 11, Mm(25.0), Mm(y_pos), &font_bold);
            current_layer.use_text(model, 10, Mm(60.0), Mm(y_pos), &font_regular);
            y_pos -= 8.0;
        }

        if let Some(serial) = &certificate.device_info.serial_number {
            current_layer.use_text("Serial Number:", 11, Mm(25.0), Mm(y_pos), &font_bold);
            current_layer.use_text(serial, 10, Mm(60.0), Mm(y_pos), &font_mono);
            y_pos -= 8.0;
        }

        current_layer.use_text("Capacity:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&certificate.device_info.capacity_human, 10, Mm(60.0), Mm(y_pos), &font_regular);
        y_pos -= 8.0;

        current_layer.use_text("Type:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&certificate.device_info.device_type, 10, Mm(60.0), Mm(y_pos), &font_regular);
        y_pos -= 15.0;

        // Wipe Details Section
        current_layer.use_text("WIPE OPERATION DETAILS", 14, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 10.0;

        current_layer.use_text("Method Used:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&certificate.wipe_details.method_used, 10, Mm(70.0), Mm(y_pos), &font_regular);
        y_pos -= 8.0;

        current_layer.use_text("Total Passes:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&certificate.wipe_details.total_passes.to_string(), 10, Mm(70.0), Mm(y_pos), &font_regular);
        y_pos -= 8.0;

        current_layer.use_text("Data Processed:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&format_bytes(certificate.wipe_details.bytes_processed), 10, Mm(70.0), Mm(y_pos), &font_regular);
        y_pos -= 8.0;

        current_layer.use_text("Duration:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        current_layer.use_text(&format_duration(certificate.wipe_details.duration_seconds), 10, Mm(70.0), Mm(y_pos), &font_regular);
        y_pos -= 8.0;

        current_layer.use_text("Verification:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        let verification_text = if certificate.wipe_details.verification_passed { "PASSED ✓" } else { "FAILED ✗" };
        current_layer.use_text(verification_text, 10, Mm(70.0), Mm(y_pos), &font_bold);
        y_pos -= 15.0;

        // Standards Compliance
        current_layer.use_text("STANDARDS COMPLIANCE", 14, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 10.0;

        for standard in &certificate.wipe_details.standards_compliance {
            current_layer.use_text("•", 11, Mm(25.0), Mm(y_pos), &font_bold);
            current_layer.use_text(standard, 10, Mm(30.0), Mm(y_pos), &font_regular);
            y_pos -= 8.0;
        }
        y_pos -= 5.0;

        // Verification Section
        current_layer.use_text("CERTIFICATE VERIFICATION", 14, Mm(20.0), Mm(y_pos), &font_bold);
        y_pos -= 10.0;

        current_layer.use_text("Verification Hash:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        y_pos -= 5.0;
        current_layer.use_text(&certificate.verification_data.verification_hash, 8, Mm(25.0), Mm(y_pos), &font_mono);
        y_pos -= 10.0;

        current_layer.use_text("Tamper-Proof Seal:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        y_pos -= 5.0;
        current_layer.use_text(&certificate.verification_data.tamper_proof_seal, 8, Mm(25.0), Mm(y_pos), &font_mono);
        y_pos -= 15.0;

        current_layer.use_text("Verification URL:", 11, Mm(25.0), Mm(y_pos), &font_bold);
        y_pos -= 5.0;
        current_layer.use_text(&certificate.verification_data.verification_url, 9, Mm(25.0), Mm(y_pos), &font_regular);
        y_pos -= 15.0;

        // QR Code (placeholder - would need image embedding in real implementation)
        current_layer.use_text("QR CODE FOR MOBILE VERIFICATION", 12, Mm(130.0), Mm(100.0), &font_bold);
        
        // Footer
        current_layer.use_text("This certificate was generated by SecureWipe Data Erasure Tool", 9, Mm(20.0), Mm(20.0), &font_regular);
        current_layer.use_text("Certificate authenticity can be verified at the URL above", 9, Mm(20.0), Mm(15.0), &font_regular);

        // Save PDF
        Ok(doc.save_to_bytes()?)
    }

    pub fn generate_json_metadata(&self, certificate: &WipeCertificate) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(certificate)?)
    }

    fn create_certificate_data(&self, device_info: &DeviceCertificateInfo, wipe_details: &WipeDetails, timestamp: &DateTime<Utc>) -> Vec<u8> {
        format!(
            "{}:{}:{}:{}:{}:{}",
            device_info.name,
            device_info.serial_number.as_ref().unwrap_or(&"unknown".to_string()),
            wipe_details.method_used,
            wipe_details.bytes_processed,
            wipe_details.verification_passed,
            timestamp.timestamp()
        ).into_bytes()
    }

    fn generate_qr_code(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let code = QrCode::new(url.as_bytes())?;
        let image = code.render::<Rgb<u8>>().build();
        
        // Convert to base64 for storage/embedding
        let mut buffer = Vec::new();
        let dynamic_image = DynamicImage::ImageRgb8(image);
        dynamic_image.write_to(&mut Cursor::new(&mut buffer), image::ImageOutputFormat::Png)?;
        
        Ok(base64::encode(&buffer))
    }

    fn get_compliance_standards(&self, method: &WipeMethod) -> Vec<String> {
        match method {
            WipeMethod::DoD522022M => vec![
                "DoD 5220.22-M Standard".to_string(),
                "US Department of Defense Approved".to_string(),
            ],
            WipeMethod::NIST80088 => vec![
                "NIST SP 800-88 Rev. 1".to_string(),
                "US National Institute of Standards".to_string(),
            ],
            WipeMethod::Gutmann => vec![
                "Gutmann Method".to_string(),
                "35-Pass Secure Deletion".to_string(),
            ],
            _ => vec![
                "Industry Standard Secure Deletion".to_string(),
            ],
        }
    }

    fn get_pattern_descriptions(&self, method: &WipeMethod) -> Vec<String> {
        match method {
            WipeMethod::SinglePass => vec!["Zero Pattern (0x00)".to_string()],
            WipeMethod::DoD522022M => vec![
                "Pass 1: 0x01 Pattern".to_string(),
                "Pass 2: 0xFE Pattern".to_string(),
                "Pass 3: Random Data".to_string(),
            ],
            WipeMethod::NIST80088 => vec!["Random Data Pattern".to_string()],
            WipeMethod::Random3Pass => vec![
                "Pass 1: Random Data".to_string(),
                "Pass 2: Random Data".to_string(),
                "Pass 3: Random Data".to_string(),
            ],
            _ => vec!["Custom Secure Pattern".to_string()],
        }
    }
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

fn format_duration(seconds: u64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}
