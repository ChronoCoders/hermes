@echo off
REM ============================================================
REM HERMES v1.1.0 - Windows Setup Script
REM ============================================================

echo.
echo ========================================
echo  HERMES v1.1.0 - WINDOWS SETUP
echo ========================================
echo.

REM Check if running from correct directory
if not exist "Cargo.toml" (
    echo ERROR: Cargo.toml not found!
    echo Please run this script from the Hermes project root directory.
    pause
    exit /b 1
)

REM Step 1: Build Release
echo [1/6] Building release binary...
cargo build --release
if %ERRORLEVEL% neq 0 (
    echo ERROR: Build failed!
    pause
    exit /b 1
)
echo Done!
echo.

REM Step 2: Initialize Configuration
echo [2/6] Initializing configuration...
target\release\hermes.exe init
if %ERRORLEVEL% neq 0 (
    echo ERROR: Init failed!
    pause
    exit /b 1
)
echo Done!
echo.

REM Step 3: Create Vault Directories
echo [3/6] Creating vault directories...
mkdir C:\hermes_vault\inbox 2>nul
mkdir C:\hermes_vault\outbox 2>nul
mkdir C:\hermes_vault\files 2>nul
echo Done!
echo.

REM Step 4: Generate SSH Key
echo [4/6] Generating SSH key...
if not exist "%USERPROFILE%\.ssh" mkdir "%USERPROFILE%\.ssh"
ssh-keygen -t rsa -b 4096 -f "%USERPROFILE%\.ssh\hermes_key" -N ""
if %ERRORLEVEL% neq 0 (
    echo WARNING: SSH key generation failed or already exists
)
echo Done!
echo.

REM Step 5: Start SSH Service
echo [5/6] Starting SSH service...
net start sshd 2>nul
if %ERRORLEVEL% neq 0 (
    echo WARNING: Could not start SSH service
    echo You may need to install OpenSSH Server
)
echo Done!
echo.

REM Step 6: Test Installation
echo [6/6] Testing installation...
target\release\hermes.exe --version
if %ERRORLEVEL% neq 0 (
    echo ERROR: Hermes test failed!
    pause
    exit /b 1
)
echo Done!
echo.

echo ========================================
echo  SETUP COMPLETE!
echo ========================================
echo.
echo Hermes v1.1.0 is ready to use!
echo.
echo Quick Start Commands:
echo   hermes.exe interactive          - Launch interactive mode
echo   hermes.exe send-file file.txt -p Password
echo   hermes.exe send-batch *.pdf -p Password
echo   hermes.exe send-dir C:\folder --recursive -p Password
echo.
echo Configuration file:
echo   %APPDATA%\hermes\config.toml
echo.
echo Vault directories:
echo   C:\hermes_vault\inbox
echo   C:\hermes_vault\outbox
echo   C:\hermes_vault\files
echo.
pause
