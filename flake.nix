{
  description = "A rofi plugin that adds the ability to launch recent projects in JetBrains IDEs";

  nixConfig = {
    extra-substituters = ["https://rofi-jetbrains.cachix.org"];
    extra-trusted-public-keys = ["rofi-jetbrains.cachix.org-1:jCHjg5XBg0A17G5/n1QBD39fxbg++URiJCvEuC5cnCs="];
  };

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  } @ inputs: let
    systems = ["x86_64-linux" "aarch64-linux" "i686-linux"];
  in
    flake-utils.lib.eachSystem systems (system: let
      lib = nixpkgs.lib;
      pkgs = import nixpkgs {
        inherit system;
        config = {};
        overlays = [
          (final: prev: {
            fenix = import inputs.fenix {
              pkgs = prev;
              rust-analyzer-src = throw "not used";
            };
          })
        ];
      };
      toolchain = pkgs.fenix.stable;
    in rec {
      devShell = import ./shell.nix {inherit lib pkgs toolchain;};
      defaultPackage = packages.rofi-jetbrains;
      packages = {
        default = packages.rofi-jetbrains;
        rofi-jetbrains = import ./. {inherit lib pkgs toolchain;};
      };
    });
}
