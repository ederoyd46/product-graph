{
  description = "Rust project with cross-compilation to MUSL and WASM";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
    }:
    let
      system = "aarch64-darwin";
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
