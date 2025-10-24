# HERMES

Secure file transfer with military-grade encryption.

## Features

- **AES-256-GCM** encryption
- **Argon2** key derivation
- **SFTP** secure transfer
- **Cyberpunk CLI** interface
- **Zero dependencies** (single binary)

## Installation
```cmd
cargo build --release
```

Binary location: `target/release/hermes.exe`

## Setup

### 1. Initialize configuration
```cmd
hermes init
```

### 2. Edit config file

Location: `C:\Users\<USERNAME>\AppData\Roaming\hermes\config.toml`
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

### 3. Create SSH key
```cmd
ssh-keygen -t rsa -b 4096 -f C:\Users\%USERNAME%\.ssh\hermes_key
type C:\Users\%USERNAME%\.ssh\hermes_key.pub >> C:\Users\%USERNAME%\.ssh\authorized_keys
```

### 4. Create vault directories
```cmd
mkdir C:\hermes_vault\inbox
mkdir C:\hermes_vault\outbox
mkdir C:\hermes_vault\files
```

## Usage

### Send encrypted message
```cmd
hermes send-msg "Secret message" --password MyPass123
```

### Receive encrypted message
```cmd
hermes recv-msg msg_20251024_194523.enc --password MyPass123
```

### Upload encrypted file
```cmd
hermes send-file secret.pdf --password MyPass123
```

### Download encrypted file
```cmd
hermes recv-file secret_20251024.enc --password MyPass123 --output decrypted.pdf
```

## Security

- **AES-256-GCM**: Authenticated encryption
- **Argon2**: Memory-hard key derivation
- **SSH keys**: No password transmission
- **Random nonces**: Each encryption unique

## License

MIT