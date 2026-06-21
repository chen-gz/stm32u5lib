{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    pkgs.cargo-binutils
    pkgs.probe-rs-tools
    pkgs.python3
    pkgs.python3Packages.pyserial
    pkgs.jq
    pkgs.just
    pkgs.cargo-llvm-cov
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "thumbv8m.main-none-eabihf" ];
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "llvm-tools-preview"
      "rust-src"
    ];
  };

  # https://devenv.sh/reference/options/#devcontainerenable
  devcontainer.enable = false;

  devcontainer.settings = {
    customizations.vscode.extensions = [
      "cachix.devenv"
      "mkhl.direnv"
      "marus25.cortex-debug"
      "probe-rs.probe-rs-debugger"
      "rust-lang.rust-analyzer"
    ];
  };

  # Scripts to migrate from Justfile
  scripts.test-lse.exec = "cargo test --features stm32u575zi,lse,defmt";
  scripts.test.exec = "cargo test --features stm32u575zi,defmt";

  scripts.test-host.exec = ''
    HOST_TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
    cargo test --lib --target "$HOST_TARGET"
  '';

  scripts.coverage.exec = ''
    HOST_TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
    cargo llvm-cov --lib --target "$HOST_TARGET"
  '';

  scripts.coverage-html.exec = ''
    HOST_TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
    cargo llvm-cov --lib --target "$HOST_TARGET" --html
    echo "HTML coverage report generated at target/llvm-cov/html/index.html"
  '';

  scripts.ci.exec = ''
    HOST_TARGET=$(rustc -vV | grep host | cut -d' ' -f2)
    cargo fmt --all --check
    cargo clippy --lib --target "$HOST_TARGET"
    cargo clippy --lib --target thumbv8m.main-none-eabihf
    cargo clippy --tests --target thumbv8m.main-none-eabihf
    cargo check --example camera --target "$HOST_TARGET"
    cargo test --lib --target "$HOST_TARGET"
    cargo llvm-cov --lib --target "$HOST_TARGET"
  '';

  scripts.test-auto.exec = ''
    echo "Starting USB Test Harness in background..."
    python3 scripts/test_usb_acm.py > /tmp/usb_test.log 2>&1 &
    PID=$!
    echo "USB Harness started (PID: $PID). Running tests..."
    cargo test --features nucleo_u575,defmt
    EXIT_CODE=$?
    echo "Tests finished. Cleaning up USB Harness..."
    kill $PID > /dev/null 2>&1
    exit $EXIT_CODE
  '';

  scripts.generate-devcontainer.exec = ''
    rm -f .devcontainer.json
    devenv eval devcontainer.settings | jq '.["devcontainer.settings"]' > .devcontainer.json
  '';
}
