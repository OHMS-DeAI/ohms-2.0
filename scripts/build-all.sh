#!/bin/bash

# OHMS 2.0 Comprehensive Build Script
# Builds all components with proper dependency management and Candid generation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[BUILD]${NC} $1"
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

print_step() {
    echo -e "${PURPLE}[STEP]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "package.json" ] || [ ! -f "dfx.json" ]; then
    print_error "Please run this script from the root of the ohms-2.0 monorepo"
    exit 1
fi

print_status "Building OHMS 2.0 Complete Ecosystem..."

# Step 1: Clean previous builds
print_step "Cleaning previous builds..."
cargo clean
rm -rf target/
rm -rf .dfx/
find . -name "node_modules" -type d -exec rm -rf {} + 2>/dev/null || true
find . -name "dist" -type d -exec rm -rf {} + 2>/dev/null || true
find . -name "out" -type d -exec rm -rf {} + 2>/dev/null || true

# Step 2: Install all dependencies
print_step "Installing dependencies..."

# Install Rust components
print_status "Installing Rust dependencies..."
cargo fetch

# Install Node.js dependencies
print_status "Installing Node.js dependencies..."
npm install --legacy-peer-deps

# Install pnpm for ohms-ui if not already installed
if ! command -v pnpm &> /dev/null; then
    print_status "Installing pnpm..."
    npm install -g pnpm
fi

cd ohms-ui && pnpm install && cd ..
cd ohms-website && npm install --legacy-peer-deps && cd ..

# Step 3: Build shared types first
print_step "Building shared types..."
cd shared-types
npx tsc --noEmit # Type check
cd ..

# Step 4: Build Rust workspace (canisters)
print_step "Building Rust canisters..."

# Build ohms-shared first (dependency for all others)
print_status "Building ohms-shared..."
cargo build -p ohms-shared --release

# Build ohms-adaptq (NOVAQ compression engine)
print_status "Building NOVAQ compression engine..."
cargo build -p ohms-adaptq --release

# Build individual canisters in dependency order
print_status "Building ohms-model canister..."
cargo build -p ohms-model --release --target wasm32-unknown-unknown

print_status "Building ohms-econ canister..."
cargo build -p ohms_econ --release --target wasm32-unknown-unknown

print_status "Building ohms-agent canister..."
cargo build -p ohms_agent --release --target wasm32-unknown-unknown

print_status "Building ohms-coordinator canister..."
cargo build -p ohms_coordinator --release --target wasm32-unknown-unknown

# Step 5: Generate Candid interfaces
print_step "Generating Candid interfaces..."

# Check if candid-extractor is installed
if ! command -v candid-extractor &> /dev/null; then
    print_status "Installing candid-extractor..."
    cargo install candid-extractor
fi

# Generate Candid files from compiled Wasm
print_status "Extracting Candid interfaces..."

candid-extractor target/wasm32-unknown-unknown/release/ohms_model.wasm > ohms-model/src/ohms_model.did
candid-extractor target/wasm32-unknown-unknown/release/ohms_econ.wasm > ohms-econ/src/ohms_econ.did
candid-extractor target/wasm32-unknown-unknown/release/ohms_agent.wasm > ohms-agent/src/ohms_agent.did
candid-extractor target/wasm32-unknown-unknown/release/ohms_coordinator.wasm > ohms-coordinator/src/ohms_coordinator.did

# Step 6: Generate TypeScript declarations from Candid
print_step "Generating TypeScript declarations..."

# Check if didc is installed
if ! command -v didc &> /dev/null; then
    print_status "Installing didc..."
    # Download didc binary
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        wget https://github.com/dfinity/candid/releases/latest/download/didc-linux64 -O /tmp/didc
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        wget https://github.com/dfinity/candid/releases/latest/download/didc-macos -O /tmp/didc
    else
        print_error "Unsupported OS for didc installation"
        exit 1
    fi
    chmod +x /tmp/didc
    sudo mv /tmp/didc /usr/local/bin/didc
fi

# Generate TypeScript bindings
print_status "Generating TypeScript bindings..."
mkdir -p ohms-ui/src/declarations
mkdir -p ohms-website/src/declarations

# Generate for ohms-ui
didc bind ohms-model/src/ohms_model.did --target ts > ohms-ui/src/declarations/ohms_model.ts
didc bind ohms-econ/src/ohms_econ.did --target ts > ohms-ui/src/declarations/ohms_econ.ts
didc bind ohms-agent/src/ohms_agent.did --target ts > ohms-ui/src/declarations/ohms_agent.ts
didc bind ohms-coordinator/src/ohms_coordinator.did --target ts > ohms-ui/src/declarations/ohms_coordinator.ts

# Generate for ohms-website
didc bind ohms-model/src/ohms_model.did --target ts > ohms-website/src/declarations/ohms_model.ts
didc bind ohms-econ/src/ohms_econ.did --target ts > ohms-website/src/declarations/ohms_econ.ts
didc bind ohms-agent/src/ohms_agent.did --target ts > ohms-website/src/declarations/ohms_agent.ts
didc bind ohms-coordinator/src/ohms_coordinator.did --target ts > ohms-website/src/declarations/ohms_coordinator.ts

# Step 7: Build frontend applications
print_step "Building frontend applications..."

# Build ohms-ui
print_status "Building OHMS UI..."
cd ohms-ui
pnpm run build
cd ..

# Build ohms-website
print_status "Building OHMS Website..."
cd ohms-website
npm run build
npm run export
cd ..

# Step 8: Optimize WASM binaries
print_step "Optimizing WASM binaries..."

# Check if wasm-opt is installed
if ! command -v wasm-opt &> /dev/null; then
    print_status "Installing wasm-opt..."
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        sudo apt-get update && sudo apt-get install -y binaryen
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        brew install binaryen
    else
        print_warning "wasm-opt not available, skipping optimization"
    fi
fi

if command -v wasm-opt &> /dev/null; then
    print_status "Optimizing WASM files..."
    wasm-opt -Oz target/wasm32-unknown-unknown/release/ohms_model.wasm -o target/wasm32-unknown-unknown/release/ohms_model_opt.wasm
    wasm-opt -Oz target/wasm32-unknown-unknown/release/ohms_econ.wasm -o target/wasm32-unknown-unknown/release/ohms_econ_opt.wasm
    wasm-opt -Oz target/wasm32-unknown-unknown/release/ohms_agent.wasm -o target/wasm32-unknown-unknown/release/ohms_agent_opt.wasm
    wasm-opt -Oz target/wasm32-unknown-unknown/release/ohms_coordinator.wasm -o target/wasm32-unknown-unknown/release/ohms_coordinator_opt.wasm
    
    # Replace original files with optimized versions
    mv target/wasm32-unknown-unknown/release/ohms_model_opt.wasm target/wasm32-unknown-unknown/release/ohms_model.wasm
    mv target/wasm32-unknown-unknown/release/ohms_econ_opt.wasm target/wasm32-unknown-unknown/release/ohms_econ.wasm
    mv target/wasm32-unknown-unknown/release/ohms_agent_opt.wasm target/wasm32-unknown-unknown/release/ohms_agent.wasm
    mv target/wasm32-unknown-unknown/release/ohms_coordinator_opt.wasm target/wasm32-unknown-unknown/release/ohms_coordinator.wasm
fi

# Step 9: Verify build integrity
print_step "Verifying build integrity..."

# Check that all WASM files exist
for canister in ohms_model ohms_econ ohms_agent ohms_coordinator; do
    if [ ! -f "target/wasm32-unknown-unknown/release/${canister}.wasm" ]; then
        print_error "Missing WASM file: ${canister}.wasm"
        exit 1
    fi
    print_success "✓ ${canister}.wasm built successfully"
done

# Check that frontend builds exist
if [ ! -d "ohms-ui/dist" ]; then
    print_error "OHMS UI build not found"
    exit 1
fi
print_success "✓ OHMS UI built successfully"

if [ ! -d "ohms-website/out" ]; then
    print_error "OHMS Website build not found"
    exit 1
fi
print_success "✓ OHMS Website built successfully"

# Check that Candid files are generated
for canister in ohms_model ohms_econ ohms_agent ohms_coordinator; do
    if [ ! -f "${canister//_/-}/src/${canister}.did" ]; then
        print_error "Missing Candid file: ${canister}.did"
        exit 1
    fi
    print_success "✓ ${canister}.did generated successfully"
done

# Step 10: Generate build report
print_step "Generating build report..."

BUILD_TIME=$(date)
BUILD_REPORT="build-report-$(date +%Y%m%d-%H%M%S).txt"

cat > $BUILD_REPORT << EOF
OHMS 2.0 Build Report
Generated: $BUILD_TIME

=== Build Summary ===
✓ Rust workspace compiled successfully
✓ All canisters built (ohms-model, ohms-econ, ohms-agent, ohms-coordinator)
✓ NOVAQ compression engine built (ohms-adaptq)
✓ Candid interfaces generated
✓ TypeScript bindings generated
✓ Frontend applications built (ohms-ui, ohms-website)
✓ WASM binaries optimized

=== File Sizes ===
EOF

echo "=== WASM File Sizes ===" >> $BUILD_REPORT
for canister in ohms_model ohms_econ ohms_agent ohms_coordinator; do
    size=$(du -h "target/wasm32-unknown-unknown/release/${canister}.wasm" | cut -f1)
    echo "${canister}.wasm: $size" >> $BUILD_REPORT
done

echo "" >> $BUILD_REPORT
echo "=== Frontend Build Sizes ===" >> $BUILD_REPORT
ui_size=$(du -sh ohms-ui/dist | cut -f1)
website_size=$(du -sh ohms-website/out | cut -f1)
echo "ohms-ui/dist: $ui_size" >> $BUILD_REPORT
echo "ohms-website/out: $website_size" >> $BUILD_REPORT

echo "" >> $BUILD_REPORT
echo "=== NOVAQ Binary ===" >> $BUILD_REPORT
novaq_size=$(du -h target/release/novaq 2>/dev/null | cut -f1 || echo "Not built")
echo "novaq binary: $novaq_size" >> $BUILD_REPORT

print_success "Build report saved to: $BUILD_REPORT"

print_success "🎉 OHMS 2.0 build completed successfully!"
print_status "Ready for deployment with: ./scripts/deploy-all.sh"
print_status "Start development with: ./scripts/dev-all.sh"

# Display quick deployment instructions
echo ""
echo "=== Quick Start ==="
echo "1. Start DFX: dfx start --clean"
echo "2. Deploy: ./scripts/deploy-all.sh local"
echo "3. Test: ./scripts/test-all.sh"
echo ""
