#!/bin/bash
# SENTIENT Benchmark Runner
# Usage: ./run_benchmarks.sh [scenario]

set -e

echo "🚀 SENTIENT Benchmark Suite"
echo "============================"
echo ""

# Configuration
ITERATIONS=${ITERATIONS:-10}
WARMUP=${WARMUP:-3}
OUTPUT_DIR="results/$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_result() {
    echo -e "${GREEN}✓${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

print_info() {
    echo -e "${YELLOW}ℹ${NC} $1"
}

# System info
echo "📊 System Information:"
echo "  OS: $(uname -s)"
echo "  CPU: $(nproc) cores"
echo "  Memory: $(free -h | grep Mem | awk '{print $2}')"
echo ""

# Benchmark scenarios
run_benchmark() {
    local name=$1
    local command=$2
    
    echo ""
    echo "🔬 Running: $name"
    echo "----------------------------"
    
    # Warmup
    print_info "Warming up ($WARMUP iterations)..."
    for i in $(seq 1 $WARMUP); do
        $command > /dev/null 2>&1 || true
    done
    
    # Actual benchmark
    print_info "Running benchmark ($ITERATIONS iterations)..."
    
    local times=()
    for i in $(seq 1 $ITERATIONS); do
        start=$(date +%s%N)
        $command > /dev/null 2>&1
        end=$(date +%s%N)
        duration=$(( (end - start) / 1000000 ))
        times+=($duration)
        echo "  Iteration $i: ${duration}ms"
    done
    
    # Calculate statistics
    local sum=0
    for t in "${times[@]}"; do
        sum=$((sum + t))
    done
    local avg=$((sum / ITERATIONS))
    
    # Sort for percentiles
    IFS=$'\n' sorted=($(sort -n <<<"${times[*]}")); unset IFS
    local p50=${sorted[$((ITERATIONS / 2))]}
    local p99=${sorted[$((ITERATIONS * 99 / 100))]}
    
    echo ""
    echo "  Results:"
    echo "    Average: ${avg}ms"
    echo "    P50: ${p50}ms"
    echo "    P99: ${p99}ms"
    
    # Save results
    echo "$name,${avg},${p50},${p99}" >> "$OUTPUT_DIR/results.csv"
    
    print_result "$name completed"
}

# Run Cargo benchmarks
run_cargo_bench() {
    echo ""
    echo "🔬 Running Cargo Benchmarks"
    echo "----------------------------"
    
    cargo bench -- --save-baseline "$OUTPUT_DIR" 2>&1 | tee "$OUTPUT_DIR/cargo_bench.log"
    
    print_result "Cargo benchmarks completed"
}

# Memory benchmark
run_memory_benchmark() {
    echo ""
    echo "🔬 Memory Benchmark"
    echo "----------------------------"
    
    # Start agent
    cargo build --release 2>/dev/null
    
    # Measure idle memory
    ./target/release/sentient &
    local pid=$!
    sleep 2
    
    local idle_mem=$(ps -o rss= -p $pid)
    echo "  Idle memory: ${idle_mem}KB"
    
    # Stop
    kill $pid 2>/dev/null || true
    
    echo "$idle_mem" > "$OUTPUT_DIR/memory.txt"
    print_result "Memory benchmark completed"
}

# Throughput benchmark
run_throughput_benchmark() {
    echo ""
    echo "🔬 Throughput Benchmark"
    echo "----------------------------"
    
    # Start server
    cargo run --release --example web-api &
    local pid=$!
    sleep 5
    
    # Run wrk
    if command -v wrk &> /dev/null; then
        wrk -t4 -c100 -d30s http://localhost:3000/health > "$OUTPUT_DIR/throughput.txt"
        cat "$OUTPUT_DIR/throughput.txt"
    else
        print_info "wrk not installed, skipping throughput test"
    fi
    
    # Stop server
    kill $pid 2>/dev/null || true
    
    print_result "Throughput benchmark completed"
}

# Latency benchmark
run_latency_benchmark() {
    echo ""
    echo "🔬 Latency Benchmark"
    echo "----------------------------"
    
    cargo run --release --example web-api &
    local pid=$!
    sleep 5
    
    # Measure latencies
    local latencies=()
    for i in $(seq 1 100); do
        start=$(date +%s%N)
        curl -s http://localhost:3000/health > /dev/null
        end=$(date +%s%N)
        latency=$(( (end - start) / 1000000 ))
        latencies+=($latency)
    done
    
    # Calculate
    IFS=$'\n' sorted=($(sort -n <<<"${latencies[*]}")); unset IFS
    echo "  P50: ${sorted[50]}ms"
    echo "  P90: ${sorted[90]}ms"
    echo "  P99: ${sorted[99]}ms"
    
    kill $pid 2>/dev/null || true
    
    print_result "Latency benchmark completed"
}

# Main
case "${1:-all}" in
    "cargo")
        run_cargo_bench
        ;;
    "memory")
        run_memory_benchmark
        ;;
    "throughput")
        run_throughput_benchmark
        ;;
    "latency")
        run_latency_benchmark
        ;;
    "all")
        run_cargo_bench
        run_memory_benchmark
        run_throughput_benchmark
        run_latency_benchmark
        ;;
    *)
        echo "Unknown scenario: $1"
        echo "Usage: $0 [cargo|memory|throughput|latency|all]"
        exit 1
        ;;
esac

# Summary
echo ""
echo "============================"
echo "📊 Benchmark Summary"
echo "============================"
echo ""
echo "Results saved to: $OUTPUT_DIR"
echo ""

if [ -f "$OUTPUT_DIR/results.csv" ]; then
    echo "Benchmark Results:"
    column -t -s',' "$OUTPUT_DIR/results.csv"
fi

echo ""
print_result "All benchmarks completed!"
