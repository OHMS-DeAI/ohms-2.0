#!/bin/bash

# OHMS 2.0 Monorepo Setup Script
# Sets up the complete development environment

set -e

echo "🚀 Setting up OHMS 2.0 Monorepo Development Environment..."

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

# Check prerequisites
print_status "Checking prerequisites..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed. Please install Node.js 18+ first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    print_error "npm is not installed. Please install npm first."
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed. Please install Rust first."
    exit 1
fi

# Check if DFX is installed
if ! command -v dfx &> /dev/null; then
    print_warning "DFX is not installed. Installing DFX..."
    sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
fi

print_success "Prerequisites check passed!"

# Install Node.js dependencies
print_status "Installing Node.js dependencies..."
npm run install:all -- --legacy-peer-deps

# Install Rust dependencies
print_status "Installing Rust dependencies..."
cargo build

# Setup git hooks (if needed)
if [ -d ".git" ]; then
    print_status "Setting up git hooks..."
    # Add any git hooks setup here if needed
fi

print_success "OHMS 2.0 monorepo setup completed!"
echo ""
echo "🎯 Next steps:"
echo "1. Start DFX: npm run dfx:start"
echo "2. Deploy locally: npm run dfx:deploy:local"
echo "3. Start UI: npm run dev:ui"
echo "4. Start website: npm run dev:website"
echo ""
echo "📚 Useful commands:"
echo "- npm run build:all     # Build all components"
echo "- npm run test:ui       # Run UI tests"
echo "- npm run canister:status # Check canister status"
echo ""
print_success "Happy coding! 🚀"
