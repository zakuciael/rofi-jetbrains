{
  crane,
  callPackage,
  writeShellApplication,
  fetchFromGitHub,
}: let
  cargo-wrapper = let
    cargo = crane.cargo;
  in
    writeShellApplication {
      name = "cargo-wrapper";
      text = ''
        if [ "$1" == "check" ]; then
        	${cargo}/bin/cargo clippy "''${@:2}"
        else
        	${cargo}/bin/cargo "$@"
        fi
      '';
    };
  toolchain = crane.rustc.overrideAttrs (prev: {
    buildCommand =
      prev.buildCommand
      + ''
        cp -f ${cargo-wrapper}/bin/cargo-wrapper $out/bin/cargo
      '';
  });
  knope = let
    src = crane.cleanCargoSource (fetchFromGitHub {
      owner = "knope-dev";
      repo = "knope";
      rev = "knope/v0.18.1";
      hash = "sha256-KA5ePuN9MWbhsrz3UVr8brbs77P0AHXK/3f6RccfWac=";
    });
    commonArgs = {
      inherit src;
      inherit (crane.crateNameFromCargoToml {cargoToml = "${src}/crates/knope/Cargo.toml";}) pname version;
      strictDeps = true;
    };
    cargoArtifacts = crane.buildDepsOnly commonArgs;
  in
    crane.buildPackage (commonArgs
      // {
        inherit cargoArtifacts;
        cargoExtraArgs = "-p knope";
        doCheck = false;
      });
in
  crane.devShell {
    name = "rofi-jetbrains";
    inputsFrom = [
      (callPackage ./. {inherit crane;})
    ];

    packages = [
      knope
    ];

    shellHook = ''
      if [ ! -d .direnv/links/rust ]; then
      	mkdir -p .direnv/links/
      	ln -sf "${toolchain}" .direnv/links/rust
      fi
    '';
  }
