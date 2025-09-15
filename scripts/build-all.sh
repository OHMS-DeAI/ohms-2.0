#!/bin/bash

# OHMS 2.0 Monorepo Build Script
# Builds all components of the OHMS ecosystem

set -e

echo "🚀 Building OHMS 2.0 Monorepo..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "package.json" ] || [ ! -f "dfx.json" ]; then
    print_error "Please run this script from the root of the ohms-2.0 monorepo"
    exit 1
fi

# Install Node.js dependencies
print_status "Installing Node.js dependencies..."
npm run install:all

# Build Rust canisters
print_status "Building Rust canisters..."
cargo build --release

# Build frontend assets
print_status "Building frontend assets..."
npm run build:all

# Build NOVAQ compression engine
print_status "Building NOVAQ compression engine..."
cd ohms-adaptq
cargo build --release
cd ..

print_success "OHMS 2.0 build completed successfully!"
print_status "You can now deploy with: npm run dfx:deploy:local"
