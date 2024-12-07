{
  crane,
  callPackage,
  rustup,
}:
crane.devShell {
  name = "rofi-jetbrains";
  inputsFrom = [
    (callPackage ./. {inherit crane;})
  ];

  packages = [
    rustup
  ];

  shellHook = ''
    if [ ! -d .direnv/links/rustup ]; then
    	mkdir -p .direnv/links/
    	ln -sf "${crane.rustc}" .direnv/links/rust
    	ln -sf "${rustup}" .direnv/links/rustup
    fi
  '';
}
