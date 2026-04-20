@echo off
REM Rust and Cargo Installer for Windows
REM This script downloads and installs Rust using rustup

echo ========================================
echo Rust and Cargo Installer for Windows
echo ========================================
echo.

REM Check if Rust is already installed
where cargo >nul 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Rust is already installed!
    cargo --version
    rustc --version
    echo.
    echo Installation skipped. Rust is already available.
    pause
    exit /b 0
)

echo Rust is not installed. Starting installation...
echo.

REM Create a temporary directory for the installer
set TEMP_DIR=%TEMP%\rust_installer
if not exist "%TEMP_DIR%" mkdir "%TEMP_DIR%"

REM Download rustup-init.exe
echo Downloading rustup-init.exe...
set RUSTUP_URL=https://win.rustup.rs/x86_64
set RUSTUP_EXE=%TEMP_DIR%\rustup-init.exe

powershell -Command "Invoke-WebRequest -Uri '%RUSTUP_URL%' -OutFile '%RUSTUP_EXE%' -UseBasicParsing"
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Failed to download rustup-init.exe
    echo Please check your internet connection and try again.
    pause
    exit /b 1
)

echo Download completed successfully.
echo.

REM Run rustup-init.exe
echo Running Rust installer...
echo.
echo The installer will ask you a few questions.
echo Recommended answers:
echo   1) Proceed with installation (default)
echo   2) Default installation (default)
echo   3) Add PATH variable (default)
echo.

"%RUSTUP_EXE%" -y --default-toolchain stable

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo ERROR: Rust installation failed.
    echo Please try running the installer manually.
    pause
    exit /b 1
)

echo.
echo Installation completed successfully!
echo.

REM Refresh PATH for the current session
for /f "delims=" %%i in ('rustup show home') do set RUST_HOME=%%i
set PATH=%RUST_HOME%\bin;%PATH%

REM Verify installation
echo Verifying installation...
cargo --version
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo WARNING: cargo command not found in PATH.
    echo You may need to restart your terminal or add Rust to your PATH manually.
    echo Rust is installed at: %RUST_HOME%
)

rustc --version
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo WARNING: rustc command not found in PATH.
    echo You may need to restart your terminal or add Rust to your PATH manually.
)

echo.
echo ========================================
echo Installation Summary
echo ========================================
echo Rust has been installed successfully!
echo.
echo To use Rust in your current terminal, you may need to:
echo   1. Close and reopen this terminal
echo   2. OR run: refreshenv (if using PowerShell with Chocolatey)
echo   3. OR manually add Rust to your PATH
echo.
echo Rust installation directory: %RUST_HOME%
echo.

REM Clean up temporary files
echo Cleaning up temporary files...
if exist "%TEMP_DIR%" rmdir /s /q "%TEMP_DIR%"

echo Done!
echo.
pause
