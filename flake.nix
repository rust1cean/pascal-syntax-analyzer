{
  description = "Rust 1.86 flake project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system;
          overlays = overlays;
        };

        rustVersion = "1.86.0";

        rustToolchain = pkgs.rust-bin.stable.${rustVersion}.default;

      in {
        devShell = pkgs.mkShell {
          buildInputs = [
            rustToolchain
            pkgs.rustup
            pkgs.rust-analyzer
            pkgs.pkg-config
            pkgs.openssl
          ];
          RUST_BACKTRACE = "1";
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "pascal-syntax-analyzer";
          version = "0.1.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = [ pkgs.openssl ];
        };
      });
}
