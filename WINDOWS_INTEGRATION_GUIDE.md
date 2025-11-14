# 🪟 HERMES v1.1.0 - WINDOWS INTEGRATION GUIDE

## 📋 WINDOWS CMD INSTALLATION

### Step 1: Copy Files to Your Project

```cmd
REM Navigate to your Hermes project
cd C:\Users\%USERNAME%\hermes

REM Create directories if they don't exist
mkdir src\commands 2>nul

REM Copy new command files
copy outputs\src\commands\send_batch.rs src\commands\
copy outputs\src\commands\send_dir.rs src\commands\
copy outputs\src\commands\recv_batch.rs src\commands\
copy outputs\src\commands\interactive.rs src\commands\

REM Copy updated files
copy outputs\src\commands\mod.rs src\commands\
copy outputs\src\main.rs src\

REM Copy documentation
type outputs\CHANGELOG_v1.1.0.md >> CHANGELOG.md
type outputs\README_v1.1.0_UPDATE.md >> README.md
```

---

## 🔧 BUILD AND TEST (Windows CMD)

### Build Release Binary

```cmd
REM Navigate to project
cd C:\Users\%USERNAME%\hermes

REM Build release version
cargo build --release

REM Binary will be at:
REM target\release\hermes.exe
```

### Test New Commands

```cmd
REM Test interactive mode
target\release\hermes.exe interactive

REM Test batch operations
target\release\hermes.exe send-batch --help
target\release\hermes.exe send-dir --help
target\release\hermes.exe recv-batch --help
```

---

## 🚀 USAGE EXAMPLES (Windows CMD)

### Interactive Mode

```cmd
REM Launch interactive mode
hermes.exe interactive
```

### Batch Operations

```cmd
REM Encrypt multiple files
hermes.exe send-batch file1.txt file2.pdf file3.jpg -p MyPassword123

REM Encrypt directory (Windows path)
hermes.exe send-dir C:\Users\%USERNAME%\Documents\project -p Password123 --recursive

REM Decrypt multiple files to output folder
hermes.exe recv-batch msg1.enc msg2.enc file1.enc -p Password123 -o C:\decrypted\
```

### Single File Operations

```cmd
REM Encrypt a file
hermes.exe send-file C:\Users\%USERNAME%\Documents\report.pdf -p SecurePass456

REM Decrypt a file
hermes.exe recv-file report_20250126.enc -p SecurePass456 -o C:\decrypted\report.pdf
```

### Multi-Recipient

```cmd
REM Generate your keypair
hermes.exe keygen alice

REM Export your public key
hermes.exe export-pubkey alice -o C:\keys\alice_public.pem

REM Import recipient's public key
hermes.exe import-pubkey bob C:\keys\bob_public.pem

REM Send to multiple recipients
hermes.exe send-file document.pdf --recipients alice,bob,charlie
```

---

## 📁 WINDOWS PATH EXAMPLES

### Local Paths

```cmd
REM Current directory
hermes.exe send-file report.pdf -p Password123

REM Absolute path
hermes.exe send-file C:\Users\John\Documents\secret.docx -p Password123

REM Relative path
hermes.exe send-file ..\Documents\file.txt -p Password123

REM User directory
hermes.exe send-dir %USERPROFILE%\Documents\project --recursive -p Password123
```

### Batch with Wildcards

```cmd
REM Encrypt all PDFs in current directory
hermes.exe send-batch *.pdf -p Password123

REM Encrypt all text files
hermes.exe send-batch *.txt -p Password123

REM Decrypt all .enc files
hermes.exe recv-batch *.enc -p Password123 -o C:\decrypted\
```

---

## ⚙️ CONFIGURATION (Windows)

### Configuration File Location

```
C:\Users\%USERNAME%\AppData\Roaming\hermes\config.toml
```

### Default Configuration

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

### Setup Vault Directories

```cmd
REM Create vault directories
mkdir C:\hermes_vault\inbox
mkdir C:\hermes_vault\outbox
mkdir C:\hermes_vault\files
```

---

## 🔑 SSH KEY SETUP (Windows)

### Generate SSH Key

```cmd
REM Navigate to .ssh directory
cd %USERPROFILE%\.ssh

REM Generate RSA-4096 key
ssh-keygen -t rsa -b 4096 -f hermes_key -N ""

REM Convert to PEM format (if needed)
ssh-keygen -p -m PEM -f hermes_key
```

### Configure Authorized Keys

```cmd
REM Add public key to authorized_keys
type %USERPROFILE%\.ssh\hermes_key.pub >> %USERPROFILE%\.ssh\authorized_keys
```

---

## 🎯 BATCH SCRIPTS FOR COMMON TASKS

### encrypt_folder.bat

```batch
@echo off
REM Encrypt entire folder with password
SET FOLDER=%1
SET PASSWORD=%2

if "%FOLDER%"=="" (
    echo Usage: encrypt_folder.bat [folder_path] [password]
    exit /b 1
)

if "%PASSWORD%"=="" (
    echo Usage: encrypt_folder.bat [folder_path] [password]
    exit /b 1
)

hermes.exe send-dir "%FOLDER%" --recursive -p "%PASSWORD%"
echo Done!
```

