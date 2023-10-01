use std::path::{Path, PathBuf};

use resolve_path::PathResolveExt;

use crate::rofi::xrmoptions::config_parse_option;

static ROFI_CONFIG_PREFIX: &str = "jetbrains-";

#[derive(Debug)]
pub struct Config {
  pub shell_scripts_path: PathBuf,
  pub configs_path: PathBuf,
  pub android_studio_config_path: PathBuf,
}

impl Config {
  pub fn from_rofi() -> Self {
    let shell_scripts_path = config_parse_option::<String>(
      &(ROFI_CONFIG_PREFIX.to_owned() + "shell-scripts-path"),
      "A path to the JetBrains IDE shell scripts directory",
    )
    .map(|raw| raw.resolve().to_path_buf());

    let configs_path = config_parse_option::<String>(
      &(ROFI_CONFIG_PREFIX.to_owned() + "configs-path"),
      "A path to the JetBrains IDE configs directory",
    )
    .map(|raw| raw.resolve().to_path_buf());

    let android_studio_config_path = config_parse_option::<String>(
      &(ROFI_CONFIG_PREFIX.to_owned() + "android-studio-config-path"),
      "A path to the Android Studio config directory",
    )
    .map(|raw| raw.resolve().to_path_buf());

    Self {
      shell_scripts_path: shell_scripts_path.unwrap_or(
        Path::new("~/.local/share/JetBrains/Toolbox/scripts")
          .resolve()
          .to_path_buf(),
      ),
      configs_path: configs_path
        .unwrap_or(Path::new("~/.config/JetBrains").resolve().to_path_buf()),
      android_studio_config_path: android_studio_config_path
        .unwrap_or(Path::new("~/.config/Google").resolve().to_path_buf()),
    }
  }
}
