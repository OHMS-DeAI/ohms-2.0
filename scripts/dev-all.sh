#!/bin/bash

# OHMS 2.0 Development Environment Script
# Sets up complete development environment with hot reloading

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[DEV]${NC} $1"
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

print_status "Starting OHMS 2.0 Development Environment..."

# Function to cleanup background processes
cleanup() {
    print_status "Cleaning up background processes..."
    jobs -p | xargs -r kill
    exit
}

trap cleanup EXIT

# Step 1: Check prerequisites
print_step "Checking prerequisites..."

# Check if DFX is installed
if ! command -v dfx &> /dev/null; then
    print_error "DFX is not installed. Please install it first."
    exit 1
fi

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    print_error "Node.js is not installed. Please install it first."
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Rust is not installed. Please install it first."
    exit 1
fi

print_success "All prerequisites satisfied"

# Step 2: Install dependencies if needed
print_step "Installing dependencies..."

if [ ! -d "node_modules" ]; then
    print_status "Installing npm dependencies..."
    npm install --legacy-peer-deps
fi

if [ ! -d "ohms-ui/node_modules" ]; then
    print_status "Installing ohms-ui dependencies..."
    cd ohms-ui && pnpm install && cd ..
fi

if [ ! -d "ohms-website/node_modules" ]; then
    print_status "Installing ohms-website dependencies..."
    cd ohms-website && npm install --legacy-peer-deps && cd ..
fi

# Step 3: Start DFX replica
print_step "Starting DFX replica..."

if dfx ping > /dev/null 2>&1; then
    print_warning "DFX is already running"
else
    print_status "Starting DFX replica in background..."
    dfx start --clean --background
    
    # Wait for DFX to be ready
    print_status "Waiting for DFX to be ready..."
    while ! dfx ping > /dev/null 2>&1; do
        sleep 1
    done
fi

print_success "DFX replica is ready"

# Step 4: Build and deploy canisters
print_step "Building and deploying canisters..."

# Build the project first
print_status "Building project..."
./scripts/build-all.sh

# Deploy canisters
print_status "Deploying canisters..."
dfx deploy --network local

print_success "Canisters deployed successfully"

# Step 5: Start file watchers and development servers
print_step "Starting development servers..."

# Create log directory
mkdir -p logs

