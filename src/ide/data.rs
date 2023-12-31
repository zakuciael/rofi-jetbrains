use std::path::{Path, PathBuf};

use crate::ide::product_info::IDEProductInfo;
use crate::ide::IDEType;

#[derive(Debug, Clone)]
pub struct IDEData {
  pub ide_type: IDEType,
  pub version: String,
  pub config_path: PathBuf,
  pub fallback_icon_path: PathBuf,
  pub icon_name: String,
  pub launcher_path: PathBuf,
}

impl IDEData {
  pub fn from_product_info<A, B>(
    product_info: &IDEProductInfo,
    install_dir: A,
    config_path: B,
  ) -> Self
  where
    A: AsRef<Path>,
    B: AsRef<Path>,
  {
    let config_path = config_path.as_ref();
    let install_dir = install_dir.as_ref();

    let launch_settings = &product_info.launch_settings[0];

    Self {
      ide_type: product_info.ide_type.clone(),
      version: product_info.version.clone(),
      config_path: config_path.join(&product_info.data_directory_name),
      fallback_icon_path: install_dir.join(&product_info.svg_icon_path),
      icon_name: launch_settings.startup_wm_class.clone(),
      launcher_path: install_dir.join(&launch_settings.launcher_path),
    }
  }
}
