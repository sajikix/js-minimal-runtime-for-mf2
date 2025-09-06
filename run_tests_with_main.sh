#!/bin/bash

echo "Building Rust project..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "Failed to build Rust project"
    exit 1
fi

echo ""
echo "========================================="

for test_file in test/*.js; do
    if [ -f "$test_file" ]; then
        echo ""
        echo "-----------------------------------------"
        echo "Running: $test_file"
        echo "-----------------------------------------"
        ./target/release/js-minimal-runtime-for-mf2 "$test_file"
    fi
done

echo ""
echo "========================================="