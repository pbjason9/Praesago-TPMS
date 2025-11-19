use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub format: String,
    pub original_filename: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptionInfo {
    pub backend: String,
    pub algorithm: String,
    pub key_ref: String,
    pub ciphertext_file: String,
    pub iv_hex: String,
    pub tag_hex: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrityInfo {
    pub plaintext_sha256: String,
    pub ciphertext_sha256: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PackagingInfo {
    pub schema_version: String,
    pub tool: String,
    pub tool_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelPackageManifest {
    pub model: ModelInfo,
    pub encryption: EncryptionInfo,
    pub integrity: IntegrityInfo,
    pub packaging: PackagingInfo,
}
