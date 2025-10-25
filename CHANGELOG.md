# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[1.0.0]: https://github.com/yourusername/hermes/releases/tag/v1.0.0
[0.1.0]: https://github.com/yourusername/hermes/releases/tag/v0.1.0
