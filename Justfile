default: setup test

test:
    cargo test --features stm32u575zi,lse,log
    cargo test --features stm32u575zi,log

test-auto:
    @echo "Starting USB Test Harness in background..."
    @python3 scripts/test_usb_acm.py > /tmp/usb_test.log 2>&1 & \
    PID=$$!; \
    echo "USB Harness started (PID: $$PID). Running tests..."; \
    cargo test --features nucleo_u575,defmt; \
    EXIT_CODE=$$?; \
    echo "Tests finished. Cleaning up USB Harness..."; \
    kill $$PID > /dev/null 2>&1; \
    exit $$EXIT_CODE

setup:
    rustup default stable
    rustup target add thumbv8m.main-none-eabihf
    rustup component add llvm-tools-preview rust-src rust-analyzer
    cargo install cargo-binutils
    cargo install probe-rs-tools

# nucleo_u575 = ["stm32u575zi", "lse"]
