{ inputs, ... }:
{
  systems = inputs.nixpkgs.lib.systems.flakeExposed;
  imports = [
    # keep-sorted start case=no numeric=yes
    ./dev-shells.nix
    ./formatter.nix
    ./packages.nix
    # keep-sorted end
  ];
}
