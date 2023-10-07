use std::path::{Path, PathBuf};

use resolve_path::PathResolveExt;

use crate::rofi::xrmoptions::config_parse_option;

static ROFI_CONFIG_PREFIX: &str = "jetbrains-";

#[derive(Debug)]
pub struct Config {
  pub install_dir: PathBuf,
}

impl Config {
  pub fn from_rofi() -> Self {
    // TODO: Add configuration option for IDE aliases
    let install_dir = config_parse_option::<String>(
      &(ROFI_CONFIG_PREFIX.to_owned() + "install-dir"),
      "A path to the directory where all IDEs are installed",
    )
    .map(|raw| raw.resolve().to_path_buf());

    Self {
      install_dir: install_dir.unwrap_or_else(|| {
        Path::new("~/.local/share/JetBrains/Toolbox/apps/")
          .resolve()
          .to_path_buf()
      }),
    }
  }
}
