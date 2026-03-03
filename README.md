# Hermes

A command-line secure file transfer system with hybrid RSA+AES encryption, self-destruct timers, and multi-recipient support.

## Features

### Core Security
- Hybrid encryption: RSA-4096 + AES-256-GCM
- Key derivation: Argon2id (memory-hard, resistant to GPU attacks)
- Integrity verification: SHA-256 checksums
- Custom binary protocol: efficient, compact file format

### Multi-Recipient Support
- RSA public key encryption for multiple recipients
- Individual key management per recipient
- Key fingerprinting for identity verification
- Backward compatible with password-based encryption

### Advanced Features
- Self-destruct timer: automatic expiration (TTL-based)
- GZIP compression for files larger than 1KB
- SFTP integration for secure remote storage
- Custom remote paths for file organization

## Installation

```bash
git clone https://github.com/ChronoCoders/hermes.git
cd hermes
cargo build --release
```

## Quick Start

### Initialize Configuration

```bash
hermes init
```

Edit `~/.config/hermes/config.toml` with your SFTP credentials.

### Password-Based Encryption

```bash
hermes send-msg "Secret message" -p MySecurePassword123
hermes send-file document.pdf -p MySecurePassword123
hermes recv-msg msg_20250125_120000.enc -p MySecurePassword123
hermes recv-file document_20250125_120000.enc -p MySecurePassword123
```

### Multi-Recipient Encryption

```bash
hermes keygen alice
hermes export-pubkey alice -o alice_public.pem
hermes import-pubkey bob bob_public.pem
hermes send-msg "Top secret" --recipients alice,bob,charlie
hermes send-file document.pdf --recipients alice,bob
hermes recv-msg msg_20250125_120000.enc --recipient alice
hermes recv-file document_20250125_120000.enc --recipient alice
```

### Self-Destruct Timer

```bash
hermes send-msg "This will self-destruct" -p Pass123 -t 24
hermes send-file secret.txt -p Pass123 -t 48
hermes send-file classified.pdf --recipients alice,bob -t 72
```

## Commands

### Configuration and Setup

| Command | Description |
|---------|-------------|
| `hermes init` | Initialize Hermes configuration |
| `hermes config` | Display current configuration |
| `hermes list` | List all encrypted files with status |

### Key Management

| Command | Description |
|---------|-------------|
| `hermes keygen <name>` | Generate RSA-4096 keypair |
| `hermes export-pubkey <name> -o <file>` | Export public key |
| `hermes import-pubkey <name> <file>` | Import recipient public key |
| `hermes list-keys` | List all keys and recipients |

### Encryption and Decryption

Messages:

```bash
hermes send-msg <message> -p <password> [-t <hours>]
hermes recv-msg <file> -p <password>
hermes send-msg <message> --recipients <name1,name2> [-t <hours>]
hermes recv-msg <file> --recipient <name>
```

Files:

```bash
hermes send-file <path> -p <password> [-t <hours>]
hermes recv-file <file> -p <password> [-o <output>]
hermes send-file <path> --recipients <name1,name2> [-t <hours>]
hermes recv-file <file> --recipient <name> [-o <output>]
```

## Security Details

### Encryption

- Symmetric: AES-256-GCM
- Asymmetric: RSA-4096 with PKCS#1 v1.5 padding
- Key derivation: Argon2id
- Random generation: OS-provided CSPRNG

### Binary Protocol Format

```
[Magic: 4 bytes]           "HRMS"
[Version: 1 byte]          0x01
[Flags: 1 byte]            Compressed, Multi-recipient
[Salt Length: 2 bytes]
[Salt: variable]
[Nonce: 12 bytes]
[Checksum: 32 bytes]       SHA-256
[Original Size: 8 bytes]
[Expires At: 8 bytes]      Unix timestamp
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

## License

MIT License. See LICENSE for details.

## Disclaimer

This software is provided for educational and legitimate security purposes only.

## Contact

Altug Tatlisu — contact@chronocoder.dev