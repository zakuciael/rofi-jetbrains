{
  lib,
  crane,
  pkg-config,
  glib,
  gtk3,
  jq,
  rofi_next ? false,
  ...
}: let
  commonArgs = {
    src = crane.cleanCargoSource ./.;
    strictDeps = true;

    nativeBuildInputs = [
      pkg-config
    ];

    buildInputs = [
      glib.dev
      gtk3.dev
    ];
  };
in
  crane.buildPackage (commonArgs
    // {
      cargoArtifacts = crane.buildDepsOnly commonArgs;

      RUSTFLAGS = lib.optionalString rofi_next "--cfg rofi_next";

      installPhaseCommand = ''
        function installRofiPluginFromCargoBuildLog() {
        	local dest=''${1:-''${out}}
        	local log=''${2:-''${cargoBuildLog:?not defined}}

        	if ! [ -f "''${log}" ]; then
        		echo "unable to install plugin, cargo build log does not exist at: ''${log}"
        		false
        	fi

        	echo "searching for plugin files to install from cargo build log at ''${log}"
        	echo "jq: ${lib.getExe jq}"

        	local logs
        	logs=$(${lib.getExe jq} -R 'fromjson?' <"''${log}")

        	local select_non_deps_artifact='select(contains("/deps/artifact/") | not)'
        	local members="$(command cargo metadata --format-version 1 | ${lib.getExe jq} -c '.workspace_members')"
        	local select_non_test_members='select(.reason == "compiler-artifact" and .profile.test == false)
          	| select(.package_id as $pid
              | '"''${members}"'
              | contains([$pid])
            )'
        	local select_lib_files="''${select_non_test_members}"'
            | select(.target.kind
                | contains(["cdylib"])
                or contains(["dylib"])
                or contains(["staticlib"])
            )
            | .filenames[]
            | select(endswith(".rlib") | not)
            | '"''${select_non_deps_artifact}"

          function installArtifacts() {
          	local loc=''${1?:missing}
          	mkdir -p "''${loc}"

          	while IFS= read -r to_install; do
          		echo "installing ''${to_install}"
          		cp "''${to_install}" "''${loc}"
          	done

          	rmdir --ignore-fail-on-non-empty "''${loc}"
          }

          echo "''${logs}" | ${lib.getExe jq} -r "''${select_lib_files}" | installArtifacts "''${dest}/lib/rofi"

          echo "installation complete"
        }

        installRofiPluginFromCargoBuildLog "$out" "$cargoBuildLog"
      '';
    })
