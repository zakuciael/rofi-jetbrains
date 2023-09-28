use std::path::{Path, PathBuf};

use glib::{debug, error};
use resolve_path::PathResolveExt;

use crate::rofi::xrmoptions::config_parse_option;
use crate::G_LOG_DOMAIN;

static ROFI_CONFIG_PREFIX: &str = "jetbrains-";

type IDEAlias = (String, String);

#[derive(Debug)]
pub struct Config {
  pub shell_scripts_path: PathBuf,
  pub configs_path: PathBuf,
  pub android_studio_config_path: PathBuf,
  pub ide_aliases: Vec<IDEAlias>,
}

#[derive(Default, Debug)]
pub struct ConfigBuilder {
  shell_scripts_path: Option<PathBuf>,
  configs_path: Option<PathBuf>,
  android_studio_config_path: Option<PathBuf>,
  ide_aliases: Option<Vec<IDEAlias>>,
}

impl Config {
  pub fn builder() -> ConfigBuilder {
    ConfigBuilder::new()
  }

  pub fn default() -> Self {
    ConfigBuilder::new().with_rofi_config().build()
  }
}

impl ConfigBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn build(self) -> Config {
    // We can ignore the `.unwrap()`
    // calls because when declaring the defaults we know it will not panic.

    // TODO: Add default aliases
    Config {
      shell_scripts_path: self.shell_scripts_path.unwrap_or(
        Path::new("~/.local/share/JetBrains/Toolbox/scripts")
          .resolve()
          .to_path_buf(),
      ),
      configs_path: self
        .configs_path
        .unwrap_or(Path::new("~/.config/JetBrains").resolve().to_path_buf()),
      android_studio_config_path: self
        .android_studio_config_path
        .unwrap_or(Path::new("~/.config/Google").resolve().to_path_buf()),
      ide_aliases: self.ide_aliases.unwrap_or_default(),
    }
  }

  pub fn with_rofi_config(mut self) -> Self {
    let shell_scripts_path_key = ROFI_CONFIG_PREFIX.to_owned() + "shell-scripts-path";
    let configs_path_key = ROFI_CONFIG_PREFIX.to_owned() + "configs-path";
    let android_studio_config_path_key =
      ROFI_CONFIG_PREFIX.to_owned() + "android-studio-config-path";
    let ide_aliases_key = ROFI_CONFIG_PREFIX.to_owned() + "ide-aliases";

    let shell_scripts_path = config_parse_option::<String>(
      &shell_scripts_path_key,
      "A path to the JetBrains IDE shell scripts directory",
    )
    .map(PathBuf::from)
    .map(|path| {
      let path = path.resolve();
      if !path.is_dir() {
        error!("Invalid \"{shell_scripts_path_key}\" config value, not a valid directory");
        panic!();
      }

      path.to_path_buf()
    });
    let configs_path = config_parse_option::<String>(
      &configs_path_key,
      "A path to the JetBrains IDE configs directory",
    )
    .map(PathBuf::from)
    .map(|path| {
      let path = path.resolve();
      if !path.is_dir() {
        error!("Invalid \"{configs_path_key}\" config value, not a valid directory");
        panic!();
      }

      path.to_path_buf()
    });
    let android_studio_config_path = config_parse_option::<String>(
      &android_studio_config_path_key,
      "A path to the Android Studio config directory",
    )
    .map(PathBuf::from)
    .map(|path| path.resolve().to_path_buf());

    // TODO: Implement parsing for raw aliases
    let ide_aliases =
      config_parse_option::<String>(&ide_aliases_key, "A comma-separated list of IDE aliases").map(
        |raw| {
          debug!("Found raw aliases: {:?}", raw);
          vec![]
        },
      );

    self.shell_scripts_path = shell_scripts_path.or(self.shell_scripts_path);
    self.configs_path = configs_path.or(self.configs_path);
    self.android_studio_config_path =
      android_studio_config_path.or(self.android_studio_config_path);
    self.ide_aliases = ide_aliases.or(self.ide_aliases);

    self
  }

  pub fn with_shell_scripts_path<T: AsRef<Path>>(mut self, path: T) -> Self {
    self.shell_scripts_path = Some(path.as_ref().into());
    self
  }

  pub fn with_configs_path<T: AsRef<Path>>(mut self, path: T) -> Self {
    self.configs_path = Some(path.as_ref().into());
    self
  }

  pub fn with_android_studio_config_path<T: AsRef<Path>>(mut self, path: T) -> Self {
    self.android_studio_config_path = Some(path.as_ref().into());
    self
  }

  pub fn with_ide_aliases(mut self, aliases: Vec<IDEAlias>) -> Self {
    if aliases.is_empty() {
      self.ide_aliases = None;
    } else {
      self.ide_aliases = Some(aliases);
    }

    self
  }

  pub fn with_alias(mut self, alias: IDEAlias) -> Self {
    self.ide_aliases = self.ide_aliases.map(|mut v| {
      v.push(alias);
      v
    });

    self
  }
}
