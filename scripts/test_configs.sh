#!/bin/bash
set -e

echo "Testing default configuration (LSE enabled)..."
cargo check --target thumbv8m.main-none-eabihf

echo "Testing LSI configuration (LSE disabled)..."
cargo check --target thumbv8m.main-none-eabihf --no-default-features --features stm32u575zi

echo "All configurations verified successfully."
