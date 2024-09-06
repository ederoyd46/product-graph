{
  description = "Rust project with cross-compilation to MUSL and WASM";

  inputs = {
    # nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
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
        # Nix seems to like plurals when using flake-utils so use packages and set a default, then you can run nix build .
        packages = rec {
          default = standardBuild;

          standardBuild = pkgs.rustPlatform.buildRustPackage {
            name = "product-graph";
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.Security ];
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
                gnumake
                cargo-lambda
                cargo-zigbuild
                rustSetup
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.Security ];

            src = self;
            buildPhase = ''
              echo HERE!!!
              cargo zigbuild
            '';
          };
        };

        # Nix seems to like plurals when using flake-utils so use devShells and set a default, then you can run nix develop . or nix develop .#devShells
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
                rustSetup
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.Security ];
          };
        };
      }
    );
}