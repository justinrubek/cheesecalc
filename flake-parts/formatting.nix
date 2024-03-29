{
  inputs,
  self,
  ...
}: {
  perSystem = {
    pkgs,
    lib,
    ...
  }: let
    formatters = [
      pkgs.alejandra
      pkgs.rustfmt
      pkgs.nodePackages.prettier
    ];

    treefmt = pkgs.writeShellApplication {
      name = "treefmt";
      runtimeInputs = [pkgs.treefmt] ++ formatters;
      text = ''
        exec treefmt "$@"
      '';
    };
  in {
    packages = {
      inherit treefmt;
    };

    legacyPackages = {
      inherit formatters;
    };
  };
}