Usage:
```cmd
encrypt_folder.bat C:\MyProject MyPassword123
```

### decrypt_all.bat

```batch
@echo off
REM Decrypt all .enc files in current directory
SET PASSWORD=%1

if "%PASSWORD%"=="" (
    echo Usage: decrypt_all.bat [password]
    exit /b 1
)

hermes.exe recv-batch *.enc -p "%PASSWORD%" -o .\decrypted\
echo Done!
```

Usage:
```cmd
decrypt_all.bat MyPassword123
```

### backup_project.bat

```batch
@echo off
REM Backup project folder with timestamp
SET PROJECT=%1
SET PASSWORD=%2

if "%PROJECT%"=="" (
    echo Usage: backup_project.bat [project_folder] [password]
    exit /b 1
)

if "%PASSWORD%"=="" (
    echo Usage: backup_project.bat [project_folder] [password]
    exit /b 1
)

echo Backing up %PROJECT%...
hermes.exe send-dir "%PROJECT%" --recursive -p "%PASSWORD%" -t 168
echo Backup complete! Files will expire in 7 days.
```

Usage:
```cmd
backup_project.bat C:\MyProject BackupPassword123
```

---

## 📊 WINDOWS ENVIRONMENT VARIABLES

### Set Hermes Path

```cmd
REM Add Hermes to PATH temporarily
set PATH=%PATH%;C:\hermes\target\release

REM Add permanently (requires admin)
setx PATH "%PATH%;C:\hermes\target\release" /M
```

### Common Paths

```cmd
%USERPROFILE%          = C:\Users\YourName
%APPDATA%              = C:\Users\YourName\AppData\Roaming
%LOCALAPPDATA%         = C:\Users\YourName\AppData\Local
%TEMP%                 = C:\Users\YourName\AppData\Local\Temp
%HOMEDRIVE%            = C:
%HOMEPATH%             = \Users\YourName
```

---

## 🐛 TROUBLESHOOTING (Windows)

### Error: "hermes.exe not found"

```cmd
REM Check if binary exists
dir target\release\hermes.exe

REM Run from project directory
cd C:\path\to\hermes
target\release\hermes.exe --help

REM Or add to PATH
set PATH=%PATH%;C:\path\to\hermes\target\release
hermes.exe --help
```

### Error: "SFTP connection failed"

```cmd
REM Check OpenSSH service
sc query sshd

REM Start OpenSSH service
net start sshd

REM Enable on startup
sc config sshd start=auto
```

### Error: "Permission denied"

```cmd
REM Run CMD as Administrator
REM Right-click CMD → Run as Administrator

REM Or fix file permissions
icacls C:\hermes_vault /grant %USERNAME%:F /T
```

---

## 🎮 QUICK START (Windows CMD)

### Complete Setup

```cmd
REM 1. Navigate to Hermes directory
cd C:\hermes

REM 2. Build release
cargo build --release

REM 3. Add to PATH (temporary)
set PATH=%PATH%;C:\hermes\target\release

REM 4. Initialize configuration
hermes.exe init

REM 5. Create vault directories
mkdir C:\hermes_vault\inbox
mkdir C:\hermes_vault\outbox
mkdir C:\hermes_vault\files

REM 6. Generate SSH key
ssh-keygen -t rsa -b 4096 -f %USERPROFILE%\.ssh\hermes_key -N ""

REM 7. Start SSH service
net start sshd

REM 8. Test interactive mode
hermes.exe interactive
```

---

## 📝 NOTES FOR WINDOWS

### Path Separators
- Use backslash: `C:\folder\file.txt`
- Or forward slash (also works): `C:/folder/file.txt`
- Escape in TOML: `C:\\folder\\file.txt`

### Wildcards
- `*` matches any characters
- `?` matches single character
- Example: `*.txt`, `file?.pdf`

### Quotes
- Use quotes for paths with spaces: `"C:\My Documents\file.txt"`
- Use quotes for passwords with special chars: `"Pass@word!123"`

### Line Endings
- Windows uses CRLF (`\r\n`)
- Git may auto-convert to LF on commit
- Rust handles both automatically

---

## 🚀 PRODUCTION DEPLOYMENT (Windows)

### Create Installation Package

```cmd
REM 1. Build release
cargo build --release

REM 2. Copy binary
copy target\release\hermes.exe C:\Program Files\Hermes\

REM 3. Create default config
hermes.exe init

REM 4. Create vault directories
mkdir C:\ProgramData\Hermes\vault\inbox
mkdir C:\ProgramData\Hermes\vault\outbox
mkdir C:\ProgramData\Hermes\vault\files

REM 5. Setup SSH
ssh-keygen -t rsa -b 4096 -f C:\ProgramData\Hermes\.ssh\hermes_key -N ""

REM 6. Add to system PATH
setx PATH "%PATH%;C:\Program Files\Hermes" /M
```

---

## 📞 SUPPORT

- Email: contact@chronocoder.dev
- GitHub: https://github.com/ChronoCoders/hermes

---

**⚡ Windows 10/11 Compatible | CMD Ready | Built with Rust 🦀**
