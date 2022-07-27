{
  description = "cheese calculator";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    gitignore,
    rust-overlay,
    pre-commit-hooks,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          rust-overlay.overlays.default
        ];
      };
      inherit (gitignore.lib) gitignoreSource;
      pre-commit-check = pre-commit-hooks.lib.${system}.run {
        src = gitignoreSource ./.;
        hooks = {
          alejandra.enable = true;
          rustfmt.enable = true;
        };
      };

      rust = pkgs.rust-bin.stable.latest.default;
      rustPackage = pkgs.rustPlatform.buildRustPackage {
        pname = "cheesecalc";
        version = "0.2.0";

        src = gitignoreSource ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };
    in rec {
      packages = {
        cheesecalc = rustPackage;
        default = packages.cheesecalc;
      };
      devShells = {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [rust rustfmt];
          inherit (pre-commit-check) shellHook;
        };
      };
      apps = {
        cheesecalc = {
          type = "app";
          program = "${packages.default}/bin/cheesecalc";
        };
        default = apps.cheesecalc;
      };
    });
}
