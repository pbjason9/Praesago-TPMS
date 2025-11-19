pub mod crypto;
pub mod manifest;
pub mod package;

// Re-export common types and functions so users of the crate
// can just `use tmps_core::...` instead of deep module paths.

pub use crypto::{
    Aes256GcmCiphertext,
    CryptoError,
    decrypt_aes256_gcm,
    encrypt_aes256_gcm,
    sha256_bytes,
};

pub use manifest::{ModelInfo, ModelPackageManifest};

pub use package::{package_model_file, write_manifest_to_yaml};
