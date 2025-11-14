# 🪟 HERMES v1.1.0 - WINDOWS INSTALLATION

## 🚀 QUICK START (5 Minutes)

### Option 1: Automatic Setup (Recommended)

```cmd
REM 1. Download v1.1.0 files to C:\hermes_v1.1.0\

REM 2. Run integration script
cd C:\hermes_v1.1.0\outputs
integrate_v1.1.0.bat

REM 3. Follow prompts and enter your project path
REM    Example: C:\Users\YourName\hermes

REM 4. Build and run
cd C:\Users\YourName\hermes
cargo build --release
target\release\hermes.exe interactive
```

### Option 2: Manual Setup

```cmd
REM 1. Navigate to your Hermes project
cd C:\Users\%USERNAME%\hermes

REM 2. Copy new files
copy C:\hermes_v1.1.0\outputs\src\commands\*.rs src\commands\
copy C:\hermes_v1.1.0\outputs\src\main.rs src\
copy C:\hermes_v1.1.0\outputs\src\commands\mod.rs src\commands\

REM 3. Build release
cargo build --release

REM 4. Test
target\release\hermes.exe interactive
```

---

## 📦 WHAT'S INCLUDED

```
outputs\
├── integrate_v1.1.0.bat            - Automatic integration script
├── setup_windows.bat               - First-time setup script
├── WINDOWS_INTEGRATION_GUIDE.md    - Complete Windows guide
├── V1.1.0_COMPLETE_SUMMARY_EN.md   - Feature summary
├── CHANGELOG_v1.1.0.md             - Changelog
├── README_v1.1.0_UPDATE.md         - README updates
├── QUICK_REFERENCE_v1.1.0.txt      - Quick reference
└── src\
    ├── main.rs                     - Updated main file
    └── commands\
        ├── mod.rs                  - Updated module list
        ├── send_batch.rs           - NEW: Batch encryption
        ├── send_dir.rs             - NEW: Directory encryption
        ├── recv_batch.rs           - NEW: Batch decryption
        └── interactive.rs          - NEW: Interactive mode
```

---

## ✅ NEW FEATURES (v1.1.0)

### 1. Interactive Mode
```cmd
hermes.exe interactive
```
- Menu-driven interface
- No command-line arguments needed
- Perfect for beginners

### 2. Batch Operations
```cmd
REM Encrypt multiple files
hermes.exe send-batch file1.txt file2.pdf file3.jpg -p Password

REM Encrypt directory
hermes.exe send-dir C:\MyProject --recursive -p Password

REM Decrypt multiple files
hermes.exe recv-batch *.enc -p Password -o C:\output\
```

### 3. Progress Bars
- Real-time progress for large files
- Visual feedback for all operations

### 4. Shell Completion
```cmd
hermes.exe completion powershell > hermes-completion.ps1
```

### 5. Config Validation
```cmd
hermes.exe validate --test-connection
```

---

## 🎯 COMMON TASKS (Windows CMD)

### Encrypt Files

```cmd
REM Single file
hermes.exe send-file report.pdf -p MyPassword123

REM Multiple files
hermes.exe send-batch *.txt -p MyPassword123

REM Entire directory
hermes.exe send-dir C:\Documents\project --recursive -p MyPassword123
```

### Decrypt Files

```cmd
REM Single file
hermes.exe recv-file report_20250126.enc -p MyPassword123

REM Multiple files
hermes.exe recv-batch *.enc -p MyPassword123 -o C:\decrypted\

REM All encrypted files in directory
hermes.exe recv-batch C:\encrypted\*.enc -p MyPassword123 -o C:\decrypted\
```

### Multi-Recipient

```cmd
REM Generate keypair
hermes.exe keygen alice

REM Export public key
hermes.exe export-pubkey alice -o C:\keys\alice_public.pem

REM Import recipient key
hermes.exe import-pubkey bob C:\keys\bob_public.pem

REM Send to multiple recipients
hermes.exe send-file document.pdf --recipients alice,bob,charlie
```

---

## 🔧 CONFIGURATION

### Config File Location
```
C:\Users\%USERNAME%\AppData\Roaming\hermes\config.toml
```

### Default Config
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

### Create Vault Directories
```cmd
mkdir C:\hermes_vault\inbox
mkdir C:\hermes_vault\outbox
mkdir C:\hermes_vault\files
```

---

## 🔑 SSH SETUP

