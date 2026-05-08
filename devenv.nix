{ pkgs, lib, config, inputs, ... }:

{
  # Enable Rust and Zig language support natively in devenv
  languages.rust = {
    enable = true;
  };

  languages.zig = {
    enable = true;
  };

  # Additional CLI tools
  packages = [
    pkgs.cargo-binutils
    pkgs.probe-rs-tools
    pkgs.python3
    pkgs.python3Packages.pyserial
    pkgs.jq
    pkgs.just
  ];

  enterShell = ''
    rustup target add thumbv8m.main-none-eabihf
    rustup component add llvm-tools-preview
  '';

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
}
