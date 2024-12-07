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
        toolchain,
        ...
      }: let
        lib = nixpkgs.lib;
      in rec {
        _module.args = {
          pkgs = import nixpkgs {
            inherit system;
            config = {};
            overlays = [fenix.overlays.default];
          };
          toolchain = pkgs.fenix.stable;
        };

        packages = {
          default = packages.rofi-jetbrains;
          rofi-jetbrains = import ./. {
            inherit lib pkgs toolchain;
          };
          rofi-jetbrains-next = import ./. {
            inherit lib pkgs toolchain;
            rofi_next = true;
          };
        };

        devShells = {
          default = import ./shell.nix {
            inherit lib pkgs toolchain;
          };
        };

        apps = let
          mkRofiPackage = rofi: plugin:
            if builtins.hasAttr "override" rofi
            then rofi.override (old: {plugins = (old.plugins or []) ++ [plugin];})
            else rofi;
        in {
          default =
            if builtins.getEnv "WAYLAND_DISPLAY" == ""
            then apps.rofi
            else apps.rofi-wayland;
          rofi-wayland = {
            type = "app";
            program = "${lib.getExe (mkRofiPackage pkgs.rofi-wayland packages.rofi-jetbrains-next)}";
            meta.description = "rofi-wayland cli with the `rofi-jetbrains` plugin pre-installed";
          };
          rofi = {
            type = "app";
            program = "${lib.getExe (mkRofiPackage pkgs.rofi packages.rofi-jetbrains)}";
            meta.description = "rofi cli with the `rofi-jetbrains` plugin pre-installed";
          };
        };
      };
    };
}