### Install OpenSSH Server

**Windows 10/11:**
1. Open Settings → Apps → Optional Features
2. Click "Add a feature"
3. Search for "OpenSSH Server"
4. Click Install

**Or via PowerShell (Admin):**
```powershell
Add-WindowsCapability -Online -Name OpenSSH.Server~~~~0.0.1.0
```

### Generate SSH Key

```cmd
REM Create .ssh directory
mkdir %USERPROFILE%\.ssh

REM Generate key
ssh-keygen -t rsa -b 4096 -f %USERPROFILE%\.ssh\hermes_key -N ""

REM Add to authorized keys
type %USERPROFILE%\.ssh\hermes_key.pub >> %USERPROFILE%\.ssh\authorized_keys
```

### Start SSH Service

```cmd
REM Start service
net start sshd

REM Enable on startup
sc config sshd start=auto
```

---

## 📝 BATCH SCRIPTS

### encrypt_folder.bat
```batch
@echo off
hermes.exe send-dir %1 --recursive -p %2
```

Usage:
```cmd
encrypt_folder.bat C:\MyProject MyPassword
```

### decrypt_all.bat
```batch
@echo off
hermes.exe recv-batch *.enc -p %1 -o .\decrypted\
```

Usage:
```cmd
decrypt_all.bat MyPassword
```

---

## 🐛 TROUBLESHOOTING

### "hermes.exe not found"
```cmd
REM Check binary location
dir target\release\hermes.exe

REM Run from project directory
cd C:\path\to\hermes
target\release\hermes.exe --help

REM Or add to PATH
set PATH=%PATH%;C:\path\to\hermes\target\release
```

### "SFTP connection failed"
```cmd
REM Check SSH service
sc query sshd

REM Start SSH service
net start sshd

REM Enable on startup
sc config sshd start=auto
```

### "Permission denied"
```cmd
REM Run as Administrator
REM Right-click CMD → Run as Administrator

REM Fix permissions
icacls C:\hermes_vault /grant %USERNAME%:F /T
```

---

## 💡 PRO TIPS

### 1. Add to PATH
```cmd
REM Temporary (current session)
set PATH=%PATH%;C:\hermes\target\release

REM Permanent (requires admin)
setx PATH "%PATH%;C:\hermes\target\release" /M
```

### 2. Use Quotes for Paths with Spaces
```cmd
hermes.exe send-file "C:\My Documents\report.pdf" -p Password
```

### 3. Wildcards Work!
```cmd
hermes.exe send-batch *.pdf -p Password
hermes.exe recv-batch msg*.enc -p Password -o C:\output\
```

### 4. Environment Variables
```cmd
hermes.exe send-dir %USERPROFILE%\Documents --recursive -p Password
```

---

## 📊 SYSTEM REQUIREMENTS

- **OS:** Windows 10/11 (64-bit)
- **Rust:** 1.70+ (for building)
- **OpenSSH:** Server component (optional)
- **Disk:** 50 MB for binary + vault space
- **RAM:** 100 MB minimum

---

## 🚀 DEPLOYMENT

### Production Deployment

```cmd
REM 1. Build release
cargo build --release

REM 2. Copy to Program Files
copy target\release\hermes.exe "C:\Program Files\Hermes\"

REM 3. Add to system PATH (admin)
setx PATH "%PATH%;C:\Program Files\Hermes" /M

REM 4. Create vault
mkdir C:\ProgramData\Hermes\vault\inbox
mkdir C:\ProgramData\Hermes\vault\outbox
mkdir C:\ProgramData\Hermes\vault\files

REM 5. Initialize
hermes.exe init
```

---

## 📞 SUPPORT

- **Email:** contact@chronocoder.dev
- **GitHub:** https://github.com/ChronoCoders/hermes
- **Docs:** WINDOWS_INTEGRATION_GUIDE.md

---

## ✅ CHECKLIST

After installation, verify:

- [ ] `hermes.exe --version` works
- [ ] Config file created at `%APPDATA%\hermes\config.toml`
- [ ] Vault directories exist at `C:\hermes_vault\`
- [ ] SSH key exists at `%USERPROFILE%\.ssh\hermes_key`
- [ ] `hermes.exe interactive` launches
- [ ] `hermes.exe send-batch --help` shows help

---

**⚡ Windows 10/11 | CMD Compatible | Built with Rust 🦀 | v1.1.0 ✅**
