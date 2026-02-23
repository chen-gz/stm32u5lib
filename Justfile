default: setup test-arm test-host

test-arm:
    cargo test --features stm32u575zi,lse,log --target thumbv8m.main-none-eabihf
    cargo test --features stm32u575zi,log --target thumbv8m.main-none-eabihf

test: test-arm

HOST_TARGET := `rustc -vV | grep host | cut -d ' ' -f 2`

test-host:
    cargo test --test walltimer_test --target {{HOST_TARGET}}

coverage:
    cargo llvm-cov --test walltimer_test --target {{HOST_TARGET}}

setup:
    rustup default stable
    rustup target add thumbv8m.main-none-eabihf
    rustup component add llvm-tools-preview rust-src rust-analyzer
    cargo install cargo-binutils
    cargo install probe-rs-tools
    cargo install cargo-llvm-cov

# nucleo_u575 = ["stm32u575zi", "lse"]
