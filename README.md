# 🔐 HERMES - Military-Grade Secure File Transfer System

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Security](https://img.shields.io/badge/security-military--grade-brightgreen.svg)]()

A command-line secure file transfer system with military-grade encryption, featuring hybrid RSA+AES encryption, self-destruct timers, and multi-recipient support.

## 🎯 Features

### 🔒 Core Security
- **Hybrid Encryption**: RSA-4096 + AES-256-GCM
- **Key Derivation**: Argon2 (memory-hard, resistant to GPU attacks)
- **Integrity Verification**: SHA-256 checksums
- **Custom Binary Protocol**: Efficient, compact file format

### 👥 Multi-Recipient Support
- **RSA Public Key Encryption**: Send to multiple recipients
- **Individual Key Management**: Each recipient uses their own private key
- **Key Fingerprinting**: Verify recipient identities
- **Backward Compatible**: Works with password-based encryption

### ⏱️ Advanced Features
- **Self-Destruct Timer**: Automatic expiration (TTL-based)
- **Compression**: GZIP compression for files >1KB
- **SFTP Integration**: Secure remote storage
- **Custom Remote Paths**: Organize encrypted files

### 🎨 User Experience
- Beautiful CLI interface with colored output
- Progress indicators for large operations
- Detailed status messages
- Cross-platform support (Windows, Linux, macOS)

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/hermes.git
cd hermes

# Build release version
cargo build --release

# Install (optional)
cargo install --path .
```

## 🚀 Quick Start

### 1. Initialize Configuration

```bash
hermes init
```

Edit `~/.config/hermes/config.toml` with your SFTP credentials.

### 2. Password-Based Encryption (Simple)

```bash
# Encrypt and send a message
hermes send-msg "Secret message" -p MySecurePassword123

# Encrypt and send a file
hermes send-file document.pdf -p MySecurePassword123

# Decrypt a message
hermes recv-msg msg_20250125_120000.enc -p MySecurePassword123

# Decrypt a file
hermes recv-file document_20250125_120000.enc -p MySecurePassword123
```

### 3. Multi-Recipient Encryption (Advanced)

```bash
# Generate your RSA keypair
hermes keygen alice

# Export your public key to share
hermes export-pubkey alice -o alice_public.pem

# Import recipient's public key
hermes import-pubkey bob bob_public.pem

# Send to multiple recipients
hermes send-msg "Top secret" --recipients alice,bob,charlie
hermes send-file document.pdf --recipients alice,bob

# Decrypt with your private key
hermes recv-msg msg_20250125_120000.enc --recipient alice
hermes recv-file document_20250125_120000.enc --recipient alice
```

### 4. Self-Destruct Timer

```bash
# Message expires in 24 hours
hermes send-msg "This will self-destruct" -p Pass123 -t 24

# File expires in 48 hours
hermes send-file secret.txt -p Pass123 -t 48

# Multi-recipient with expiry
hermes send-file classified.pdf --recipients alice,bob -t 72
```

## 📖 Commands

### Configuration & Setup

| Command | Description |
|---------|-------------|
| `hermes init` | Initialize Hermes configuration |
| `hermes config` | Display current configuration |
| `hermes list` | List all encrypted files with status |

### Key Management

| Command | Description |
|---------|-------------|
| `hermes keygen <n>` | Generate RSA-4096 keypair |
| `hermes export-pubkey <n> -o <file>` | Export public key |
| `hermes import-pubkey <n> <file>` | Import recipient's public key |
| `hermes list-keys` | List all keys and recipients |

### Encryption & Decryption

**Messages:**
```bash
# Password-based
hermes send-msg <message> -p <password> [-t <hours>]
hermes recv-msg <file> -p <password>

# Multi-recipient
hermes send-msg <message> --recipients <name1,name2> [-t <hours>]
hermes recv-msg <file> --recipient <n>
```

**Files:**
```bash
# Password-based
hermes send-file <path> -p <password> [-t <hours>]
hermes recv-file <file> -p <password> [-o <output>]

# Multi-recipient
hermes send-file <path> --recipients <name1,name2> [-t <hours>]
hermes recv-file <file> --recipient <n> [-o <output>]
```

## 🔐 Security Details

### Encryption Algorithm
- **Symmetric**: AES-256-GCM (Galois/Counter Mode)
- **Asymmetric**: RSA-4096 with PKCS#1 v1.5 padding
- **Key Derivation**: Argon2id (default parameters)
- **Random Generation**: OS-provided CSPRNG

### Binary Protocol Format

```
[Magic: 4 bytes] "HRMS"
[Version: 1 byte] 0x01
[Flags: 1 byte] Compressed, Multi-recipient
[Salt Length: 2 bytes]
[Salt: variable]
[Nonce: 12 bytes]
[Checksum: 32 bytes] SHA-256
[Original Size: 8 bytes]
[Expires At: 8 bytes] Unix timestamp
[Filename Length: 2 bytes]
[Filename: variable]
[Recipient Count: 2 bytes]
  For each recipient:
    [Name Length: 2 bytes]
    [Name: variable]
    [Encrypted Key Length: 2 bytes]
    [Encrypted Key: ~512 bytes]
[Ciphertext Length: 4 bytes]
[Ciphertext: variable]
```

## 📝 Changelog

### v1.0.0 (2025-01-25)
- ✨ Multi-recipient RSA+AES hybrid encryption
- ✨ Self-destruct timer (TTL-based expiration)
- ✨ Custom binary protocol
- ✨ RSA key management
- ✨ GZIP compression
- ✨ SHA-256 integrity verification
- ✨ Backward compatibility with password-based encryption

## 📄 License

This project is licensed under the MIT License.

## ⚠️ Disclaimer

This software is provided for educational and legitimate security purposes only.

## 📧 Contact

- Author: Altug Tatlisu
- Email: contact@chronocoder.dev

---

**⚡ Built with Rust 🦀 | Secured by Mathematics 🔢 | Protected by Design 🛡️**
