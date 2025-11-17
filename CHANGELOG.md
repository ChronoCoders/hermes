# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.3.0] - 2025-11-17

### Added
- 🖼️ **Steganography Support**
  - LSB (Least Significant Bit) steganography for hiding data in PNG images
  - `hermes stego-hide` - Embed encrypted files into cover images
  - `hermes stego-reveal` - Extract hidden data from steganographic images
  - `hermes stego-capacity` - Check image capacity for hidden data
  - Support for both password and recipient-based encryption
  - Magic header verification (`HRMSSTEG`) for data integrity
  - Analysis tools for detecting steganographic content
  - Image capacity calculation (approximately 37.5% of pixel count in bytes)

### Technical Details
- Uses RGB channel LSB modification (alpha channel preserved)
- 4-byte length prefix for data size validation
- Imperceptible to human eye

### Usage
```bash
# Hide secret file in image
hermes stego-hide secret.txt --cover photo.png --output innocent.png -p password

# Extract hidden file
hermes stego-reveal innocent.png --output recovered.txt -p password

# Check capacity
hermes stego-capacity photo.png --analyze
```

## [2.2.0] - 2025-11-17

### Added
- 🔄 **Key Rotation Mechanism**
  - `hermes key-rotate` - Securely rotate cryptographic keys
  - `hermes list-archived-keys` - View history of rotated keys
  - Automatic archiving with timestamped backups
  - Support for rotating RSA, Kyber (PQC), and Dilithium keys
  - Rotation metadata tracking for audit purposes
  - Backward compatibility preserved for old encrypted files

### Security Features
- Archive directory at `~/.hermes/keys/archive/`
- Timestamp-based backup naming (e.g., `alice_20251117_143022.pem`)
- Rotation metadata with fingerprint tracking
- Warning to distribute new public keys after rotation

### Usage
```bash
# Rotate all keys with archiving
hermes key-rotate alice --archive --pqc --sign

# View archived keys
hermes list-archived-keys
```

## [2.1.0] - 2025-11-17

### Added
- 🔏 **Post-Quantum Digital Signatures**
  - Dilithium-5 (NIST PQC Level 5 security)
  - `hermes sign-file` - Create quantum-resistant signatures
  - `hermes verify-signature` - Verify and extract signed files
  - `hermes keygen --sign` - Generate Dilithium signing keypairs
  - SHA-256 fingerprinting for Dilithium keys
  - PEM-formatted key storage

### Cryptographic Properties
- Algorithm: Dilithium-5 (CRYSTALS-Dilithium)
- Security level: NIST Level 5 (256-bit quantum security)
- Public key size: ~2.5 KB
- Signature size: ~4.5 KB

### Usage
```bash
# Generate signing keypair
hermes keygen alice --sign

# Sign a file
hermes sign-file document.pdf --key alice

# Verify and extract
hermes verify-signature document.pdf.sig --signer alice --output verified.pdf
```

## [2.0.0] - 2025-11-17

### Added
- 🛡️ **Post-Quantum Cryptography (Hybrid Encryption)**
  - Kyber-1024 (ML-KEM) post-quantum key encapsulation
  - Hybrid RSA-4096 + Kyber-1024 encryption
  - `hermes keygen --pqc` - Generate Kyber keypairs
  - `hermes send-file --pqc` - Use hybrid encryption
  - `hermes import-kyber-pubkey` - Import recipient's Kyber key
  - `hermes export-kyber-pubkey` - Export your Kyber key
  - Package format version 0x02 with PQC flag
  - Backward compatibility with v1.x packages

### Security Architecture
- **Defense in Depth**: Both RSA and Kyber keys must match for decryption
- **Future-Proof**: Protected against quantum computer attacks
- **Interoperable**: Old clients can still decrypt v1.x packages

### Technical Details
- Primary: RSA-4096 with OAEP padding
- Post-Quantum: Kyber-1024 (ML-KEM) KEM
- Key Encapsulation: XOR-based with Kyber shared secret
- Package flags: `FLAG_PQC_ENABLED = 0b00000100`

### Migration Guide
```bash
# Generate PQC-enabled keypair
hermes keygen alice --pqc

# Export Kyber public key
hermes export-kyber-pubkey alice --output alice_kyber.pub

# Import recipient's Kyber key
hermes import-kyber-pubkey bob bob_kyber.pub

# Send with hybrid encryption
hermes send-file sensitive.zip --recipients bob --pqc
```

### Breaking Changes
- Package format version bumped to 0x02
- New flag bit for PQC-enabled packages
- Old v1.x clients cannot decrypt v2.x PQC packages (but can decrypt non-PQC v2.x packages)

## [1.3.1] - 2025-11-17

### Added
- 🔑 **Shamir's Secret Sharing**
  - `hermes key-split` - Split private keys into multiple shares
  - `hermes key-recover` - Reconstruct keys from threshold shares
  - `hermes share-verify` - Verify share file integrity
  - Threshold cryptography (k-of-n scheme)
  - SHA-256 checksums for each share
  - JSON-formatted share files with metadata
  - GF(256) field arithmetic for cryptographic security

### Use Cases
- Distributed key custody
- Emergency key recovery
- Multi-party authorization
- Secure key backup

