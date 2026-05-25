default: setup test

test-lse:
    cargo test --features stm32u575zi,lse,defmt

test:
    cargo test --features stm32u575zi,defmt

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

# Generate/update .devcontainer configuration using devbox
generate-devcontainer:
    #!/usr/bin/env bash
    exts=$(jq -c '.customizations.vscode.extensions // []' .devcontainer/devcontainer.json 2>/dev/null || echo '[]')
    devbox generate devcontainer --force
    jq --argjson old "$exts" '.customizations.vscode.extensions = (($old + (.customizations.vscode.extensions // [])) | unique)' .devcontainer/devcontainer.json > .devcontainer/devcontainer.tmp
    mv .devcontainer/devcontainer.tmp .devcontainer/devcontainer.json



