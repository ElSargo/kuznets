{
  description = "Dev shell the project";

  inputs = {
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        rust = fenix.packages.${system}.complete.toolchain;
      in {
        nixpkgs.overlays = [ fenix.overlays.complete ];
        devShells.default = pkgs.mkShell rec {

          buildInputs = [
            rust
            pkgs.jupyter
            pkgs.evcxr
          ];
          LD_LIBRARY_PATH = nixpkgs.lib.makeLibraryPath buildInputs;
        };
      });
}

