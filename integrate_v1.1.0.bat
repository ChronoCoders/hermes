@echo off
REM ============================================================
REM HERMES v1.1.0 - Integration Script for Windows
REM This script copies v1.1.0 files to your Hermes project
REM ============================================================

echo.
echo ========================================
echo  HERMES v1.1.0 - FILE INTEGRATION
echo ========================================
echo.

REM Check if outputs directory exists
if not exist "outputs\src\commands" (
    echo ERROR: outputs\src\commands directory not found!
    echo Please ensure you have the v1.1.0 files in the outputs directory.
    pause
    exit /b 1
)

REM Get project path from user
set /p PROJECT_PATH="Enter your Hermes project path (e.g., C:\hermes): "

if "%PROJECT_PATH%"=="" (
    echo ERROR: No project path specified!
    pause
    exit /b 1
)

if not exist "%PROJECT_PATH%\Cargo.toml" (
    echo ERROR: Cargo.toml not found in %PROJECT_PATH%
    echo Please enter a valid Hermes project directory.
    pause
    exit /b 1
)

echo.
echo Project path: %PROJECT_PATH%
echo.
set /p CONFIRM="Continue? (Y/N): "
if /i not "%CONFIRM%"=="Y" (
    echo Cancelled.
    pause
    exit /b 0
)

echo.
echo [1/3] Creating directories...
mkdir "%PROJECT_PATH%\src\commands" 2>nul
echo Done!

echo.
echo [2/3] Copying new command files...
copy /Y "outputs\src\commands\send_batch.rs" "%PROJECT_PATH%\src\commands\"
copy /Y "outputs\src\commands\send_dir.rs" "%PROJECT_PATH%\src\commands\"
copy /Y "outputs\src\commands\recv_batch.rs" "%PROJECT_PATH%\src\commands\"
copy /Y "outputs\src\commands\interactive.rs" "%PROJECT_PATH%\src\commands\"
echo Done!

echo.
echo [3/3] Copying updated files...
copy /Y "outputs\src\commands\mod.rs" "%PROJECT_PATH%\src\commands\"
copy /Y "outputs\src\main.rs" "%PROJECT_PATH%\src\"
echo Done!

echo.
echo ========================================
echo  INTEGRATION COMPLETE!
echo ========================================
echo.
echo Files copied to: %PROJECT_PATH%
echo.
echo Next steps:
echo   1. cd %PROJECT_PATH%
echo   2. cargo build --release
echo   3. target\release\hermes.exe interactive
echo.
echo New commands available:
echo   - hermes.exe send-batch
echo   - hermes.exe send-dir
echo   - hermes.exe recv-batch
echo   - hermes.exe interactive
echo.
pause
