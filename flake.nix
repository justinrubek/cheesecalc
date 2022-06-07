{
  description = "cheese calculator";

  inputs = {
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
    nixpkgs.follows = "cargo2nix/nixpkgs";
    flake-utils.follows = "cargo2nix/flake-utils";
  };

  outputs = { self, nixpkgs, cargo2nix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system: 
    let
        pkgs = import nixpkgs {
              inherit system;
              overlays = [cargo2nix.overlays.default];
        };

        rustPkgs = pkgs.rustBuilder.makePackageSet {
            rustVersion = "1.60.0";
            packageFun = import ./Cargo.nix;
        };
    in rec {
        packages = {
            cheesecalc = (rustPkgs.workspace.cheesecalc {}).bin;
            default = packages.cheesecalc;
        };
        devShells = {
            default = (rustPkgs.workspaceShell { });
        };
        apps = {
            cheesecalc = { type = "app"; program = "${packages.default}/bin/cheesecalc"; };
            default = apps.cheesecalc;
        };
    });
}
