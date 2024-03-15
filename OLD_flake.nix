{
  description = "Product Graph";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };

  };

  outputs = { self, nixpkgs, flake-utils, fenix }: 
    flake-utils.lib.eachDefaultSystem (system: 
      let 
        pkgs = import nixpkgs { inherit system; };
        rust = fenix.packages.${system}.stable.toolchain;

        buildPackage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "product-graph";
          version = "0.1.0";
          src = self;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # Add SystemConfiguration to build inputs
          buildInputs = [ pkgs.darwin.apple_sdk.frameworks.SystemConfiguration ]; 

          # Set the target to musl and add musl to nativeBuildInputs
          target = "x86_64-unknown-linux-musl";
          nativeBuildInputs = [ pkgs.musl pkgs.musl.dev ];
        };

      in
      {
        # //Use this to define multiple packages to build
        packages.matt1 = buildPackage;

        packages.shell = pkgs.mkShell {
          buildInputs = [ 
            rust 
            pkgs.darwin.apple_sdk.frameworks.SystemConfiguration 
            pkgs.libiconv
            pkgs.cargo-lambda
          ];
        };

        defaultPackage = self.packages.${system}.shell;
        devShell = self.packages.${system}.shell;
      }
    );
}
