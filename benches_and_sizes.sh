#!/usr/bin/env bash
set -euo pipefail

# Determine crate name
CRATE_NAME=$(grep -m1 '^name' Cargo.toml | sed -E 's/name *= *"(.*)"/\1/')
CRATE_RLIB="lib${CRATE_NAME//-/_}.rlib"

echo "Detected crate: $CRATE_NAME"
echo

######################################
# Part 1 BENCHMARKS
######################################
echo "Running benchmarks (part1)..."
BENCH1_OUTPUT=$(cargo bench --quiet 2>/dev/null || true)

echo
echo "### Benchmarks (part1):"
echo '```'
echo "$BENCH1_OUTPUT" | sed -n '/bench_part1/,/samples/p'
echo '```'
echo


######################################
# no_std builds
######################################
echo "Building no_std libs..."
cargo build --release --lib --target-dir target/lib-part1
cargo build --release --lib --features part2 --target-dir target/lib-part2


######################################
# Part 2 BENCHMARKS
######################################
echo
echo "Running benchmarks (part2)..."
BENCH2_OUTPUT=$(cargo bench --quiet --features part2 2>/dev/null || true)

echo
echo "### Benchmarks (part2):"
echo '```'
echo "$BENCH2_OUTPUT" | sed -n '/bench_part2/,/samples/p'
echo '```'
echo


######################################
# no_std sizes
######################################
LIB1="target/lib-part1/release/$CRATE_RLIB"
LIB2="target/lib-part2/release/$CRATE_RLIB"

if [[ ! -f "$LIB1" || ! -f "$LIB2" ]]; then
    echo "Error: expected .rlib not found:"
    echo "  $LIB1"
    echo "  $LIB2"
    exit 1
fi

SIZE1=$(stat -f%z "$LIB1")
SIZE2=$(stat -f%z "$LIB2")

echo "### no_std library sizes:"
echo "* Part 1: ${SIZE1} bytes"
echo "* Part 2: ${SIZE2} bytes"
echo
