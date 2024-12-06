{
  lib ? pkgs.lib,
  pkgs ? (import <nixpkgs> {
    overlays = [
      (import "${fetchTarball "https://github.com/nix-community/fenix/archive/main.tar.gz"}/overlay.nix")
    ];
  }),
  toolchain ? pkgs.fenix.stable,
  ...
}:
pkgs.mkShell {
  name = "rofi-jetbrains";
  inputsFrom = [
    (import ./. {inherit lib pkgs toolchain;})
  ];

  shellHook = ''
    if [ ! -d .direnv/rustup ]; then
      ln -sf "${pkgs.rustup}" .direnv/rustup
    fi
  '';

  nativeBuildInputs = [
    (toolchain.withComponents ["cargo" "rustc" "rust-src" "clippy" "rustfmt"])
    pkgs.rustup
  ];
}
