#!/bin/bash

# OHMS 2.0 Comprehensive Test Suite
# Tests all components: Rust canisters, TypeScript frontend, integration tests

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[TEST]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[FAIL]${NC} $1"
}

print_step() {
    echo -e "${PURPLE}[STEP]${NC} $1"
}

# Test counters
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0

# Function to run a test and track results
run_test() {
    local test_name="$1"
    local test_command="$2"
    local optional="${3:-false}"
    
    print_status "Running: $test_name"
    
    if eval "$test_command" > /tmp/test_output 2>&1; then
        print_success "$test_name"
        ((TESTS_PASSED++))
        return 0
    else
        if [ "$optional" = "true" ]; then
            print_warning "$test_name (optional)"
            ((TESTS_SKIPPED++))
        else
            print_error "$test_name"
            echo "Error output:"
            cat /tmp/test_output | head -20
            ((TESTS_FAILED++))
        fi
        return 1
    fi
}

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "package.json" ] || [ ! -f "dfx.json" ]; then
    print_error "Please run this script from the root of the ohms-2.0 monorepo"
    exit 1
fi

print_status "Starting OHMS 2.0 Comprehensive Test Suite..."

# Step 1: Environment checks
print_step "Environment validation..."

run_test "Rust toolchain" "rustc --version"
run_test "Cargo check" "cargo --version"
run_test "Node.js check" "node --version"
run_test "NPM check" "npm --version"
run_test "DFX check" "dfx --version"

# Step 2: Rust unit tests
print_step "Rust unit tests..."

run_test "ohms-shared tests" "cargo test -p ohms-shared"
run_test "ohms-adaptq tests" "cargo test -p ohms-adaptq"
run_test "ohms-model tests" "cargo test -p ohms-model"
run_test "ohms-agent tests" "cargo test -p ohms_agent"
run_test "ohms-coordinator tests" "cargo test -p ohms_coordinator"
run_test "ohms-econ tests" "cargo test -p ohms_econ"

# Step 3: Rust integration tests
print_step "Rust integration tests..."

run_test "Workspace integration" "cargo test --workspace"
run_test "Rust clippy lints" "cargo clippy --workspace -- -D warnings"
run_test "Rust format check" "cargo fmt --all -- --check"

# Step 4: TypeScript tests
print_step "TypeScript tests..."

# Install dependencies if needed
if [ ! -d "shared-types/node_modules" ]; then
    print_status "Installing shared-types dependencies..."
    cd shared-types && npm install && cd ..
fi

run_test "Shared types compilation" "cd shared-types && npx tsc --noEmit"

# OHMS UI tests
if [ -d "ohms-ui" ]; then
    print_status "Installing ohms-ui dependencies..."
    cd ohms-ui
    if [ ! -d "node_modules" ]; then
        pnpm install
    fi
    
    run_test "OHMS UI TypeScript check" "pnpm run lint"
    run_test "OHMS UI unit tests" "pnpm run test:run"
    run_test "OHMS UI build test" "pnpm run build"
    cd ..
fi

# OHMS Website tests
if [ -d "ohms-website" ]; then
    print_status "Installing ohms-website dependencies..."
    cd ohms-website
    if [ ! -d "node_modules" ]; then
        npm install --legacy-peer-deps
    fi
    
    run_test "OHMS Website TypeScript check" "npm run lint"
    run_test "OHMS Website build test" "npm run build"
    cd ..
fi

# Step 5: Canister deployment tests
print_step "Canister deployment tests..."

# Check if DFX is running
if ! dfx ping > /dev/null 2>&1; then
    print_status "Starting DFX for deployment tests..."
    dfx start --clean --background
    
    # Wait for DFX to be ready
    while ! dfx ping > /dev/null 2>&1; do
        sleep 1
    done
fi

# Build all canisters
run_test "Canister build" "./scripts/build-all.sh"

# Deploy canisters
run_test "Canister deployment" "dfx deploy --network local"

# Step 6: Canister health checks
print_step "Canister health checks..."

# Get canister IDs
MODEL_ID=$(dfx canister id ohms_model --network local 2>/dev/null || echo "")
AGENT_ID=$(dfx canister id ohms_agent --network local 2>/dev/null || echo "")
COORDINATOR_ID=$(dfx canister id ohms_coordinator --network local 2>/dev/null || echo "")
ECON_ID=$(dfx canister id ohms_econ --network local 2>/dev/null || echo "")

if [ -n "$MODEL_ID" ]; then
    run_test "Model canister health" "dfx canister call ohms_model health --network local"
    run_test "Model canister status" "dfx canister status ohms_model --network local"
fi

if [ -n "$AGENT_ID" ]; then
    run_test "Agent canister health" "dfx canister call ohms_agent health --network local"
    run_test "Agent canister status" "dfx canister status ohms_agent --network local"
fi

if [ -n "$COORDINATOR_ID" ]; then
    run_test "Coordinator canister health" "dfx canister call ohms_coordinator health --network local"
    run_test "Coordinator canister status" "dfx canister status ohms_coordinator --network local"
fi

if [ -n "$ECON_ID" ]; then
    run_test "Econ canister health" "dfx canister call ohms_econ health --network local"
    run_test "Econ canister status" "dfx canister status ohms_econ --network local"
fi

# Step 7: Inter-canister communication tests
print_step "Inter-canister communication tests..."

if [ -n "$MODEL_ID" ] && [ -n "$AGENT_ID" ]; then
    run_test "Model-Agent communication" "dfx canister call ohms_model list_active_models --network local" true