# Start Rust file watcher for canisters
print_status "Starting Rust file watcher..."
{
    while true; do
        # Watch for changes in Rust files
        find ohms-*/ -name "*.rs" -newer .rust_build_timestamp 2>/dev/null | head -1 | read -r changed_file
        if [ -n "$changed_file" ]; then
            print_status "Rust file changed: $changed_file"
            print_status "Rebuilding canisters..."
            
            # Determine which canister to rebuild based on the changed file
            if [[ "$changed_file" == ohms-model/* ]]; then
                cargo build -p ohms-model --release --target wasm32-unknown-unknown
                dfx deploy ohms_model --network local
            elif [[ "$changed_file" == ohms-agent/* ]]; then
                cargo build -p ohms_agent --release --target wasm32-unknown-unknown
                dfx deploy ohms_agent --network local
            elif [[ "$changed_file" == ohms-coordinator/* ]]; then
                cargo build -p ohms_coordinator --release --target wasm32-unknown-unknown
                dfx deploy ohms_coordinator --network local
            elif [[ "$changed_file" == ohms-econ/* ]]; then
                cargo build -p ohms_econ --release --target wasm32-unknown-unknown
                dfx deploy ohms_econ --network local
            elif [[ "$changed_file" == ohms-shared/* ]]; then
                # Shared library changed, rebuild all canisters
                cargo build --release --target wasm32-unknown-unknown
                dfx deploy --network local
            fi
            
            # Update timestamp
            touch .rust_build_timestamp
        fi
        sleep 2
    done
} > logs/rust-watcher.log 2>&1 &

# Initialize timestamp file
touch .rust_build_timestamp

# Start OHMS UI development server
print_status "Starting OHMS UI development server..."
{
    cd ohms-ui
    pnpm run dev
} > ../logs/ohms-ui-dev.log 2>&1 &

# Start OHMS Website development server
print_status "Starting OHMS Website development server..."
{
    cd ohms-website
    npm run dev
} > ../logs/ohms-website-dev.log 2>&1 &

# Start NOVAQ development watcher
print_status "Starting NOVAQ development watcher..."
{
    while true; do
        find ohms-adaptq/src -name "*.rs" -newer .novaq_build_timestamp 2>/dev/null | head -1 | read -r changed_file
        if [ -n "$changed_file" ]; then
            print_status "NOVAQ file changed: $changed_file"
            print_status "Rebuilding NOVAQ..."
            cd ohms-adaptq
            cargo build --release
            cd ..
            touch .novaq_build_timestamp
        fi
        sleep 2
    done
} > logs/novaq-watcher.log 2>&1 &

# Initialize NOVAQ timestamp
touch .novaq_build_timestamp

# Wait for servers to start
sleep 5

# Step 6: Display development information
print_step "Development environment ready!"

# Get canister IDs
OHMS_UI_ID=$(dfx canister id ohms_ui --network local 2>/dev/null || echo "not deployed")
OHMS_WEBSITE_ID=$(dfx canister id ohms_website --network local 2>/dev/null || echo "not deployed")
OHMS_MODEL_ID=$(dfx canister id ohms_model --network local 2>/dev/null || echo "not deployed")
OHMS_AGENT_ID=$(dfx canister id ohms_agent --network local 2>/dev/null || echo "not deployed")
OHMS_COORDINATOR_ID=$(dfx canister id ohms_coordinator --network local 2>/dev/null || echo "not deployed")
OHMS_ECON_ID=$(dfx canister id ohms_econ --network local 2>/dev/null || echo "not deployed")

echo ""
echo "🎉 OHMS 2.0 Development Environment is Running!"
echo ""
echo "=== Frontend Applications ==="
echo "OHMS UI (Vite):       http://localhost:5173"
echo "OHMS Website (Next):  http://localhost:3000"
echo ""
echo "=== Canister Endpoints ==="
if [ "$OHMS_UI_ID" != "not deployed" ]; then
    echo "OHMS UI Canister:     http://$OHMS_UI_ID.localhost:4943/"
fi
if [ "$OHMS_WEBSITE_ID" != "not deployed" ]; then
    echo "OHMS Website Canister: http://$OHMS_WEBSITE_ID.localhost:4943/"
fi
echo ""
echo "=== Backend Canisters ==="
echo "Model Repository:     $OHMS_MODEL_ID"
echo "Agent Factory:        $OHMS_AGENT_ID"
echo "Coordinator:          $OHMS_COORDINATOR_ID"
echo "Economics:            $OHMS_ECON_ID"
echo ""
echo "=== Development Tools ==="
echo "DFX Dashboard:        http://localhost:4943/_/dashboard"
echo "Candid UI:            http://localhost:4943/_/candid"
echo ""
echo "=== Logs ==="
echo "Rust Watcher:         tail -f logs/rust-watcher.log"
echo "OHMS UI:              tail -f logs/ohms-ui-dev.log"
echo "OHMS Website:         tail -f logs/ohms-website-dev.log"
echo "NOVAQ Watcher:        tail -f logs/novaq-watcher.log"
echo ""
echo "=== Commands ==="
echo "Rebuild all:          ./scripts/build-all.sh"
echo "Redeploy:             dfx deploy --network local"
echo "Run tests:            ./scripts/test-all.sh"
echo "Stop environment:     Ctrl+C"
echo ""

# Step 7: Monitor and display real-time information
print_status "Monitoring development environment..."
print_status "Press Ctrl+C to stop all services"

# Function to show real-time canister status
show_status() {
    while true; do
        sleep 30
        clear
        echo "=== OHMS 2.0 Development Status ==="
        echo "Time: $(date)"
        echo ""
        
        # Check canister status
        echo "=== Canister Health ==="
        for canister in ohms_model ohms_agent ohms_coordinator ohms_econ; do
            status=$(dfx canister status $canister --network local 2>/dev/null | grep "Status:" | cut -d' ' -f2 || echo "unknown")
            echo "$canister: $status"
        done
        echo ""
        
        # Show recent log entries
        echo "=== Recent Activity ==="
        if [ -f "logs/rust-watcher.log" ]; then
            echo "Rust builds:"
            tail -3 logs/rust-watcher.log | grep "Rebuilding\|deployed" || echo "No recent activity"
        fi
        echo ""
        
        # Show file watcher status
        echo "=== File Watchers ==="
        echo "Rust watcher: $(pgrep -f 'rust-watcher' > /dev/null && echo 'running' || echo 'stopped')"
        echo "NOVAQ watcher: $(pgrep -f 'novaq-watcher' > /dev/null && echo 'running' || echo 'stopped')"
        echo ""
        
        echo "Press Ctrl+C to stop development environment"
    done
}

# Run status monitor in background
show_status &

# Wait for interrupt signal
wait
