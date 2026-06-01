{ pkgs, lib, config, inputs, ... }:

{
  # Enable Zig language support natively in devenv
  languages.zig = {
    enable = true;
  };

  # https://devenv.sh/packages/
  packages = [
    pkgs.cargo-binutils
    pkgs.probe-rs-tools
    pkgs.python3
    pkgs.python3Packages.pyserial
    pkgs.jq
    pkgs.just
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
}
