default: setup test

test:
    cargo test --features stm32u575zi,lse,log
    cargo test --features stm32u575zi,log

setup:
    rustup default stable
    rustup target add thumbv8m.main-none-eabihf
    rustup component add llvm-tools-preview rust-src rust-analyzer
    cargo install cargo-binutils
    cargo install probe-rs-tools

# nucleo_u575 = ["stm32u575zi", "lse"]
