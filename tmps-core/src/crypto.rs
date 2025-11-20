use openssl::symm::{Cipher, Crypter, Mode};
use rand::RngCore;
use sha2::{Digest, Sha256};
use thiserror::Error;

/// Result of an AES-256-GCM encryption operation.
#[derive(Debug)]
pub struct Aes256GcmCiphertext {
    pub ciphertext: Vec<u8>,
    pub iv: Vec<u8>,   // 12-byte GCM IV/nonce
    pub tag: Vec<u8>,  // 16-byte GCM auth tag
}

#[derive(Debug, Error)]
pub enum CryptoError {
    #[error("invalid AES-256 key length: expected 32 bytes, got {0}")]
    InvalidKeyLength(usize),

    #[error("OpenSSL error: {0}")]
    OpenSsl(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Encrypt `plaintext` with AES-256-GCM using the provided 32-byte key.
///
/// - Generates a random 96-bit IV (recommended for GCM)
/// - Returns ciphertext, IV, and 128-bit auth tag
pub fn encrypt_aes256_gcm(
    key: &[u8],
    plaintext: &[u8],
) -> Result<Aes256GcmCiphertext, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength(key.len()));
    }

    let cipher = Cipher::aes_256_gcm();

    // 96-bit (12-byte) IV is standard for GCM
    let mut iv = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut iv);

    let mut crypter = Crypter::new(cipher, Mode::Encrypt, key, Some(&iv))
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;
    crypter.pad(false);

    let mut ciphertext = vec![0u8; plaintext.len() + cipher.block_size()];
    let mut count = crypter
        .update(plaintext, &mut ciphertext)
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;
    count += crypter
        .finalize(&mut ciphertext[count..])
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;
    ciphertext.truncate(count);

    let mut tag = vec![0u8; 16];
    crypter
        .get_tag(&mut tag)
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;

    Ok(Aes256GcmCiphertext { ciphertext, iv, tag })
}

/// Decrypt AES-256-GCM ciphertext with the given key, IV, and tag.
pub fn decrypt_aes256_gcm(
    key: &[u8],
    iv: &[u8],
    tag: &[u8],
    ciphertext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    if key.len() != 32 {
        return Err(CryptoError::InvalidKeyLength(key.len()));
    }

    let cipher = Cipher::aes_256_gcm();

    let mut crypter = Crypter::new(cipher, Mode::Decrypt, key, Some(iv))
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;
    crypter.pad(false);

    // Set the expected authentication tag before finalize
    crypter
        .set_tag(tag)
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;

    let mut plaintext = vec![0u8; ciphertext.len() + cipher.block_size()];
    let mut count = crypter
        .update(ciphertext, &mut plaintext)
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;
    count += crypter
        .finalize(&mut plaintext[count..])
        .map_err(|e| CryptoError::OpenSsl(e.to_string()))?;
    plaintext.truncate(count);

    Ok(plaintext)
}

/// Compute SHA-256 of arbitrary bytes and return hex string.
pub fn sha256_bytes(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}
















































