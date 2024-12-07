{
  crane,
  callPackage,
  writeShellApplication,
  rustup,
}: let
  rustc = crane.rustc;
  wrapper = writeShellApplication {
    name = "cargo-wrapper";
    text = ''
      if [ "$1" == "check" ]; then
      	${rustc}/bin/cargo clippy "''${@:2}"
      else
      	${rustc}/bin/cargo "$@"
      fi
    '';
  };
  toolchain = crane.rustc.overrideAttrs (prev: {
    buildCommand =
      prev.buildCommand
      + ''
        cp -f ${wrapper}/bin/cargo-wrapper $out/bin/cargo
      '';
  });
in
  crane.devShell {
    name = "rofi-jetbrains";
    inputsFrom = [
      (callPackage ./. {inherit crane;})
    ];

    packages = [
      rustup
    ];

    shellHook = ''
      if [ ! -d .direnv/links/rust ]; then
      	mkdir -p .direnv/links/
      	ln -sf "${toolchain}" .direnv/links/rust
      fi
    '';
  }
