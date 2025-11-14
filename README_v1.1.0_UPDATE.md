# 🔐 HERMES v1.1.0 - README UPDATE

## NEW COMMANDS (v1.1.0)

Add the following sections to your README.md:

---

## 🎮 Interactive Mode (NEW in v1.1.0)

Launch the interactive menu-driven interface - perfect for beginners!

```bash
hermes interactive
```

Features:
- 📋 Menu-driven interface (no command-line arguments needed)
- 🎯 Guided wizards for all operations
- ✨ Beautiful UI with colored prompts
- 🔑 Integrated key management
- ⚙️ Configuration wizard
- 📦 Batch operations support

Perfect for:
- Users new to command-line tools
- Quick operations without remembering syntax
- Exploratory usage

---

## 📦 Batch Operations (NEW in v1.1.0)

### Send Multiple Files

```bash
# Encrypt multiple files at once
hermes send-batch file1.txt file2.pdf file3.jpg -p MyPassword123

# With recipients
hermes send-batch *.pdf --recipients alice,bob
```

### Send Entire Directory

```bash
# Encrypt all files in a directory
hermes send-dir /path/to/folder -p MyPassword123

# Recursive (include subdirectories)
hermes send-dir /path/to/folder -p MyPassword123 --recursive

# With self-destruct timer
hermes send-dir /docs --recipients alice,bob -t 48 --recursive
```

### Receive Multiple Files

```bash
# Decrypt multiple files
hermes recv-batch msg1.enc msg2.enc file1.enc -p MyPassword123

# Specify output directory
hermes recv-batch *.enc -p MyPassword123 -o ./decrypted/

# Multi-recipient batch
hermes recv-batch *.enc --recipient alice -o ./output/
```

---

## 📖 Updated Command Reference

### Configuration & Setup

| Command | Description |
|---------|-------------|
| `hermes init` | Initialize Hermes configuration |
| `hermes config` | Display current configuration |
| `hermes validate` | Validate configuration |
| `hermes validate --test-connection` | Test SFTP connection |
| `hermes list` | List all encrypted files with status |
| `hermes interactive` | 🆕 Launch interactive TUI mode |

### Batch Operations 🆕

| Command | Description |
|---------|-------------|
| `hermes send-batch <files...>` | Encrypt multiple files |
| `hermes send-dir <directory>` | Encrypt entire directory |
| `hermes send-dir <dir> --recursive` | Encrypt directory recursively |
| `hermes recv-batch <files...>` | Decrypt multiple files |

### Key Management

| Command | Description |
|---------|-------------|
| `hermes keygen <name>` | Generate RSA-4096 keypair |
| `hermes export-pubkey <name> -o <file>` | Export public key |
| `hermes import-pubkey <name> <file>` | Import recipient's public key |
| `hermes list-keys` | List all keys and recipients |

### Encryption & Decryption

**Messages:**
```bash
# Password-based
hermes send-msg <message> -p <password> [-t <hours>]
hermes recv-msg <file> -p <password>

# Multi-recipient
hermes send-msg <message> --recipients <name1,name2> [-t <hours>]
hermes recv-msg <file> --recipient <name>
```

**Files:**
```bash
# Password-based
hermes send-file <path> -p <password> [-t <hours>]
hermes recv-file <file> -p <password> [-o <output>]

# Multi-recipient
hermes send-file <path> --recipients <name1,name2> [-t <hours>]
hermes recv-file <file> --recipient <name> [-o <output>]
```

---

## 🚀 Quick Start Examples (v1.1.0)

### Example 1: Interactive Mode (Easiest!)

```bash
# Launch interactive mode
hermes interactive

# Then follow the on-screen menu:
# 1. Select "📤 Send File"
# 2. Enter file path
# 3. Choose encryption type (Password or Recipients)
# 4. Done!
```

### Example 2: Batch Encrypt Project Files

```bash
# Encrypt all PDFs in a directory
hermes send-batch ./reports/*.pdf -p ProjectPassword2024

# Encrypt entire project folder recursively
hermes send-dir ./my-project --recursive --recipients alice,bob,charlie -t 72

# Output:
# [1/15] Processing: report1.pdf
#    ✓ Success: /vault/files/report1_20250126_123456.enc
# [2/15] Processing: report2.pdf
#    ✓ Success: /vault/files/report2_20250126_123457.enc
# ...
# Total Files: 15
# Successful: 15
# Failed: 0
```

### Example 3: Batch Decrypt

```bash
# Decrypt all encrypted files
hermes recv-batch *.enc -p ProjectPassword2024 -o ./decrypted/

# Multi-recipient
hermes recv-batch msg_*.enc --recipient alice -o ./messages/
```

---

## 🎯 Features (Updated v1.1.0)

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
- **🆕 Batch Operations**: Process multiple files efficiently
- **🆕 Interactive Mode**: User-friendly TUI interface

### 🎨 User Experience
- Beautiful CLI interface with colored output
- Progress indicators for large operations
- Detailed status messages
- Cross-platform support (Windows, Linux, macOS)
- 🆕 Interactive wizards for all operations
- 🆕 Shell completion support (bash, zsh, fish, etc.)

---

## 💡 Pro Tips (v1.1.0)

### Tip 1: Use Interactive Mode for Exploration

```bash
# Perfect for learning and testing
hermes interactive
```

### Tip 2: Batch Operations Save Time

```bash
# Instead of:
hermes send-file file1.pdf -p pass
hermes send-file file2.pdf -p pass
hermes send-file file3.pdf -p pass

# Do this:
hermes send-batch file1.pdf file2.pdf file3.pdf -p pass
```

### Tip 3: Recursive Directory Encryption

```bash
# Encrypt entire project including subdirectories
hermes send-dir ./my-project --recursive -p ProjectPass

# With expiration
hermes send-dir ./temp-files --recursive -p pass -t 24
```

### Tip 4: Shell Completion

```bash
# Install completion for your shell
hermes completion bash > ~/.hermes-completion.sh
echo "source ~/.hermes-completion.sh" >> ~/.bashrc

# Now you have tab completion!
hermes send-<TAB>
# Shows: send-batch  send-dir  send-file  send-msg
```

---

## 📄 License

This project is licensed under the MIT License.

---

**⚡ Built with Rust 🦀 | Secured by Mathematics 🔢 | Protected by Design 🛡️**
**🎮 v1.1.0 - Now with Interactive Mode & Batch Operations!**
