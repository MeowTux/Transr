#!/bin/bash
# Build script for TransR in Termux

set -e

echo "=========================================="
echo "TransR Build Script for Termux"
echo "=========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}✗ Rust not found!${NC}"
    echo "Install Rust with:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo -e "${GREEN}✓ Rust found: $(rustc --version)${NC}"

# Check if Python is installed
if ! command -v python &> /dev/null; then
    echo -e "${RED}✗ Python not found!${NC}"
    echo "Install Python with:"
    echo "  pkg install python"
    exit 1
fi

echo -e "${GREEN}✓ Python found: $(python --version)${NC}"

# Check if maturin is installed
if ! command -v maturin &> /dev/null; then
    echo -e "${YELLOW}⚠ Maturin not found. Installing...${NC}"
    pip install maturin
fi

echo -e "${GREEN}✓ Maturin found: $(maturin --version)${NC}"
echo ""

# Build options
BUILD_TYPE="${1:-release}"

if [ "$BUILD_TYPE" = "dev" ]; then
    echo "Building in development mode..."
    maturin develop
elif [ "$BUILD_TYPE" = "release" ]; then
    echo "Building in release mode (optimized)..."
    maturin develop --release --strip
elif [ "$BUILD_TYPE" = "wheel" ]; then
    echo "Building wheel package..."
    maturin build --release --strip
    echo ""
    echo "Wheel file created in: target/wheels/"
    echo "Install with: pip install target/wheels/*.whl"
else
    echo -e "${RED}✗ Invalid build type: $BUILD_TYPE${NC}"
    echo "Usage: ./build.sh [dev|release|wheel]"
    exit 1
fi

echo ""
echo -e "${GREEN}=========================================="
echo "✓ Build completed successfully!"
echo "==========================================${NC}"
echo ""
echo "Test the installation with:"
echo "  python -c 'from transR import PipelineR, Feature; print(\"✓ TransR imported successfully\")'"
echo ""
echo "Run tests:"
echo "  python python/test/test.py"
echo "  python python/test/TestAll.py"
