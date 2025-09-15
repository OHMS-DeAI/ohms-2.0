#!/bin/bash

# OHMS 2.0 Monorepo Deployment Script
# Deploys all canisters in the correct dependency order

set -e

echo "🚀 Deploying OHMS 2.0 Ecosystem..."

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
if [ ! -f "dfx.json" ]; then
    print_error "Please run this script from the root of the ohms-2.0 monorepo"
    exit 1
fi

# Check if DFX is running
if ! dfx ping >/dev/null 2>&1; then
    print_error "DFX is not running. Please start it with: dfx start --clean"
    exit 1
fi

# Default to local network if not specified
NETWORK=${1:-local}

print_status "Deploying to network: $NETWORK"

# Deploy canisters in dependency order
print_status "Deploying OHMS Model Repository..."
dfx deploy --network $NETWORK ohms_model

print_status "Deploying OHMS Economics..."
dfx deploy --network $NETWORK ohms_econ

print_status "Deploying OHMS Agent Factory..."
dfx deploy --network $NETWORK ohms_agent

print_status "Deploying OHMS Coordinator..."
dfx deploy --network $NETWORK ohms_coordinator

print_status "Deploying OHMS UI..."
dfx deploy --network $NETWORK ohms_ui

print_status "Deploying OHMS Website..."
dfx deploy --network $NETWORK ohms_website

print_success "OHMS 2.0 deployment completed successfully!"
print_status "Canister IDs saved to canister_ids.json files"

# Display canister status
print_status "Current canister status:"
dfx canister --network $NETWORK status --all

if [ "$NETWORK" = "local" ]; then
    print_status "Local deployment URLs:"
    OHMS_UI_ID=$(dfx canister --network local id ohms_ui 2>/dev/null || echo "not deployed")
    OHMS_WEBSITE_ID=$(dfx canister --network local id ohms_website 2>/dev/null || echo "not deployed")

    if [ "$OHMS_UI_ID" != "not deployed" ]; then
        echo -e "${GREEN}OHMS UI:${NC} http://$OHMS_UI_ID.localhost:4943/"
    fi

    if [ "$OHMS_WEBSITE_ID" != "not deployed" ]; then
        echo -e "${GREEN}OHMS Website:${NC} http://$OHMS_WEBSITE_ID.localhost:4943/"
    fi
fi
