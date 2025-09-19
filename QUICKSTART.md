# WORD UP - Wordlist Operations & Reconnaissance Data - Ultimate Profiling - Quick Start Guide

## 🚀 **Get Started in 3 Steps**

### 1. **Install Rust** (if not already installed)
```bash
# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Windows
# Download and run rustup-init.exe from https://rustup.rs/
```

### 2. **Build the Tool**
```bash
# macOS/Linux
./build.sh

# Windows
build.bat
```

### 3. **Run It**
```bash
# Basic usage
./target/release/word-up acme.com

# With all features
./target/release/word-up acme.com --extract-emails --verbose
```

## 📋 **Common Commands**

| Command | Description |
|---------|-------------|
| `./target/release/word-up target.com` | Basic scan |
| `./target/release/word-up target.com -e` | Extract emails |
| `./target/release/word-up target.com -v` | Verbose output |
| `./target/release/word-up target.com -w 50` | 50 concurrent workers |
| `./target/release/word-up target.com -t 30` | 30 second timeout |
| `./target/release/word-up target.com -g 3` | 3-word groups |

## 🎯 **Quick Examples**

### **Fast Scan**
```bash
./target/release/word-up target.com --workers 50 --timeout 5
```

### **Stealth Scan**
```bash
./target/release/word-up target.com --workers 5 --timeout 60
```

### **Email Focus**
```bash
./target/release/word-up target.com --extract-emails --min-word-length 4
```

### **Comprehensive Scan**
```bash
./target/release/word-up target.com \
    --extract-emails \
    --extract-metadata \
    --workers 20 \
    --timeout 15 \
    --verbose
```

## 📁 **Output Files**

After running, you'll get:
- `{company}_{timestamp}_final.txt` - **Main wordlist** (use this!)
- `{company}_{timestamp}_emails.txt` - Email addresses
- `{company}_{timestamp}_groups.txt` - Word combinations
- `{company}_{timestamp}_stats.json` - Statistics

## ⚡ **Performance Tips**

- **Fast**: `--workers 50` (use all CPU cores)
- **Stealth**: `--workers 5 --timeout 60`
- **Memory**: `--workers 10` (lower memory usage)
- **Network**: `--timeout 30` (slow networks)

## 🔧 **Troubleshooting**

**Build fails?**
```bash
# Install dependencies
sudo apt-get install build-essential libssl-dev pkg-config  # Linux
xcode-select --install  # macOS
```

**Permission denied?**
```bash
chmod +x ./target/release/word-up
```

**Slow performance?**
```bash
# Use more workers
./target/release/word-up target.com --workers $(nproc)
```

## 📖 **Need More Help?**

- **Full documentation**: See `README.md`
- **Command help**: `./target/release/word-up --help`
- **Debug mode**: `RUST_LOG=debug ./target/release/word-up target.com -v`

---

**Happy wordlist generating! 🚀**
