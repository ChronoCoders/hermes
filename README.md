# Hermes

Military-grade secure file transfer CLI with hybrid post-quantum encryption, steganography, threshold secret sharing, and a Dead Man's Switch — written in Rust.

## Features

| Category | Details |
|----------|---------|
| **Encryption** | AES-256-GCM symmetric + RSA-4096 or Kyber-1024 key wrapping |
| **Post-Quantum** | Kyber-1024 KEM + Dilithium-5 digital signatures |
| **Key Derivation** | Argon2id (memory-hard, GPU-resistant) |
| **Integrity** | SHA-256 checksums embedded in every file |
| **Key Recovery** | Shamir's Secret Sharing (threshold k-of-n) |
| **Steganography** | LSB embedding of encrypted payloads in PNG images |
| **Dead Man's Switch** | Auto-deletes files if check-in interval is missed |
| **Remote Transfer** | SSH2 SFTP with progress tracking |
| **Large Files** | Memory-efficient chunked transfer and reassembly |
| **Web UI** | Local Axum web server with REST API |
| **TTL Expiry** | Self-destruct timer per encrypted file |
| **Compression** | Automatic GZIP for files >1 KB |

## Installation

```bash
git clone https://github.com/ChronoCoders/hermes.git
cd hermes
cargo build --release
```

The binary is at `target/release/hermes`. Optionally place it on your `$PATH`.

## Quick Start

### 1. Initialize

```bash
hermes init
```

Edit `~/.config/hermes/config.toml` to set SFTP credentials and default paths.

### 2. Generate a keypair

```bash
hermes keygen alice
```

### 3. Encrypt and decrypt a file

```bash
# Password-based
hermes send-file secret.pdf -p MyPassword
hermes recv-file secret_20260101_120000.enc -p MyPassword

# Public-key (one or more recipients)
hermes send-file secret.pdf --recipients alice,bob
hermes recv-file secret_20260101_120000.enc --recipient alice
```

### 4. Start the web UI

```bash
hermes web
```

Open `http://localhost:3000` in a browser.

## Command Reference

### Configuration

```bash
hermes init                   # Create default config
hermes config                 # Show current config
hermes list                   # List encrypted files with status
```

### Key Management

```bash
hermes keygen <name>                        # Generate RSA-4096 keypair
hermes keygen --pqc <name>                  # Generate Kyber + Dilithium keypair
hermes export-pubkey <name> -o <file>       # Export public key
hermes import-pubkey <name> <file>          # Import recipient public key
hermes list-keys                            # List all managed keys
hermes list-archived-keys                   # List rotated/archived keys
hermes key-rotate <name>                    # Rotate a keypair
hermes key-split <name> -k <threshold> -n <shares>   # Split private key via Shamir
hermes key-recover <name> <share1> <share2> ...       # Reconstruct private key
```

### Encryption

```bash
# Messages
hermes send-msg "text" -p <password> [-t <hours>]
hermes send-msg "text" --recipients alice,bob [-t <hours>]

# Files
hermes send-file <path> -p <password> [-t <hours>]
hermes send-file <path> --recipients alice,bob [-t <hours>]

# Chunked (large files)
hermes send-file-chunked <path> --recipients alice

# Batch
hermes send-batch <file1> <file2> ... --recipients alice

# Directory
hermes send-dir <path> --recipients alice
```

### Decryption

```bash
hermes recv-msg <file> -p <password>
hermes recv-msg <file> --recipient alice
hermes recv-file <file> -p <password> [-o <output>]
hermes recv-file <file> --recipient alice [-o <output>]
hermes recv-file-chunked <prefix> --recipient alice
hermes recv-batch <dir> --recipient alice
```

### Signatures

```bash
hermes sign-file <file> --key alice
hermes verify-signature <file> --sig <sigfile> --key alice
```

### Steganography

```bash
hermes stego-hide <image.png> <payload.enc> -o <output.png>
hermes stego-reveal <image.png> -o <payload.enc>
hermes stego-capacity <image.png>
```

### Dead Man's Switch

```bash
hermes checkin                # Record a check-in
hermes dms-status             # Show DMS state and next deadline
hermes dms-disable            # Disable the switch
```

## HRMS Binary Format

Every encrypted file is a self-contained HRMS container:

```
[Magic]           4 bytes   "HRMS"
[Version]         1 byte    0x01 (RSA) | 0x02 (PQC)
[Flags]           1 byte    Compressed, Multi-recipient
[Argon2id Salt]   variable
[AES-GCM Nonce]   12 bytes
[SHA-256 Checksum] 32 bytes
[Original Size]   8 bytes
[Expires At]      8 bytes   Unix timestamp (0 = no expiry)
[Filename]        variable
[Recipient Blocks]           One per recipient:
  [Name]          variable
  [Encrypted Key] ~512 bytes (RSA) or Kyber ciphertext
[Ciphertext]      variable  GZIP-compressed if flag set
```

## Configuration

`~/.config/hermes/config.toml`:

```toml
[sftp]
host = "storage.example.com"
port = 22
username = "hermes"
key_file = "~/.ssh/hermes_key"
```

## License

MIT — see LICENSE for details.

## Contact

Altug Tatlisu — contact@chronocoder.dev
