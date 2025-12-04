{ inputs, ... }:
{
  perSystem =
    { self', pkgs, ... }:
    {
      packages = with pkgs; {
        rust-toolchain = callPackage ../packages/rust-toolchain.nix {
          inherit (callPackage inputs.fenix { }) fromToolchainFile;
        };
      };
    };
}
