# Installation Guide for Termux

Complete installation guide untuk TransR di Termux (Android).

## 📱 Prerequisites

1. **Termux App**
   - Install dari F-Droid atau GitHub
   - Don't use Google Play Store version (outdated)

2. **Storage Permission**
   ```bash
   termux-setup-storage
   ```

3. **Update Packages**
   ```bash
   pkg update && pkg upgrade
   ```

## 🔧 Step-by-Step Installation

### Step 1: Install Required Packages

```bash
# Install essential packages
pkg install python python-dev clang binutils

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source cargo environment
source $HOME/.cargo/env

# Verify installation
rustc --version
python --version
```

### Step 2: Install Python Build Tools

```bash
# Install pip packages
pip install --upgrade pip
pip install maturin
```

### Step 3: Clone or Extract Project

```bash
# If you have the project in storage
cd /sdcard/Download/transR-project

# Or create from scratch
mkdir -p ~/transR-project
cd ~/transR-project
# Copy all project files here
```

### Step 4: Build TransR

```bash
# Make build script executable
chmod +x build.sh

# Build in release mode (recommended)
./build.sh release

# Or build in development mode (faster compile, slower runtime)
./build.sh dev

# Or build wheel package
./build.sh wheel
```

### Step 5: Verify Installation

```bash
# Quick test
python -c "from transR import PipelineR, Feature; print('✓ TransR works!')"

# Run test suite
python python/test/test.py
```

## 🚀 Quick Start After Installation

```python
from transR import PipelineR, Feature

# Create pipeline
p = PipelineR()
p.set_verbose(2)
p.print("Hello from Termux!")
p.system()
p.run()

# Use features
f = Feature()
info = f.system_info()
print(f"Running on: {info['os_name']}")
```

## 🔨 Build Options

### Development Build
- Faster compilation
- Slower execution
- Includes debug symbols

```bash
./build.sh dev
```

### Release Build (Recommended)
- Slower compilation
- Faster execution
- Optimized binary

```bash
./build.sh release
```

### Wheel Package
- Create distributable package
- Can be installed on other devices

```bash
./build.sh wheel
pip install target/wheels/*.whl
```

## ⚠️ Troubleshooting

### Issue: "command not found: maturin"

**Solution:**
```bash
pip install --upgrade maturin
```

### Issue: "rustc not found"

**Solution:**
```bash
source $HOME/.cargo/env
# Add to .bashrc for permanent:
echo 'source $HOME/.cargo/env' >> ~/.bashrc
```

### Issue: Compilation errors

**Solution:**
```bash
# Clean and rebuild
cargo clean
./build.sh release
```

### Issue: Python can't import transR

**Solution:**
```bash
# Make sure you're in the project directory
cd ~/transR-project
./build.sh release

# Or check Python path
python -c "import sys; print(sys.path)"
```

### Issue: Network features don't work

**Solution:**
- Check internet connection
- Some features require network access
- Try: `ping 8.8.8.8`

### Issue: Permission denied for port scanning

**Solution:**
- Port scanning requires network permission
- Some ports may be blocked by Termux
- Test with localhost first: `127.0.0.1`

## 📦 Dependency Issues

If you encounter dependency issues:

```bash
# Update Rust
rustup update stable

# Update packages
pkg update && pkg upgrade

# Reinstall Python development headers
pkg reinstall python-dev

# Clean and rebuild
cargo clean
rm -rf target/
./build.sh release
```

## 💾 Storage Considerations

TransR compilation requires:
- ~500MB for Rust toolchain
- ~200MB for dependencies
- ~100MB for build artifacts

Total: ~800MB free space needed

## 🔄 Updating TransR

To update to a new version:

```bash
cd ~/transR-project
git pull  # if using git
cargo clean
./build.sh release
```

## 🧪 Running Tests

After installation:

```bash
# Basic tests
python python/test/test.py

# Function tests
python python/test/MyFunc.py

# Plugin tests
python python/test/MyPlugins.py

# All tests
python python/test/TestAll.py

# Unit tests
python python/Unit/Unitesting.py
```

## 📚 Next Steps

1. Read `README.md` for usage examples
2. Check `DOC-FUNC.md` for API documentation
3. See `python/rd.md` for quick reference
4. Try example scripts in `python/test/`

## 🆘 Getting Help

If you encounter issues:

1. Check error messages carefully
2. Verify all prerequisites are installed
3. Try clean rebuild: `cargo clean && ./build.sh release`
4. Check Termux storage permissions
5. Ensure sufficient storage space

## ✅ Verification Checklist

After installation, verify:

- [ ] `rustc --version` works
- [ ] `python --version` works  
- [ ] `maturin --version` works
- [ ] `python -c "from transR import PipelineR"` works
- [ ] `python python/test/test.py` passes
- [ ] System info: `f = Feature(); f.system_info()` works

If all checks pass, TransR is ready to use! 🎉

## 📱 Performance Notes

On Termux/Android:
- Compilation takes 5-15 minutes depending on device
- First run may be slower (loading shared libraries)
- Network features depend on device network capabilities
- Some system features may have limited access

## 🔐 Security Notes

- Port scanning: Only scan systems you own
- Vulnerability scanning: Requires permission
- Network features: Uses device network connection
- System monitoring: Limited to Termux environment

---

**Happy Coding with TransR on Termux! 🚀**
