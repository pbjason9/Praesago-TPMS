use std::path::PathBuf;
use clap::Parser;

use tmps_core::{
    ModelInfo,
    package_model_file,
    write_manifest_to_yaml,
};

/// Praesago TMPS – Trusted Model Packaging & Signing Toolkit (v1)
#[derive(Parser)]
#[command(name = "tmps", version = "0.1.0")]
enum Command {
    /// Encrypt a model file and produce a manifest
    Package {
        /// Path to the model file (e.g., model.onnx)
        #[arg(long)]
        model: PathBuf,

        /// Output directory (e.g., ./out)
        #[arg(long)]
        output_dir: PathBuf,

        /// Unique model ID for the manifest
        #[arg(long)]
        model_id: String,

        /// Human-readable model name
        #[arg(long)]
        name: String,

        /// Model version
        #[arg(long)]
        version: String,

        /// Model format (onnx, pt, h5, etc.)
        #[arg(long, default_value = "onnx")]
        format: String,

        /// Reference name for the key used
        #[arg(long, default_value = "model-key-001")]
        key_ref: String,

        /// 64 hex characters = 32 bytes AES-256 key
        #[arg(long)]
        key_hex: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cmd = Command::parse();

    match cmd {
        Command::Package {
            model,
            output_dir,
            model_id,
            name,
            version,
            format,
            key_ref,
            key_hex,
        } => {
            let key_bytes = hex::decode(key_hex)?;
            if key_bytes.len() != 32 {
                anyhow::bail!("AES-256 key must be exactly 32 bytes (64 hex chars)");
            }

            let model_info = ModelInfo {
                id: model_id,
                name,
                version,
                format,
                original_filename: model
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string(),
            };

            let manifest = package_model_file(
                &key_bytes,
                &model,
                &output_dir,
                model_info,
                &key_ref,
            )?;

            let manifest_path = output_dir.join("model_package.yaml");
            write_manifest_to_yaml(&manifest, &manifest_path)?;

            println!("Model encrypted successfully.");
            println!(" Output ciphertext: {}", output_dir.join("model.enc").display());
            println!(" Manifest: {}", manifest_path.display());
        }
    }

    Ok(())
}