{
  description = "A rofi plugin that adds the ability to launch recent projects in JetBrains IDEs";

  nixConfig = {
    extra-substituters = ["https://rofi-jetbrains.cachix.org"];
    extra-trusted-public-keys = ["rofi-jetbrains.cachix.org-1:jCHjg5XBg0A17G5/n1QBD39fxbg++URiJCvEuC5cnCs="];
  };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    crane.url = "github:ipetkov/crane";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-analyzer-src.follows = "";
    };
  };

  outputs = {flake-parts, ...} @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "i686-linux"];

      debug = true;

      perSystem = {
        pkgs,
        system,
        crane,
        ...
      }: rec {
        _module.args = {
          pkgs = import inputs.nixpkgs {
            inherit system;
            config = {};
            overlays = [inputs.fenix.overlays.default];
          };
          crane =
            (inputs.crane.mkLib pkgs).overrideToolchain
            (pkgs': (pkgs'.fenix.stable.withComponents ["cargo" "rustc" "rust-src" "clippy" "rustfmt"]));
        };

        packages = {
          default = packages.rofi-jetbrains;

          rofi-jetbrains = pkgs.callPackage ./. {inherit crane;};
          rofi-jetbrains-next = pkgs.callPackage ./. {
            inherit crane;
            rofi_next = true;
          };
        };

        devShells = {
          default = pkgs.callPackage ./shell.nix {inherit crane;};
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
            program = "${mkRofiPackage pkgs.rofi-wayland packages.rofi-jetbrains-next}/bin/rofi";
            meta.description = "rofi-wayland cli with the `rofi-jetbrains` plugin pre-installed";
          };
          rofi = {
            type = "app";
            program = "${mkRofiPackage pkgs.rofi packages.rofi-jetbrains}/bin/rofi";
            meta.description = "rofi cli with the `rofi-jetbrains` plugin pre-installed";
          };
        };
      };
    };
}
