# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2025-01-27

### Added
- **File Chunking & Streaming** for large files
  - `hermes send-file-chunked` - Split and encrypt large files (50MB chunks)
  - `hermes recv-file-chunked` - Download and reassemble chunked files
  - Memory-efficient streaming (handles multi-GB files)
  - Automatic chunk integrity verification (SHA-256)
  - Manifest-based reassembly system
  - Progress tracking per chunk
  - Support for both password and multi-recipient encryption

### Technical Details
- Chunk size: 50MB (configurable)
- Hash verification for each chunk and final file
- Automatic cleanup of temporary files
- JSON manifest with chunk metadata
- Compatible with existing encryption system

## [1.1.0] - 2025-01-26

### Added
- Shell completion support
- Config validation
- Progress bars and spinners
- Batch operations (send-batch, send-dir, recv-batch)
- Interactive mode (TUI)

### Changed
- Enhanced CLI with better flag organization
- Improved error messages
- Better user feedback

### Dependencies
- Added `clap_complete` for shell completions
- Added `dialoguer` for interactive TUI mode

## [1.0.1] - 2025-01-25

### Fixed
- Fixed rand_core import issues
- Fixed argon2 SaltString generation
- Fixed OsRng usage for RSA operations
- Resolved all clippy warnings

## [1.0.0] - 2025-01-25

### Added
- Multi-recipient RSA+AES hybrid encryption
- Self-destruct timer (TTL-based expiration)
- Custom binary protocol
- RSA key management commands
- GZIP compression
- SHA-256 integrity verification
- Enhanced UI

### Security
- RSA-4096 for asymmetric encryption
- AES-256-GCM with authenticated encryption
- Argon2 key derivation
- SHA-256 integrity checksums

[1.2.0]: https://github.com/ChronoCoders/hermes/releases/tag/v1.2.0
[1.1.0]: https://github.com/ChronoCoders/hermes/releases/tag/v1.1.0
[1.0.1]: https://github.com/ChronoCoders/hermes/releases/tag/v1.0.1
[1.0.0]: https://github.com/ChronoCoders/hermes/releases/tag/v1.0.0
