{
  description = "Rust project";

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
            "x86_64-unknown-linux-musl" # Release target for AWS Lambda/Fly
            "wasm32-wasi"
          ];
        };
      in
      {
        # Nix seems to like plurals when using flake-utils so use packages and set a default, then you can run nix build .
        packages = rec {
          default = build;

          build = pkgs.rustPlatform.buildRustPackage {
            name = "product-graph";
            buildInputs = pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.Security ];
            src = self;
            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };

          buildWithRustSetup = pkgs.stdenv.mkDerivation {
            name = "product-graph";
            buildInputs =
              with pkgs;
              [
                gnumake
                cargo-zigbuild
                rustSetup
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.Security ];

            src = self;

            # As cargo dependencies like this as we have no network access here
            cargoDeps = pkgs.rustPlatform.importCargoLock { lockFile = ./Cargo.lock; };
            nativeBuildInputs = with pkgs.rustPlatform; [ cargoSetupHook ];

            buildPhase = ''
              # Build artifacts
              export HOME=$(pwd) #work around for readonly homeless-shelter
              cargo zigbuild --release
              cargo zigbuild --release --target x86_64-unknown-linux-musl #build target for AWS Lambda/Fly
            '';

            installPhase = ''
              mkdir -p $out/${system}/bin
              cp target/release/product-graph $out/${system}/bin

              mkdir -p $out/x86_64-unknown-linux-musl/bin
              cp target/x86_64-unknown-linux-musl/release/product-graph $out/x86_64-unknown-linux-musl/bin
            '';
          };
        };

        # Nix likes plurals when using flake-utils so use devShells and set a default, then you can run nix develop . or nix develop .#devShells
        devShells = rec {
          default = devShell;

          devShell = pkgs.mkShell {
            buildInputs =
              with pkgs;
              [
                zig
                cargo-lambda
                cargo-zigbuild
                (surrealdb.overrideAttrs { meta.license = [ "free" ]; }) # Override the license to bypass unfree warnings
                rustSetup
              ]
              ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ pkgs.darwin.Security ];
          };
        };
      }
    );
}
