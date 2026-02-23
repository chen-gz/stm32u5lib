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
    cargo install cargo-llvm-cov

test-host:
    cargo test -p host_tests --target x86_64-unknown-linux-gnu

coverage:
    # We must ensure we target the host to avoid trying to run llvm-cov on the embedded target
    # AND we need to copy the source file to a location where coverage report can find it relative to the workspace
    # But for now, let's just run it as we did.
    cargo llvm-cov -p host_tests --target x86_64-unknown-linux-gnu --lcov --output-path lcov.info
    # Since we use #[path] in host_tests/src/lib.rs, llvm-cov might report paths strangely or miss them if not careful.
    # But previous attempts showed it finding "src/utils.rs" when we did the copy trick.
    # The clean way is to have the code in a shared crate or a module that is compiled.
    # However, since we can't easily restructure the whole crate now, let's do the copy trick in the recipe to ensure report is good.
    mkdir -p host_tests/src/utils && cp src/utils.rs host_tests/src/utils/mod.rs
    cargo llvm-cov -p host_tests --target x86_64-unknown-linux-gnu --lcov --output-path lcov.info
    # Fix path in lcov.info to point to original source
    sed -i 's|SF:.*/host_tests/src/utils/mod.rs|SF:src/utils.rs|' lcov.info
    rm -rf host_tests/src/utils

# nucleo_u575 = ["stm32u575zi", "lse"]
