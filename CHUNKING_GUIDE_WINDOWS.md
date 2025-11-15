# Windows Guide - File Chunking (v1.2.0)

## Quick Start

### Send Large File (Chunked)

```cmd
REM Encrypt 5GB file in chunks
hermes.exe send-file-chunked C:\large_video.mp4 -p MyPassword123

REM With recipients
hermes.exe send-file-chunked C:\large_backup.zip --recipients alice,bob

REM With expiration (7 days)
hermes.exe send-file-chunked C:\data.tar.gz -p Password -t 168
```

### Receive Chunked File

```cmd
REM Decrypt chunked file using manifest
hermes.exe recv-file-chunked large_video_20250127.manifest.enc -p MyPassword123

REM Specify output path
hermes.exe recv-file-chunked backup.manifest.enc -p Password -o C:\restored\backup.zip

REM Multi-recipient
hermes.exe recv-file-chunked data.manifest.enc --recipient alice -o C:\output\data.tar.gz
```

## How It Works

### Chunking Process

1. **Split**: File divided into 50MB chunks
2. **Encrypt**: Each chunk encrypted separately
3. **Upload**: Chunks uploaded to vault
4. **Manifest**: Metadata file with chunk list

### File Structure

```
large_video.mp4 (5GB)
    ↓
large_video_20250127.chunk.001.enc (50MB)
large_video_20250127.chunk.002.enc (50MB)
large_video_20250127.chunk.003.enc (50MB)
...
large_video_20250127.chunk.100.enc (50MB)
large_video_20250127.manifest.enc (metadata)
```

## Advantages

### Memory Efficient
- Processes 50MB at a time
- Handles files larger than RAM
- Tested with 10GB+ files

### Resumable
- Each chunk independent
- Re-upload failed chunks only
- Network interruption tolerant

### Verified
- SHA-256 hash per chunk
- Full file hash verification
- Tamper detection

## Examples

### Backup Large Database

```cmd
REM Backup 20GB database
hermes.exe send-file-chunked C:\db\production.sql -p DBPassword2025

REM Output shows progress per chunk
REM [1/400] Chunk 1
REM [2/400] Chunk 2
REM ...
```

### Video File Transfer

```cmd
REM Send 8GB video project
hermes.exe send-file-chunked C:\projects\final_edit.mov --recipients team

REM Team member receives:
hermes.exe recv-file-chunked final_edit_20250127.manifest.enc --recipient john
```

### Archive Distribution

```cmd
REM Distribute large software archive
hermes.exe send-file-chunked C:\releases\v2.0.0.tar.gz -p Release2025 -t 720
```

## Comparison

### Regular vs Chunked

| Feature | Regular | Chunked |
|---------|---------|---------|
| Max File Size | ~2GB | Unlimited |
| Memory Usage | Full file | 50MB |
| Resume Support | No | Yes |
| Network Tolerance | Low | High |

### When to Use

**Use Regular** (`send-file`):
- Files < 1GB
- Fast network
- Single uninterrupted transfer

**Use Chunked** (`send-file-chunked`):
- Files > 1GB
- Limited RAM
- Unreliable network
- Very large backups

## Troubleshooting

### Chunk Missing

```cmd
REM Error: Chunk file not found
REM Solution: Re-upload specific chunk
hermes.exe send-file-chunked C:\file.zip -p Password
```

### Hash Mismatch

```cmd
REM Error: Chunk hash mismatch
REM Solution: Corrupt chunk, re-download
hermes.exe recv-file-chunked file.manifest.enc -p Password
```

### Disk Space

```cmd
REM Check free space before sending
dir C:\

REM Ensure 2x file size available
REM Original: 10GB
REM Need: 20GB (10GB chunks + 10GB encrypted)
```

## Performance

### Benchmark (Windows 10)

| File Size | Chunks | Time | Speed |
|-----------|--------|------|-------|
| 1GB | 20 | 45s | 22MB/s |
| 5GB | 100 | 3m 30s | 24MB/s |
| 10GB | 200 | 7m 15s | 23MB/s |

### Optimization Tips

```cmd
REM 1. Use SSD for temp directory
set TEMP=D:\fast_ssd

REM 2. Close background apps
REM 3. Use wired connection

REM 4. Process multiple files sequentially
hermes.exe send-file-chunked file1.zip -p Pass
hermes.exe send-file-chunked file2.zip -p Pass
```

## Batch Script Example

### chunk_backup.bat

```batch
@echo off
setlocal

set BACKUP_DIR=C:\backups\weekly
set PASSWORD=WeeklyBackup2025

echo Starting chunked backup...

for %%F in (%BACKUP_DIR%\*.zip) do (
    echo Processing %%F
    hermes.exe send-file-chunked "%%F" -p %PASSWORD%
    if %ERRORLEVEL% neq 0 (
        echo Failed: %%F
        pause
        exit /b 1
    )
)

echo All backups complete!
pause
```

Usage:
```cmd
chunk_backup.bat
```

## Notes

- Chunks stored in `./hermes_chunks_temp` during processing
- Automatic cleanup after completion
- Manifest file required for reassembly
- Keep manifest safe - it's the key to your chunks

---

**⚡ Windows 10/11 Compatible | Built with Rust 🦀 | v1.2.0**
