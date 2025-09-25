#!/bin/bash

# WORD UP Rust Build Script
# Cross-platform build script with error handling and optimization

set -e  # Exit on any error

cat <<'EOF'

             .--.           .---.        .-.
         .---|--|   .-.     | W |  .---. |~|    .--.
      .--|===|  |---|_|--.__| O |--|:::| |~|-==-|==|---._.
      |WO|RD |UP|===| |~~|%%| R |--|   |_|~|WORD UP|   |_|
      |  |   |  |===| |==|  | D |  |:::|=| |    |  |---|=|
      |  |   |  |   |_|__|  | U |__|   | | |    |  |___| |
      |~~|===|--|===|~|~~|%%| P |--|:::|=|~|----|==|---|=|
      ^--^---'--^---^-^--^--^---'--^---^-^-^-==-^--^---^-'
EOF
echo "============================================================"
echo "Wordlist Operations & Reconnaissance Data Ultimate Profiling"
echo "⚡ High-Performance • Memory-Safe • Cross-Platform"
echo "============================================================"

# Detect platform
OS="unknown"
case "$(uname -s)" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    CYGWIN*)    OS="windows";;
    MINGW*)     OS="windows";;
    MSYS*)      OS="windows";;
esac

echo "Detected platform: $OS"

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust is not installed!"
    echo ""
    echo "Please install Rust from https://rustup.rs/"
    echo ""
    echo "Quick install:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    echo "  source ~/.cargo/env"
    exit 1
fi

# Check Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "Rust version: $RUST_VERSION"

# Check if version is sufficient (1.70+)
REQUIRED_VERSION="1.70.0"
if [ "$(printf '%s\n' "$REQUIRED_VERSION" "$RUST_VERSION" | sort -V | head -n1)" != "$REQUIRED_VERSION" ]; then
    echo "❌ Error: Rust version $RUST_VERSION is too old!"
    echo "Required: $REQUIRED_VERSION or newer"
    echo "Update with: rustup update"
    exit 1
fi

# Check for system dependencies
echo "Checking system dependencies..."

if [ "$OS" = "linux" ]; then
    # Check for build essentials
    if ! command -v gcc &> /dev/null; then
        echo "⚠️  Warning: gcc not found. You may need to install build-essential:"
        echo "   sudo apt-get install build-essential  # Ubuntu/Debian"
        echo "   sudo yum groupinstall \"Development Tools\"  # CentOS/RHEL"
    fi
    
    # Check for OpenSSL
    if ! pkg-config --exists openssl; then
        echo "⚠️  Warning: OpenSSL development headers not found."
        echo "   Install with: sudo apt-get install libssl-dev pkg-config"
    fi
elif [ "$OS" = "macos" ]; then
    # Check for Xcode command line tools
    if ! command -v xcode-select &> /dev/null || ! xcode-select -p &> /dev/null; then
        echo "⚠️  Warning: Xcode command line tools not found."
        echo "   Install with: xcode-select --install"
    fi
fi

# Clean previous build
echo "Cleaning previous build..."
cargo clean

# Build in release mode for maximum performance
echo "Compiling in release mode..."

# Set optimization flags based on platform
if [ "$OS" = "macos" ]; then
    echo "Using native CPU optimization for macOS..."
    RUSTFLAGS="-C target-cpu=native" cargo build --release
else
    cargo build --release
fi

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    
    # Get binary info
    BINARY_PATH="./target/release/word-up"
    if [ "$OS" = "windows" ]; then
        BINARY_PATH="./target/release/word-up.exe"
    fi
    
    if [ -f "$BINARY_PATH" ]; then
        BINARY_SIZE=$(ls -lh "$BINARY_PATH" | awk '{print $5}')
        echo "Binary location: $BINARY_PATH"
        echo "Binary size: $BINARY_SIZE"
        echo ""
        
        # Make executable on Unix systems
        if [ "$OS" != "windows" ]; then
            chmod +x "$BINARY_PATH"
        fi
        
        echo "Usage examples:"
        echo "  $BINARY_PATH acme"
        echo "  $BINARY_PATH acme.com --extract-emails --verbose"
        echo "  $BINARY_PATH \"acme ink\" --workers 50 --timeout 15"
        echo ""
        echo "For help: $BINARY_PATH --help"
        echo ""
        echo "Performance tips:"
        echo "  - Use --workers \$(nproc) for maximum performance"
        echo "  - Use --timeout 30 for slow networks"
        echo "  - Use --extract-emails for email discovery"
        echo "  - Use --verbose for detailed output"
    else
        echo "❌ Error: Binary not found at expected location!"
        exit 1
    fi
else
    echo "❌ Build failed!"
    echo ""
    echo "Common solutions:"
    echo "1. Install system dependencies (see README.md)"
    echo "2. Update Rust: rustup update"
    echo "3. Clean and rebuild: cargo clean && cargo build --release"
    echo "4. Check for network issues"
    exit 1
fi

echo "============================================================"
echo "Build complete!"
echo "============================================================"
