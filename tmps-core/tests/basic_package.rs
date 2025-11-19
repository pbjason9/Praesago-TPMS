
use std::fs;
use std::path::PathBuf;

use tmps_core::{
    decrypt_aes256_gcm,
    package_model_file,
    write_manifest_to_yaml,
    ModelInfo,
};

#[test]
fn package_model_creates_ciphertext_and_manifest_and_decrypts() {
    // --- arrange test data ---

    // 32-byte AES-256 key (test-only static key)
    let key: [u8; 32] = [0x11; 32];

    // Create a temporary directory under the system temp dir
    let mut out_dir = std::env::temp_dir();
    out_dir.push("tmps_test_basic_package");
    let _ = fs::create_dir_all(&out_dir);

    // Create a dummy model file
    let mut model_path = out_dir.clone();
    model_path.push("test_model.onnx");

    let dummy_model_bytes = b"dummy model contents for tmps test";
    fs::write(&model_path, dummy_model_bytes).expect("write dummy model");

    let model_info = ModelInfo {
        id: "test-model-001".into(),
        name: "Test Model".into(),
        version: "0.0.1-test".into(),
        format: "onnx".into(),
        original_filename: "test_model.onnx".into(),
    };

    // --- act: package the model ---

    let manifest = package_model_file(
        &key,
        &model_path,
        &out_dir,
        model_info,
        "test-key-001",
    )
    .expect("package_model_file should succeed");

    let manifest_path: PathBuf = out_dir.join("model_package.yaml");
    write_manifest_to_yaml(&manifest, &manifest_path)
        .expect("write_manifest_to_yaml should succeed");

    // --- assert: files exist and decryption works ---

    // 1) Encrypted file exists
    let ciphertext_path = out_dir.join(&manifest.encryption.ciphertext_file);
    assert!(
        ciphertext_path.exists(),
        "ciphertext file should exist at {:?}",
        ciphertext_path
    );

    let ciphertext = fs::read(&ciphertext_path).expect("read ciphertext file");

    // 2) Manifest file exists
    assert!(
        manifest_path.exists(),
        "manifest file should exist at {:?}",
        manifest_path
    );

    // 3) We can decrypt using IV + tag from the manifest
    let iv = hex::decode(&manifest.encryption.iv_hex).expect("decode iv_hex");
    let tag = hex::decode(&manifest.encryption.tag_hex).expect("decode tag_hex");

    let decrypted = decrypt_aes256_gcm(&key, &iv, &tag, &ciphertext)
        .expect("decryption should succeed");

    assert_eq!(
        decrypted,
        dummy_model_bytes,
        "decrypted plaintext should match original model bytes"
    );
}