### Usage
```bash
# Split key into 5 shares, needing 3 to recover
hermes key-split alice -t 3 -n 5

# Recover with any 3 shares
hermes key-recover share1.json share2.json share3.json -n alice

# Verify a share
hermes share-verify alice_share_1.json
```

## [1.1.0] - 2025-01-26

### Added
- 🎯 **Shell completion support**
  - Generate completion scripts for bash, zsh, fish, powershell, elvish
  - Command: `hermes completion <shell>`
  - Easy installation with auto-generated instructions
  
- ✅ **Config validation**
  - Validate configuration files
  - Test SFTP connection
  - Commands: `hermes validate`, `hermes config --validate --test`
  - Comprehensive validation with helpful error messages
  
- 📊 **Progress bars and spinners**
  - Progress bars for file upload/download (files >1MB)
  - Progress bars for encryption/decryption operations
  - Animated spinners for RSA key generation
  - Real-time feedback for long-running operations
  - Visual indicators for file read/write operations

- 📦 **Batch operations**
  - `hermes send-batch` - Encrypt multiple files at once
  - `hermes send-dir` - Encrypt entire directories (recursive support)
  - `hermes recv-batch` - Decrypt multiple files in one command
  - Progress tracking per file
  - Error handling with success/failure summary
  - Support for both password and multi-recipient encryption

- 🎮 **Interactive mode (TUI)**
  - `hermes interactive` - Launch interactive menu-driven interface
  - Beautiful menu system with dialoguer
  - Guided wizards for all operations:
    - Send/receive messages
    - Send/receive files
    - Batch operations
    - Key management
    - Configuration
  - No command-line arguments needed
  - Perfect for beginners and GUI users

### Changed
- Enhanced CLI with better flag organization
- Improved error messages for configuration issues
- Better user feedback during validation
- File operations now show progress for large files
- Main menu reorganized for better UX

### Dependencies
- Added `clap_complete` for shell completions
- Added `dialoguer` for interactive TUI mode

## [1.0.1] - 2025-01-25

### Fixed
- Fixed rand_core import issues
- Fixed argon2 SaltString generation
- Fixed OsRng usage for RSA operations
- Added RngCore trait import
- Resolved all clippy warnings

### Changed
- Updated to rand_core 0.6 API
- Updated to argon2 0.5 API
- All tests passing

## [1.0.0] - 2025-01-25

### Added
- 🔐 **Multi-recipient RSA+AES hybrid encryption**
  - RSA-4096 keypair generation
  - Public key import/export
  - Multiple recipients per file/message
  - Individual decryption with private keys
  
- ⏱️ **Self-destruct timer (TTL-based expiration)**
  - Set expiration time in hours
  - Automatic expiry detection
  - Visual indicators for expiring files
  
- 📦 **Custom binary protocol**
  - Efficient, compact file format
  - Magic bytes for file validation
  - Version control for future compatibility
  - 30% smaller than JSON format
  
- 🔑 **RSA key management commands**
  - `hermes keygen` - Generate RSA-4096 keypair
  - `hermes export-pubkey` - Export public key
  - `hermes import-pubkey` - Import recipient's public key
  - `hermes list-keys` - List all keys and recipients
  
- 🗜️ **GZIP compression**
  - Automatic compression for files >1KB
  - Smart compression (only if beneficial)
  - Compression ratio display
  
- ✅ **SHA-256 integrity verification**
  - Checksum validation on decrypt
  - Tamper detection
  
- 🔄 **Backward compatibility**
  - Reads old JSON format
  - Seamless migration to binary format
  
- 🎨 **Enhanced UI**
  - Key fingerprint display
  - Expiry status indicators
  - Multi-recipient type badges
  - Improved error messages

### Changed
- Upgraded encryption package format to binary
- Improved command-line argument handling
- Enhanced list command with expiry information

### Security
- Implemented RSA-4096 for asymmetric encryption
- Added Argon2 key derivation
- AES-256-GCM with authenticated encryption
- SHA-256 integrity checksums

## [0.1.0] - 2025-01-24

### Added
- Initial release
- Basic password-based encryption
- AES-256-GCM encryption
- SFTP integration
- Basic CLI interface
- Configuration management
- Message encryption/decryption
- File encryption/decryption

[2.3.0]: https://github.com/ChronoCoders/hermes/releases/tag/v2.3.0
[2.2.0]: https://github.com/ChronoCoders/hermes/releases/tag/v2.2.0
[2.1.0]: https://github.com/ChronoCoders/hermes/releases/tag/v2.1.0
[2.0.0]: https://github.com/ChronoCoders/hermes/releases/tag/v2.0.0
[1.3.1]: https://github.com/ChronoCoders/hermes/releases/tag/v1.3.1
[1.1.0]: https://github.com/ChronoCoders/hermes/releases/tag/v1.1.0
[1.0.1]: https://github.com/ChronoCoders/hermes/releases/tag/v1.0.1
[1.0.0]: https://github.com/ChronoCoders/hermes/releases/tag/v1.0.0
[0.1.0]: https://github.com/ChronoCoders/hermes/releases/tag/v0.1.0