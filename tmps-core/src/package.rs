use std::fs;
use std::path::{Path, PathBuf};

use chrono::Utc;
use serde_yaml;

use crate::crypto::{encrypt_aes256_gcm, sha256_bytes, CryptoError};
use crate::manifest::{EncryptionInfo, IntegrityInfo, ModelInfo, ModelPackageManifest, PackagingInfo};

/// High-level function:
/// - reads the model file
/// - encrypts it using AES-256-GCM
/// - writes `model.enc`
/// - builds the manifest struct
pub fn package_model_file(
    key: &[u8],
    model_path: &Path,
    output_dir: &Path,
    model_info: ModelInfo,
    key_ref: &str,
) -> Result<ModelPackageManifest, CryptoError> {
    let plaintext = fs::read(model_path)?;

    let enc_result = encrypt_aes256_gcm(key, &plaintext)?;

    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;

    let ciphertext_path: PathBuf = output_dir.join("model.enc");
    fs::write(&ciphertext_path, &enc_result.ciphertext)?;

    let plaintext_sha = sha256_bytes(&plaintext);
    let ciphertext_sha = sha256_bytes(&enc_result.ciphertext);

    let iv_hex = hex::encode(enc_result.iv);
    let tag_hex = hex::encode(enc_result.tag);

    let encryption = EncryptionInfo {
        backend: "openssl-3.0".into(),
        algorithm: "AES-256-GCM".into(),
        key_ref: key_ref.to_string(),
        ciphertext_file: ciphertext_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string(),
        iv_hex,
        tag_hex,
        created_at: Utc::now(),
    };

    let integrity = IntegrityInfo {
        plaintext_sha256: plaintext_sha,
        ciphertext_sha256: ciphertext_sha,
    };

    let packaging = PackagingInfo {
        schema_version: "1.0.0".into(),
        tool: "praesago-tmps".into(),
        tool_version: "0.1.0".into(),
    };

    let manifest = ModelPackageManifest {
        model: model_info,
        encryption,
        integrity,
        packaging,
    };

    Ok(manifest)
}

/// Serialize the manifest to YAML and write it to `output_path`.
pub fn write_manifest_to_yaml(
    manifest: &ModelPackageManifest,
    output_path: &Path,
) -> Result<(), CryptoError> {
    let yaml = serde_yaml::to_string(manifest)
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?; // reuse error type
    fs::write(output_path, yaml)?;
    Ok(())
}