use std::path::{Path, PathBuf};

use glib::warn;
use itertools::Itertools;
use resolve_path::PathResolveExt;

use crate::ide::IDEType;
use crate::macros::ensure_result;
use crate::rofi::xrmoptions::config_parse_option;
use crate::G_LOG_DOMAIN;

static ROFI_CONFIG_PREFIX: &str = "jetbrains-";

#[derive(Debug)]
pub struct Config {
  pub install_dir: PathBuf,
  pub custom_aliases: Vec<(String, IDEType)>,
}

impl Config {
  pub fn from_rofi() -> Self {
    let install_dir: Option<PathBuf> = config_parse_option(
      &(ROFI_CONFIG_PREFIX.to_owned() + "install-dir"),
      "A path to the directory where all IDEs are installed",
    );

    let custom_aliases = config_parse_option::<Option<Vec<String>>>(
      &(ROFI_CONFIG_PREFIX.to_owned() + "custom-aliases"),
      "A rofi list declaring custom IDE aliases",
    )
    .unwrap_or_default();

    let custom_aliases = custom_aliases
      .into_iter()
      .map(|raw| -> Result<_, _> {
        let (alias, product_code) = ensure_result!(
          raw.split(':').collect_tuple::<(&str, &str)>(),
          "Failed to parse custom alias, {:?} is not a valid alias",
          raw
        );

        let ide_type = ensure_result!(
          IDEType::from_product_code(product_code),
          "Failed to parse {:?} alias, {:?} is an unknown IDE",
          alias,
          product_code
        );

        Ok((alias.to_owned(), ide_type))
      })
      .filter_map(|res| match res {
        Ok(v) => Some(v),
        Err(err) => {
          warn!("{}", err);
          None
        }
      })
      .collect::<Vec<_>>();

    Self {
      install_dir: install_dir.unwrap_or_else(|| {
        Path::new("~/.local/share/JetBrains/Toolbox/apps/")
          .resolve()
          .to_path_buf()
      }),
      custom_aliases,
    }
  }
}
