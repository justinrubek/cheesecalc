{inputs, ...}: {
  perSystem = {
    config,
    pkgs,
    system,
    inputs',
    self',
    ...
  }: let
    # packages for ci
    extraPackages = [
      self'.packages.cocogitto
      self'.packages.bomper
    ];

    packages = {
      cocogitto = pkgs.cocogitto;
      bomper = inputs'.bomper.packages.cli;
    };

    devShells = {
      ci = pkgs.mkShell rec {
        packages = extraPackages;

        LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath packages;
      };
    };
  in rec {
    inherit devShells packages;

    legacyPackages = {
      ciPackages = extraPackages;
    };
  };
}
