{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = [
    pkgs.rustup
    pkgs.cargo-binutils
    pkgs.probe-rs-tools
    pkgs.python3
    pkgs.python3Packages.pyserial
    pkgs.jq
    pkgs.just
  ];

  # https://devenv.sh/basics/
  enterShell = ''
    rustup default stable
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

