{ ... }:
{
  perSystem =
    { self', pkgs, ... }:
    {
      devShells.default = pkgs.mkShellNoCC {
        packages = [
          self'.packages.rust-toolchain
        ];
      };
    };
}
