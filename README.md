# Praesago-TPMS

ersion 1.0 — Model Encryption & Packaging

The Praesago Trusted Model Packaging & Signing (TMPS) Toolkit provides a secure, standardized mechanism to package machine learning models for protected distribution.
Version 1 focuses on one core capability:

Encrypt a model file using enterprise-grade cryptography and produce a corresponding YAML manifest that records encryption metadata and integrity information.

This is the foundational step in establishing a trusted AI supply chain, where model artifacts can be securely transported, validated, and later extended with signature, provenance, and attestation mechanisms.

Purpose of Version 1

TMPS v1 establishes the baseline architecture and cryptographic foundation for the full TMPS ecosystem.

Current capabilities:

Encrypt a model file (e.g., .onnx, .pt, .h5) using AES-256-GCM

Generate a ciphertext file (model.enc)

Compute SHA-256 integrity hashes (plaintext and ciphertext)

Generate a YAML manifest (model_package.yaml) containing:

model metadata

encryption parameters

IV and GCM auth tag

integrity hashes

packaging metadata

This delivers a secure, auditable, and verifiable artifact bundle suitable for controlled environments and sensitive workflows.

Cryptographic Foundation

TMPS v1 uses:

Language: Rust

Crypto Library: OpenSSL 3.x (via Rust openssl crate)

Encryption Algorithm: AES-256-GCM

Integrity Algorithm: SHA-256

IV: Random 96-bit (12-byte) nonce

Auth Tag: 128-bit GCM tag

This architecture is designed to evolve into support for:

Ed25519 / ECDSA signing

Certificate-based key hierarchy

TPM-protected keys

PQC algorithms (future update)

Full AI model provenance and attestation

Project Structure
praesago-tmps/
  tmps-core/      # Rust library: crypto & manifest generation
  tmps-cli/       # CLI to package & encrypt models
  docs/           # Architecture, design notes, schema definitions
  scripts/        # Developer setup scripts (Linux/Windows)

Usage (v1)
Encrypt a model file
tmps package \
  --model ./model.onnx \
  --output-dir ./out \
  --model-id my-model-001 \
  --name "Lane Detection" \
  --version "1.0.0" \
  --format "onnx" \
  --key-ref "model-key-001" \
  --key-hex "<32-byte-hex-key>"

Outputs
out/
  model.enc             # AES-256-GCM encrypted model
  model_package.yaml    # Metadata and integrity manifest

Example Manifest
model:
  id: "my-model-001"
  name: "Lane Detection"
  version: "1.0.0"
  format: "onnx"
  original_filename: "model.onnx"

encryption:
  backend: "openssl-3.0"
  algorithm: "AES-256-GCM"
  key_ref: "model-key-001"
  ciphertext_file: "model.enc"
  iv_hex: "b3c7fa21d9ab45ae11223344"
  tag_hex: "a7f1c4e298ff120947aabbccddeeff00"
  created_at: "2025-11-18T10:00:10Z"

integrity:
  plaintext_sha256: "<sha256-hash-of-model.onnx>"
  ciphertext_sha256: "<sha256-hash-of-model.enc>"

packaging:
  schema_version: "1.0.0"
  tool: "praesago-tmps"
  tool_version: "0.1.0"


This manifest forms the basis for downstream verification, signing, and provenance.

Key Management (v1)

In version 1, the user supplies a raw 256-bit AES key as a hex string.

Future versions will support:

TPM 2.0 key protection

Hardware Security Modules (HSMs)

PKCS#11 interfaces

Key derivation policies

Model-specific, dataset-specific, and pipeline-context keys

Roadmap

Upcoming releases will introduce:

Digital Signatures & Chain of Trust

Ed25519 / ECDSA signing

X.509 anchors

Manifest signing

Secure Model Verification

Validation of ciphertext

Authenticity checks against signed manifests

Trusted Model Distribution Containers

Archive formats for bundling encrypted model + manifest

TPM/HSM-Backed Key Management

Hardware-bound private keys

Restricted key policies

AI Dataset & Model Provenance

Dataset hashing

Training metadata capture

Provenance records embedded into the manifest

TMPS v1 provides the groundwork for these capabilities.

Build Instructions
Linux / macOS
cargo build --workspace


Dependencies:

sudo apt-get install libssl-dev pkg-config

Windows

Install OpenSSL 3.x

Set OPENSSL_DIR or update system PATH

Then run:

cargo build --workspace

License: Propreitary

Status

TMPS v1 is the first foundational release.
Its purpose is to establish:
the crypto backend
the packaging/manifest schema
the secure model encryption workflow
the basis for future trust, signing, and provenance layers

This version is production-grade for controlled environments and will expand rapidly in future releases.
