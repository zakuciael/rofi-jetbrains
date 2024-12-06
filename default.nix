{
  lib ? pkgs.lib,
  pkgs ? (import <nixpkgs> {
    overlays = [
      (import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")
    ];
  }),
  toolchain ? pkgs.fenix.stable,
  ...
}: let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = toolchain.cargo;
    rustc = toolchain.rustc;
  };
  cargoToml = lib.importTOML ./Cargo.toml;
  srcFileName = ''lib${builtins.replaceStrings ["-"] ["_"] cargoToml.package.name}.so'';
  outFileName = "jetbrains.so";
in
  rustPlatform.buildRustPackage rec {
    inherit (cargoToml.package) name version;
    pname = name;

    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = with pkgs; [
      glib.dev
      gtk3.dev
    ];

    postInstall = ''
      mkdir -p $out/lib/rofi
      mv $out/lib/${srcFileName} $out/lib/rofi/${outFileName}
    '';

    doCheck = false;
    cargoLock.lockFile = ./Cargo.lock;
    src = ./.;

    meta = with lib; {
      description = "A rofi plugin that adds the ability to launch recent projects in JetBrains IDEs";
      homepage = "https://github.com/zakuciael/rofi-jetbrains";
      license = licenses.mit;
      maintainers = with maintainers; [zakuciael];
      platforms = platforms.linux;
    };
  }
