{
  description = "Rust project with cross-compilation to MUSL and WASM";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        rustSetup = pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "rust-src"
            "rustfmt"
            "rust-analyzer"
            "rust-docs"
          ];
          targets = [
            "x86_64-unknown-linux-musl"
            "wasm32-wasi"
          ];
        };
      in
      {
        packages = rec {
          default = standardBuild;

          standardBuild = pkgs.rustPlatform.buildRustPackage {
            name = "product-graph";
            buildInputs = with pkgs; [ darwin.Security ] ++ [ rustSetup ];
            src = self;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };

          mattBuild = pkgs.stdenv.mkDerivation {
            name = "product-graph";
            buildInputs =
              with pkgs;
              [
                darwin.Security
                gnumake
                cargo-lambda
                cargo-zigbuild
                zig
              ]
              ++ [ rustSetup ];
            src = self;
            buildPhase = ''
              echo HERE!!!
              cargo zigbuild
            '';
          };
        };

        devShells = rec {
          default = devShell;

          devShell = pkgs.mkShell {
            buildInputs =
              with pkgs;
              [
                zig
                cargo-lambda
                cargo-zigbuild
                darwin.Security
              ]
              ++ [ rustSetup ];
          };
        };
      }
    );
}
