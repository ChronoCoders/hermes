# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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

[1.1.0]: https://github.com/ChronoCoders/hermes/releases/tag/v1.1.0
[1.0.1]: https://github.com/ChronoCoders/hermes/releases/tag/v1.0.1
[1.0.0]: https://github.com/ChronoCoders/hermes/releases/tag/v1.0.0
[0.1.0]: https://github.com/ChronoCoders/hermes/releases/tag/v0.1.0