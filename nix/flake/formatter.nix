{ inputs, ... }:
{
  imports = [
    inputs.treefmt-nix.flakeModule
  ];
  perSystem =
    { self', ... }:
    {
      treefmt = {
        projectRootFile = "flake.nix";
        programs.nixfmt.enable = true;
        programs.keep-sorted.enable = true;
        programs.rustfmt = {
          enable = true;
          package = self'.packages.rust-toolchain;
        };
        settings.global.excludes = [
          "Cargo.toml"
          "target/*"
          ".envrc"
          ".direnv/*"
          ".editorconfig"
          ".gitignore"
        ];
      };
    };
}
