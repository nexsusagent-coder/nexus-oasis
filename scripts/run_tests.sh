#!/bin/bash
# SENTIENT Test Runner
# Runs all tests with proper configuration

set -e

echo "🧪 SENTIENT Test Runner"
echo "======================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Configuration
TEST_TYPE="${1:-all}"
VERBOSE="${VERBOSE:-false}"
COVERAGE="${COVERAGE:-false}"

# Functions
run_unit_tests() {
    echo -e "${BLUE}📦 Running Unit Tests${NC}"
    echo "-------------------"
    
    if [ "$COVERAGE" = "true" ]; then
        cargo tarpaulin --lib --out Html --out Lcov
    else
        cargo test --lib $([ "$VERBOSE" = "true" ] && echo "-- --nocapture")
    fi
}

run_integration_tests() {
    echo -e "${BLUE}🔗 Running Integration Tests${NC}"
    echo "-------------------------"
    
    cargo test --test '*' $([ "$VERBOSE" = "true" ] && echo "-- --nocapture")
}

run_property_tests() {
    echo -e "${BLUE}🎲 Running Property-Based Tests${NC}"
    echo "----------------------------"
    
    cargo test --test core_properties $([ "$VERBOSE" = "true" ] && echo "-- --nocapture")
}

run_e2e_tests() {
    echo -e "${BLUE}🌐 Running E2E Tests${NC}"
    echo "----------------"
    
    if [ -z "$OPENAI_API_KEY" ]; then
        echo -e "${YELLOW}⚠ OPENAI_API_KEY not set, some tests may be skipped${NC}"
    fi
    
    cargo test --test core_e2e $([ "$VERBOSE" = "true" ] && echo "-- --nocapture")
}

run_all_tests() {
    echo -e "${BLUE}🚀 Running All Tests${NC}"
    echo "================"
    echo ""
    
    run_unit_tests
    echo ""
    run_integration_tests
    echo ""
    run_property_tests
    echo ""
    run_e2e_tests
}

run_specific_tests() {
    echo -e "${BLUE}🔍 Running Specific Tests: $1${NC}"
    echo "----------------------------"
    
    cargo test "$1" $([ "$VERBOSE" = "true" ] && echo "-- --nocapture")
}

generate_coverage_report() {
    echo -e "${BLUE}📊 Generating Coverage Report${NC}"
    echo "--------------------------"
    
    # Install tarpaulin if not present
    if ! command -v cargo-tarpaulin &> /dev/null; then
        echo "Installing tarpaulin..."
        cargo install cargo-tarpaulin
    fi
    
    # Run with coverage
    cargo tarpaulin --workspace --out Html --out Lcov --out Xml
    
    echo ""
    echo -e "${GREEN}✅ Coverage report generated${NC}"
    echo "   HTML: tarpaulin-report.html"
    echo "   LCOV: lcov.info"
    echo ""
    
    # Display summary
    if command -v lcov &> /dev/null; then
        lcov --summary lcov.info 2>/dev/null || true
    fi
}

# Main
case "$TEST_TYPE" in
    "unit")
        run_unit_tests
        ;;
    "integration")
        run_integration_tests
        ;;
    "property")
        run_property_tests
        ;;
    "e2e")
        run_e2e_tests
        ;;
    "coverage")
        generate_coverage_report
        ;;
    "all")
        run_all_tests
        ;;
    *)
        run_specific_tests "$TEST_TYPE"
        ;;
esac

echo ""
echo -e "${GREEN}✅ Tests completed!${NC}"
