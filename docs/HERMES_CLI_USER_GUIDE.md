# HERMES - CLI User Guide

## Table of Contents
1. [Installation](#installation)
2. [Initial Setup](#initial-setup)
3. [Basic Commands](#basic-commands)
4. [Advanced Usage](#advanced-usage)
5. [Examples](#examples)
6. [Troubleshooting](#troubleshooting)

---

## Installation

### Download Binary
Place `hermes.exe` in a directory of your choice:
- `C:\Program Files\Hermes\`
- `C:\hermes\`
- Or any location in your PATH

### Add to PATH (Optional)
```cmd
setx PATH "%PATH%;C:\hermes"
```

---

## Initial Setup

### 1. Initialize Configuration
```cmd
hermes init
```

This creates the configuration file at:
`C:\Users\<USERNAME>\AppData\Roaming\hermes\config.toml`

### 2. Generate SSH Key
```cmd
ssh-keygen -t rsa -b 4096 -f C:\Users\%USERNAME%\.ssh\hermes_key -N ""
ssh-keygen -p -m PEM -f C:\Users\%USERNAME%\.ssh\hermes_key
```

### 3. Configure Authorized Keys
```cmd
type C:\Users\%USERNAME%\.ssh\hermes_key.pub >> C:\Users\%USERNAME%\.ssh\authorized_keys
```

### 4. Create Vault Directories
```cmd
mkdir C:\hermes_vault\inbox
mkdir C:\hermes_vault\outbox
mkdir C:\hermes_vault\files
```

### 5. Start SSH Service
```cmd
net start sshd
```

---

## Basic Commands

### View Help
```cmd
hermes --help
hermes <command> --help
```

### View Configuration
```cmd
hermes config
```

### List Encrypted Files
```cmd
hermes list
```

Shows all encrypted files in vault directories:
- OUTBOX (sent messages)
- INBOX (received messages)
- FILES (encrypted files)

---

## Command Reference

### 1. SEND-MSG - Encrypt and Send Message

**Syntax:**
```cmd
hermes send-msg "<message>" --password <password> [--remote-path <path>]
```

**Parameters:**
- `<message>` - Text message to encrypt (required)
- `-p, --password` - Encryption password (required)
- `-r, --remote-path` - Custom upload path (optional)

**Examples:**
```cmd
hermes send-msg "Secret meeting at 3pm" --password MyPass123

hermes send-msg "Classified info" -p SecurePass456

hermes send-msg "Custom location" -p pass123 --remote-path C:\hermes_vault\inbox\custom.enc
```

**Output:**
- Remote path where file was uploaded
- Cipher type (AES-256-GCM)
- Status confirmation

---

### 2. RECV-MSG - Download and Decrypt Message

**Syntax:**
```cmd
hermes recv-msg <filename> --password <password>
```

**Parameters:**
- `<filename>` - Encrypted file name or full path (required)
- `-p, --password` - Decryption password (required)

**Examples:**
```cmd
hermes recv-msg msg_20251024_163826.enc --password MyPass123

hermes recv-msg C:\hermes_vault\outbox\msg_20251024_163826.enc -p MyPass123

hermes recv-msg custom.enc -p pass123
```

**Output:**
- Decrypted message text
- Source path
- File size
- Integrity verification status

---

### 3. SEND-FILE - Encrypt and Upload File

**Syntax:**
```cmd
hermes send-file <filepath> --password <password> [--remote-path <path>]
```

**Parameters:**
- `<filepath>` - Local file path to encrypt (required)
- `-p, --password` - Encryption password (required)
- `-r, --remote-path` - Custom upload path (optional)

**Examples:**
```cmd
hermes send-file report.pdf --password MyPass123

hermes send-file C:\Documents\secret.docx -p SecurePass456

hermes send-file data.xlsx -p pass123 --remote-path C:\hermes_vault\files\custom_data.enc
```

**Features:**
- Automatic GZIP compression for files > 1KB
- Shows compression ratio
- Progress bar for large files
- SHA-256 integrity checksum

**Output:**
- Remote path
- Original file size
- Encrypted size
- Compression status
- Lock confirmation

---

### 4. RECV-FILE - Download and Decrypt File

**Syntax:**
```cmd
hermes recv-file <filename> --password <password> [--output <path>]
```

**Parameters:**
- `<filename>` - Encrypted file name or full path (required)
- `-p, --password` - Decryption password (required)
- `-o, --output` - Output file path (optional, uses original name if not specified)

**Examples:**
```cmd
hermes recv-file report_20251024.enc --password MyPass123

hermes recv-file secret_20251024.enc -p SecurePass456 --output decrypted.docx

hermes recv-file C:\hermes_vault\files\data.enc -p pass123 -o restored_data.xlsx
```

**Features:**
- Automatic decompression if file was compressed
- Integrity verification (SHA-256)
- Original filename preservation
- Progress bar for large files

**Output:**
- Output file path
- File size
- Integrity verification status
- Decompression status

---

### 5. LIST - Show Vault Contents

**Syntax:**
```cmd
hermes list
```

**No parameters required**

**Output:**
Shows three directories with file details:

**OUTBOX** - Sent messages
```
📁 OUTBOX
   Path: C:\hermes_vault\outbox
   • msg_20251024_163826.enc (0.26 KB)
```

**INBOX** - Received messages
```
📁 INBOX
   Path: C:\hermes_vault\inbox
   • custom_msg.enc (0.26 KB)
```

**FILES** - Encrypted files
```
📁 FILES
   Path: C:\hermes_vault\files
   • report_20251024.enc (13.33 MB)
```

---

### 6. INIT - Initialize Configuration

**Syntax:**
```cmd
hermes init
```

**What it does:**
- Creates default configuration file
- Sets up SFTP connection parameters
- Configures vault directory paths

**Configuration location:**
`C:\Users\<USERNAME>\AppData\Roaming\hermes\config.toml`

---

### 7. CONFIG - View Current Configuration

**Syntax:**
```cmd
hermes config
```

**Output shows:**
- SFTP host and port
- Username
- SSH key file path
- Vault directory paths (inbox, outbox, files)

---

## Advanced Usage

### Custom Remote Paths

Upload to any location in the vault:

```cmd
hermes send-msg "Test" -p pass --remote-path C:\hermes_vault\custom\folder\msg.enc

hermes send-file doc.pdf -p pass --remote-path C:\hermes_vault\archive\2025\doc.enc
```

### Using Short Parameter Names

All commands support short parameter names:

```cmd
-p instead of --password
-r instead of --remote-path
-o instead of --output
```

**Examples:**
```cmd
hermes send-msg "Quick message" -p mypass

hermes recv-file data.enc -p mypass -o output.xlsx

hermes send-file report.pdf -p mypass -r C:\hermes_vault\files\report.enc
```

---

## Examples

### Example 1: Simple Message Exchange

**Sender:**
```cmd
hermes send-msg "Meeting tomorrow at 10am" --password SecretPass123
```

**Output:**
```
✓ MESSAGE TRANSMITTED
  Remote: C:\hermes_vault\outbox/msg_20251024_163826.enc
  Cipher: AES-256-GCM
  Status: [SECURE]
```

**Receiver:**
```cmd
hermes list
hermes recv-msg msg_20251024_163826.enc --password SecretPass123
```

**Output:**
```
✓ MESSAGE DECRYPTED & VERIFIED
┌─[PLAINTEXT]──────────────────────────────────────┐
│ Meeting tomorrow at 10am │
└──────────────────────────────────────────────────┘
  From: C:\hermes_vault\outbox/msg_20251024_163826.enc
  Size: 24 bytes
  Integrity: VERIFIED ✓
  Status: [SECURE]
```

---

### Example 2: File Encryption with Compression

```cmd
hermes send-file large_report.pdf --password FilePass456
```

**Output:**
```
✓ FILE SECURED
  Remote: C:\hermes_vault\files/large_report_20251024.enc
  Original: 25.50 MB
  Encrypted: 18.75 MB
  Compression: GZIP
  Status: [LOCKED]
```

---

### Example 3: Custom Path Organization

**Organize by project:**
```cmd
hermes send-msg "Project Alpha update" -p pass123 -r C:\hermes_vault\inbox\project_alpha\update.enc

hermes send-file alpha_report.pdf -p pass123 -r C:\hermes_vault\files\project_alpha\report.enc
```

**Organize by date:**
```cmd
hermes send-msg "Daily briefing" -p pass123 -r C:\hermes_vault\inbox\2025-10-24\briefing.enc
```

---

### Example 4: Batch Processing

**Send multiple messages:**
```cmd
hermes send-msg "Message 1" -p pass123
hermes send-msg "Message 2" -p pass123
hermes send-msg "Message 3" -p pass123
```

**Check vault:**
```cmd
hermes list
```

**Receive all:**
```cmd
hermes recv-msg msg_20251024_163826.enc -p pass123
hermes recv-msg msg_20251024_163827.enc -p pass123
hermes recv-msg msg_20251024_163828.enc -p pass123
```

---

## Security Best Practices

### 1. Password Strength
Use strong passwords with:
- Minimum 12 characters
- Mix of uppercase, lowercase, numbers, symbols
- Avoid dictionary words

**Good:** `Tr0ng!P@ssw0rd#2024`
**Bad:** `password123`

### 2. Password Management
- Use different passwords for different messages/files
- Never share passwords over insecure channels
- Consider using a password manager

### 3. SSH Key Security
```cmd
icacls C:\Users\%USERNAME%\.ssh\hermes_key /inheritance:r
icacls C:\Users\%USERNAME%\.ssh\hermes_key /grant:r "%USERNAME%:R"
```

### 4. File Cleanup
Regularly delete old encrypted files:
```cmd
del C:\hermes_vault\outbox\*.enc
```

### 5. Verify Integrity
Always check "VERIFIED ✓" status after decryption:
```
Integrity: VERIFIED ✓
```

---

## Troubleshooting

### Error: "Configuration file not found"
**Solution:**
```cmd
hermes init
```

### Error: "SFTP connection failed"
**Solution:**
```cmd
net start sshd
net localgroup "OpenSSH Users" %USERNAME% /add
```

### Error: "Key authentication failed"
**Solution:**
Convert key to PEM format:
```cmd
ssh-keygen -p -m PEM -f C:\Users\%USERNAME%\.ssh\hermes_key
```

### Error: "File not found"
**Solution:**
Use `hermes list` to see available files and copy exact filename.

### Error: "Decryption failed - wrong password?"
**Solution:**
Check password spelling, case-sensitivity matters.

### Error: "Directory not accessible"
**Solution:**
Create missing directories:
```cmd
mkdir C:\hermes_vault\inbox
mkdir C:\hermes_vault\outbox
mkdir C:\hermes_vault\files
```

---

## Configuration File Reference

**Location:** `C:\Users\<USERNAME>\AppData\Roaming\hermes\config.toml`

**Default Configuration:**
```toml
[sftp]
host = "localhost"
port = 22
username = "your_username"
key_file = "C:\\Users\\your_username\\.ssh\\hermes_key"

[paths]
inbox = "C:\\hermes_vault\\inbox"
outbox = "C:\\hermes_vault\\outbox"
files = "C:\\hermes_vault\\files"
```

**Customization:**
Edit this file to change:
- SFTP server address (for remote servers)
- SSH key location
- Vault directory paths
- Port number

---

## Command Quick Reference

| Command | Purpose | Example |
|---------|---------|---------|
| `hermes init` | Initialize config | `hermes init` |
| `hermes config` | View settings | `hermes config` |
| `hermes list` | List vault files | `hermes list` |
| `hermes send-msg` | Encrypt message | `hermes send-msg "text" -p pass` |
| `hermes recv-msg` | Decrypt message | `hermes recv-msg file.enc -p pass` |
| `hermes send-file` | Encrypt file | `hermes send-file doc.pdf -p pass` |
| `hermes recv-file` | Decrypt file | `hermes recv-file file.enc -p pass` |

---

## Technical Specifications

### Encryption
- **Algorithm:** AES-256-GCM (NSA-approved for TOP SECRET)
- **Key Derivation:** Argon2 (memory-hard, GPU-resistant)
- **Nonce:** 96-bit random (unique per encryption)
- **Authentication:** Built-in MAC (prevents tampering)

### Compression
- **Algorithm:** GZIP (Deflate)
- **Threshold:** Files > 1KB automatically compressed
- **Decision:** Only if compression reduces size

### Integrity
- **Algorithm:** SHA-256 checksum
- **Verification:** Automatic on decryption
- **Protection:** Detects any data corruption or tampering

### Transport
- **Protocol:** SFTP (SSH File Transfer Protocol)
- **Authentication:** RSA-4096 public key
- **Encryption:** SSH transport layer security

---

## Version Information

**Version:** 1.0.0
**Release Date:** October 24, 2025
**Platform:** Windows 10/11
**License:** MIT

---

## Support

For issues or questions:
1. Check this guide first
2. Run `hermes --help` for command syntax
3. Review error messages carefully
4. Check GitHub issues (if open source)

---

**End of User Guide**
