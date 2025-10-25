@echo off
REM Git push script for Hermes v1.0.0

echo ========================================
echo HERMES - Git Push Script
echo ========================================
echo.

echo Step 1: Checking git status...
git status
echo.

echo Step 2: Adding all files...
git add .
echo.

echo Step 3: Committing changes...
git commit -m "Release v1.0.0: Multi-recipient encryption with RSA+AES hybrid"
echo.

echo Commit details:
echo - Multi-recipient RSA+AES hybrid encryption
echo - Self-destruct timer (TTL-based)
echo - Custom binary protocol
echo - RSA key management
echo - GZIP compression
echo - SHA-256 integrity verification
echo - Backward compatibility
echo.

echo Step 4: Pushing to GitHub...
git push origin main
echo.

echo ========================================
echo Done! Check: https://github.com/chronocoders/hermes
echo ========================================