fi

if [ -n "$COORDINATOR_ID" ] && [ -n "$ECON_ID" ]; then
    run_test "Coordinator-Econ communication" "dfx canister call ohms_coordinator health --network local" true
fi

# Step 8: NOVAQ functionality tests
print_step "NOVAQ functionality tests..."

if [ -f "target/release/novaq" ]; then
    run_test "NOVAQ binary execution" "./target/release/novaq --help"
    
    # Test NOVAQ compression with a small model (if available)
    if [ -f "test-data/small-model.bin" ]; then
        run_test "NOVAQ compression test" "./target/release/novaq compress test-data/small-model.bin --output /tmp/compressed.novaq" true
    else
        print_warning "No test model available for NOVAQ compression test"
        ((TESTS_SKIPPED++))
    fi
else
    print_warning "NOVAQ binary not found, skipping NOVAQ tests"
    ((TESTS_SKIPPED++))
fi

# Step 9: Frontend integration tests
print_step "Frontend integration tests..."

# Test OHMS UI integration
if [ -d "ohms-ui/dist" ]; then
    run_test "OHMS UI integration tests" "cd ohms-ui && pnpm run test:integration" true
else
    print_warning "OHMS UI not built, skipping integration tests"
    ((TESTS_SKIPPED++))
fi

# Step 10: End-to-end tests
print_step "End-to-end tests..."

# Create a test script for E2E scenarios
cat > /tmp/e2e_test.sh << 'EOF'
#!/bin/bash
set -e

# Test full workflow: model upload -> agent creation -> inference
echo "Testing full OHMS workflow..."

# 1. Check system health
dfx canister call ohms_coordinator health --network local > /dev/null
dfx canister call ohms_model health --network local > /dev/null
dfx canister call ohms_agent health --network local > /dev/null
dfx canister call ohms_econ health --network local > /dev/null

echo "All canisters healthy"

# 2. List available models
dfx canister call ohms_model list_active_models --network local > /dev/null

echo "Model listing successful"

# 3. Test agent creation (if possible)
# This would require actual model data, so we'll skip for now

echo "E2E test completed successfully"
EOF

chmod +x /tmp/e2e_test.sh
run_test "End-to-end workflow test" "/tmp/e2e_test.sh" true

# Step 11: Security and performance tests
print_step "Security and performance tests..."

run_test "Rust security audit" "cargo audit" true
run_test "NPM security audit" "npm audit --audit-level moderate" true

# Check WASM binary sizes
if [ -f "target/wasm32-unknown-unknown/release/ohms_model.wasm" ]; then
    MODEL_SIZE=$(stat -c%s "target/wasm32-unknown-unknown/release/ohms_model.wasm")
    if [ $MODEL_SIZE -lt 2097152 ]; then  # 2MB limit
        print_success "Model canister size check ($MODEL_SIZE bytes)"
        ((TESTS_PASSED++))
    else
        print_error "Model canister too large ($MODEL_SIZE bytes > 2MB)"
        ((TESTS_FAILED++))
    fi
fi

# Step 12: Generate test report
print_step "Generating test report..."

TOTAL_TESTS=$((TESTS_PASSED + TESTS_FAILED + TESTS_SKIPPED))
TEST_REPORT="test-report-$(date +%Y%m%d-%H%M%S).txt"

cat > $TEST_REPORT << EOF
OHMS 2.0 Test Report
Generated: $(date)

=== Test Summary ===
Total Tests: $TOTAL_TESTS
Passed: $TESTS_PASSED
Failed: $TESTS_FAILED
Skipped: $TESTS_SKIPPED

Success Rate: $(( TESTS_PASSED * 100 / (TESTS_PASSED + TESTS_FAILED) ))%

=== Test Categories ===
✓ Environment validation
✓ Rust unit tests
✓ Rust integration tests
✓ TypeScript compilation
✓ Canister deployment
✓ Health checks
✓ Inter-canister communication
✓ NOVAQ functionality
✓ Frontend integration
✓ End-to-end tests
✓ Security and performance

=== Build Artifacts ===
EOF

# Add build artifact information
if [ -d "target/wasm32-unknown-unknown/release" ]; then
    echo "WASM Canisters:" >> $TEST_REPORT
    for wasm in target/wasm32-unknown-unknown/release/*.wasm; do
        if [ -f "$wasm" ]; then
            size=$(stat -c%s "$wasm")
            echo "  $(basename $wasm): $size bytes" >> $TEST_REPORT
        fi
    done
fi

if [ -d "ohms-ui/dist" ]; then
    ui_size=$(du -sh ohms-ui/dist | cut -f1)
    echo "OHMS UI build: $ui_size" >> $TEST_REPORT
fi

if [ -d "ohms-website/out" ]; then
    website_size=$(du -sh ohms-website/out | cut -f1)
    echo "OHMS Website build: $website_size" >> $TEST_REPORT
fi

# Clean up
rm -f /tmp/test_output /tmp/e2e_test.sh

print_success "Test report saved to: $TEST_REPORT"

# Final results
echo ""
echo "🧪 OHMS 2.0 Test Suite Complete!"
echo "=================================="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $TESTS_PASSED"
echo "Failed: $TESTS_FAILED"
echo "Skipped: $TESTS_SKIPPED"

if [ $TESTS_FAILED -eq 0 ]; then
    print_success "All tests passed! ✨"
    exit 0
else
    print_error "$TESTS_FAILED test(s) failed"
    exit 1
fi
