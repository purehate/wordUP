@echo off
REM WORD UP Rust Build Script for Windows
REM Cross-platform build script with error handling and optimization

echo ============================================================
echo     ██╗    ██╗ ██████╗ ██████╗ ██████╗     ██╗   ██╗██████╗ 
echo     ██║    ██║██╔═══██╗██╔══██╗██╔══██╗    ██║   ██║██╔══██╗
echo     ██║ █╗ ██║██║   ██║██████╔╝██║  ██║    ██║   ██║██████╔╝
echo     ██║███╗██║██║   ██║██╔══██╗██║  ██║    ██║   ██║██╔═══╝ 
echo     ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝    ╚██████╔╝██║     
echo      ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═════╝      ╚═════╝ ╚═╝     
echo ============================================================
echo 🚀 Advanced Business Wordlist Generator (Rust Edition)
echo ⚡ High-Performance • Memory-Safe • Cross-Platform
echo ============================================================

REM Check if Rust is installed
where cargo >nul 2>nul
if %ERRORLEVEL% neq 0 (
    echo ❌ Error: Rust is not installed!
    echo.
    echo Please install Rust from https://rustup.rs/
    echo.
    echo Quick install:
    echo   Download and run rustup-init.exe from https://rustup.rs/
    echo   Or run: Invoke-WebRequest -Uri "https://win.rustup.rs/" -OutFile "rustup-init.exe"
    echo   Then: .\rustup-init.exe
    pause
    exit /b 1
)

REM Check Rust version
for /f "tokens=2" %%i in ('rustc --version') do set RUST_VERSION=%%i
echo Rust version: %RUST_VERSION%

REM Clean previous build
echo Cleaning previous build...
cargo clean

REM Build in release mode for maximum performance
echo Compiling in release mode...
cargo build --release

if %ERRORLEVEL% equ 0 (
    echo.
    echo ✅ Build successful!
    echo.
    
    REM Get binary info
    set BINARY_PATH=.\target\release\word-up.exe
    
    if exist "%BINARY_PATH%" (
        echo Binary location: %BINARY_PATH%
        echo.
        
        echo Usage examples:
        echo   %BINARY_PATH% acme
        echo   %BINARY_PATH% acme.com --extract-emails --verbose
        echo   %BINARY_PATH% "acme ink" --workers 50 --timeout 15
        echo.
        echo For help: %BINARY_PATH% --help
        echo.
        echo Performance tips:
        echo   - Use --workers 8 for maximum performance (adjust based on CPU cores)
        echo   - Use --timeout 30 for slow networks
        echo   - Use --extract-emails for email discovery
        echo   - Use --verbose for detailed output
    ) else (
        echo ❌ Error: Binary not found at expected location!
        pause
        exit /b 1
    )
) else (
    echo ❌ Build failed!
    echo.
    echo Common solutions:
    echo 1. Install Visual Studio Build Tools
    echo 2. Update Rust: rustup update
    echo 3. Clean and rebuild: cargo clean ^&^& cargo build --release
    echo 4. Check for network issues
    echo 5. Run as Administrator if needed
    pause
    exit /b 1
)

echo ============================================================
echo Build complete! Happy wordlist generating! 🚀
echo ============================================================
pause
