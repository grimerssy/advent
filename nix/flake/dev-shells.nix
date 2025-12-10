{ ... }:
{
  perSystem =
    { self', pkgs, ... }:
    {
      devShells.default = pkgs.mkShellNoCC {
        packages = with pkgs; [
          self'.packages.rust-toolchain
          z3
        ];
        nativeBuildInputs = with pkgs; [
          pkg-config
        ];
      };
    };
}
