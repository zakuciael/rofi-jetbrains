{
  description = "A rofi plugin that adds the ability to launch recent projects in JetBrains IDEs";

  nixConfig = {
    extra-substituters = ["https://rofi-jetbrains.cachix.org"];
    extra-trusted-public-keys = ["rofi-jetbrains.cachix.org-1:jCHjg5XBg0A17G5/n1QBD39fxbg++URiJCvEuC5cnCs="];
  };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-parts,
    fenix,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "i686-linux"];

      perSystem = {
        pkgs,
        system,
        ...
      }: let
        lib = nixpkgs.lib;
      in rec {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          config = {};
          overlays = [fenix.overlays.default];
        };

        packages = {
          default = packages.rofi-jetbrains;
          rofi-jetbrains = import ./. {
            inherit lib pkgs;
            toolchain = pkgs.fenix.stable;
          };
        };

        devShells = {
          default = import ./shell.nix {
            inherit lib pkgs;
            toolchain = pkgs.fenix.stable;
          };
        };

        apps = let
          mkRofiPackage = pkg:
            if builtins.hasAttr "override" pkg
            then pkg.override (old: {plugins = (old.plugins or []) ++ [packages.rofi-jetbrains];})
            else pkg;
        in {
          default =
            if builtins.getEnv "WAYLAND_DISPLAY" == ""
            then apps.rofi
            else apps.rofi-wayland;
          rofi-wayland = {
            type = "app";
            program = "${lib.getExe (mkRofiPackage pkgs.rofi-wayland)}";
            meta.description = "rofi-wayland cli with the `rofi-jetbrains` plugin pre-installed";
          };
          rofi = {
            type = "app";
            program = "${lib.getExe (mkRofiPackage pkgs.rofi)}";
            meta.description = "rofi cli with the `rofi-jetbrains` plugin pre-installed";
          };
        };
      };
    };
}
