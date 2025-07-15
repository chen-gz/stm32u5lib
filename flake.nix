{
  description = "Rust development environment for stm32u5lib";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.stable."1.88.0".default.override {
          extensions = [ "rust-src" "rust-analyzer" "llvm-tools-preview" ];
          targets = [ "thumbv8m.main-none-eabihf" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # System dependencies from Dockerfile
            curl
            git
            pkg-config
            libusb1
            openssl # Common rust dependency
            fish

            # For building C code (equivalent to build-essential)
            gcc
            gnumake

            # Rust toolchain and components
            rustToolchain
            cargo-binutils
          ];

          shellHook = ''
            exec fish
          '';

          # Set environment variables from Dockerfile
          RUST_BACKTRACE = "1";
        };
      }
    );
}
