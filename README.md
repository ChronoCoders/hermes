# Hermes

Hermes is a secure file transfer system engineered for confidential data exchange. It implements hybrid encryption (RSA-4096 + AES-256-GCM), integrity verification, and strict access control mechanisms.

## Capabilities

### Core Security Architecture
*   **Hybrid Encryption**: Combines RSA-4096 for asymmetric key exchange and AES-256-GCM for symmetric content encryption.
*   **Key Derivation**: Utilizes Argon2id for resistance against GPU-based brute-force attacks.
*   **Integrity Verification**: Enforces SHA-256 checksum validation for all transferred payloads.
*   **Protocol**: Custom binary protocol optimized for minimal overhead and strict structure enforcement.

### Access Control & Management
*   **Multi-Recipient Support**: Encrypts unique session keys for multiple recipients using individual RSA public keys.
*   **Identity Verification**: Fingerprint-based verification for public keys.
*   **Expiration Policies**: Time-To-Live (TTL) enforcement for automatic data expiration.
*   **Compression**: Automatic GZIP compression for optimized throughput.

## Installation

### Build from Source

Requirements: Rust 1.70+

```bash
git clone https://github.com/ChronoCoders/hermes.git
cd hermes
cargo build --release
```

The compiled binary will be available at `target/release/hermes`.

## Usage

### Initialization

Initialize the local configuration and storage paths:

```bash
hermes init
```

Configuration is stored in `~/.config/hermes/config.toml`.

### Key Management

Generate a new RSA-4096 keypair:

```bash
hermes keygen <user_identifier>
```

Export a public key for distribution:

```bash
hermes export-pubkey <user_identifier> -o public_key.pem
```

Import a recipient's public key:

```bash
hermes import-pubkey <recipient_identifier> recipient_key.pem
```

### File Transfer

#### Encryption

Send a file to specific recipients:

```bash
hermes send-file confidential.pdf --recipients alice,bob
```

Send a file with a 24-hour expiration policy:

```bash
hermes send-file data.archive --recipients alice -t 24
```

#### Decryption

Decrypt a received file using your private key:

```bash
hermes recv-file encrypted_payload.enc --recipient <your_identifier>
```

## Technical Specifications

### Cryptographic Standards
*   **Symmetric Encryption**: AES-256-GCM
*   **Asymmetric Encryption**: RSA-4096 (PKCS#1 v1.5 padding)
*   **Key Derivation Function**: Argon2id
*   **Random Number Generation**: OS-native CSPRNG

### Protocol Structure

The Hermes binary protocol adheres to the following structure:

| Field | Size | Description |
|-------|------|-------------|
| Magic | 4 bytes | Protocol Identifier ("HRMS") |
| Version | 1 byte | Protocol Version (0x01) |
| Flags | 1 byte | Compression, Multi-recipient indicators |
| Salt | Variable | Argon2id Salt |
| Nonce | 12 bytes | AES-GCM Nonce |
| Checksum | 32 bytes | SHA-256 Hash |
| Metadata | Variable | Filename, Recipient List, Expiry |
| Payload | Variable | Encrypted Content |

## License

This project is licensed under the MIT License.

## Contact

*   **Author**: Altug Tatlisu
*   **Email**: contact@chronocoder.dev
