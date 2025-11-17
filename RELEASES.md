# Hermes Release Drafts

## Release v2.4.0 - Web User Interface

### What's New

Hermes now includes a full-featured web interface, making it accessible to users who prefer graphical interfaces over command-line tools.

### Highlights

- **Browser-Based Interface** - Access all Hermes features through your web browser at `http://localhost:8080`
- **Real-Time Dashboard** - Monitor system status, key counts, and configuration at a glance
- **Key Management UI** - Generate, rotate, and manage keys without touching the terminal
- **Drag-and-Drop Encryption** - Encrypt and decrypt files with simple drag-and-drop
- **Digital Signature Tools** - Sign and verify data using post-quantum Dilithium-5
- **Steganography Analysis** - Check image capacity for hidden data

### New Commands

```bash
# Start web UI on default port (8080)
hermes web-ui

# Start on custom port
hermes web-ui --port 3000
```

### REST API

13 new endpoints for programmatic access:
- System status and configuration
- Key generation, rotation, and listing
- Message and file encryption/decryption
- Digital signatures
- Steganography analysis

### Technical Details

- **Backend**: Axum 0.7 async web framework
- **Runtime**: Tokio full-featured async runtime
- **Frontend**: Embedded single-page application (no external dependencies)
- **Security**: CORS support, 100MB max file uploads
- **Theme**: Military-grade dark aesthetic

### Installation

```bash
cargo build --release
./target/release/hermes web-ui
```

Then open http://localhost:8080 in your browser.

---

## Release v2.3.0 - Steganography Support

### What's New

Hide your encrypted data in plain sight using LSB (Least Significant Bit) steganography. Embed secret messages and files within innocent-looking PNG images.

### Highlights

- **Covert Communication** - Hide encrypted data inside normal images
- **Imperceptible Changes** - Modifications are invisible to the human eye
- **Capacity Analysis** - Calculate how much data an image can hide
- **Integrity Verification** - Magic header ensures data validity

### New Commands

```bash
# Hide secret file in image
hermes stego-hide secret.txt --cover photo.png --output innocent.png -p password

# Extract hidden file
hermes stego-reveal innocent.png --output recovered.txt -p password

# Check capacity
hermes stego-capacity photo.png --analyze
```

### Technical Details

- Uses RGB channel LSB modification (alpha preserved)
- Magic header: `HRMSSTEG` (8 bytes)
- 4-byte length prefix for data validation
- Capacity: ~37.5% of total pixels in bytes
- Supports both password and recipient-based encryption

### Use Cases

- Covert communication channels
- Plausible deniability for sensitive data
- Bypassing content inspection
- Secure dead drops

---

## Release v2.2.0 - Key Rotation Mechanism

### What's New

Professional key lifecycle management with secure rotation and archiving. Maintain cryptographic hygiene by regularly rotating your keys while preserving the ability to decrypt old messages.

### Highlights

- **Automated Key Rotation** - Generate new keys with a single command
- **Secure Archiving** - Old keys preserved with timestamps
- **Multi-Algorithm Support** - Rotate RSA, Kyber, and Dilithium keys together
- **Audit Trail** - Rotation metadata for compliance requirements
- **Backward Compatibility** - Old encrypted files remain accessible

### New Commands

```bash
# Rotate key with archiving
hermes key-rotate alice --archive

# Rotate all key types
hermes key-rotate alice --archive --pqc --sign

# List archived keys
hermes list-archived-keys
```

### Security Features

- Keys archived to `~/.hermes/keys/archive/`
- Timestamped filenames: `keyname_YYYYMMDD_HHMMSS.pem`
- Rotation metadata tracks fingerprints and timestamps
- Automatic directory creation for archive storage

### Best Practices

- Rotate keys every 90 days for high-security environments
- Always use `--archive` to preserve decryption capability
- Distribute new public keys to recipients immediately
- Verify new fingerprints before encrypting sensitive data

---

## Release v2.1.0 - Dilithium Digital Signatures

### What's New

Post-quantum digital signatures using CRYSTALS-Dilithium, a NIST-selected algorithm for the post-quantum cryptography standard. Ensure authenticity and integrity of your messages with signatures that will remain secure even against quantum computers.

### Highlights

- **Quantum-Resistant Signatures** - NIST PQC Level 5 security
- **Authenticity Verification** - Prove message origin
- **Tamper Detection** - Any modification invalidates signature
- **Future-Proof** - Secure against quantum computing threats

### New Commands

```bash
# Generate signing keypair
hermes keygen alice --sign

# Sign a file
hermes sign-file document.pdf --key alice --output document.sig

# Verify signature
hermes verify-signature document.sig --signer alice --output verified.pdf
```

### Technical Specifications

- **Algorithm**: CRYSTALS-Dilithium (Dilithium5)
- **Security Level**: NIST Level 5 (256-bit classical, 128-bit quantum)
- **Public Key Size**: 2,592 bytes
- **Secret Key Size**: 4,864 bytes
- **Signature Size**: 4,595 bytes
- **Fingerprinting**: SHA-256 hash of public key

### Integration

- Seamless integration with existing key management
- Keys stored alongside RSA and Kyber keys
- Compatible with key rotation system
- Fingerprint tracking for verification

---

## Release v2.0.0 - Post-Quantum Cryptography

### What's New

The future of encryption is here. Hermes v2.0.0 introduces hybrid post-quantum cryptography using CRYSTALS-Kyber, protecting your data against both current and future quantum computer attacks.

### Highlights

- **Quantum-Safe Encryption** - Kyber-1024 (NIST Level 5)
- **Hybrid Approach** - RSA-4096 + Kyber for defense in depth
- **Multi-Recipient PQC** - Share quantum-safe encrypted data
- **Key Management** - Generate, import, and export PQC keys
- **Backward Compatible** - Still supports traditional encryption

### New Commands

```bash
# Generate hybrid keypair
hermes keygen alice --pqc

# Send file with PQC encryption
hermes send-file secret.pdf --recipients bob,charlie --pqc

# Import recipient's Kyber key
hermes import-kyber-pubkey bob /path/to/bob_kyber.pub

# Export your Kyber key
hermes export-kyber-pubkey alice --output alice_kyber.pub
```

### Technical Specifications

- **Classical**: RSA-4096 (traditional security)
- **Post-Quantum**: Kyber-1024 (quantum resistance)
- **Key Encapsulation**: Hybrid scheme combines both
- **Package Format**: Version 0x02 with PQC flag
- **Fingerprinting**: SHA-256 for all key types

### Why Hybrid?

The hybrid approach provides:
1. **Current Security**: RSA-4096 proven against classical attacks
2. **Future Security**: Kyber protects against quantum threats
3. **Conservative Design**: If one algorithm fails, the other protects
4. **NIST Recommendation**: Follows post-quantum migration guidance

### Migration Guide

Existing Hermes users can upgrade seamlessly:
1. Old encrypted files still decrypt normally
2. Generate new PQC-enabled keys with `--pqc` flag
3. Share Kyber public keys with recipients
4. Enable `--pqc` flag when sending to PQC-enabled recipients

### Breaking Changes

- Package format version bumped to 0x02
- New flag byte in encrypted packages
- Recipients need Kyber keys for PQC mode

---

## Upgrade Path

| From | To | Action Required |
|------|-----|-----------------|
| 1.x | 2.0 | Generate PQC keys, distribute to recipients |
| 2.0 | 2.1 | Generate signing keys if needed |
| 2.1 | 2.2 | No action required |
| 2.2 | 2.3 | No action required |
| 2.3 | 2.4 | No action required |

All versions maintain backward compatibility for decryption.
