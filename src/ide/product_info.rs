use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::Path;

use glib::warn;
use serde::Deserialize;

use crate::ide::IDEType;
use crate::G_LOG_DOMAIN;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IDELaunchSettings {
  pub launcher_path: String,
  pub startup_wm_class: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IDEProductInfo {
  pub name: String,
  pub version: String,
  pub build_number: String,
  #[serde(rename = "productCode")]
  pub ide_type: IDEType,
  pub data_directory_name: String,
  pub svg_icon_path: String,
  #[serde(rename = "launch")]
  pub launch_settings: Vec<IDELaunchSettings>,
}

impl IDEProductInfo {
  pub fn from_file<T: AsRef<Path>>(path: T) -> Result<IDEProductInfo, ()> {
    let file = File::open(path.as_ref()).map_err(|err| match err {
      _ if err.kind() == ErrorKind::NotFound => {
        warn!(
          "Failed to read {:?} file, file no longer exists",
          path.as_ref()
        );
      }
      _ if err.kind() == ErrorKind::PermissionDenied => {
        warn!(
          "Failed to read {:?} file, insufficient permissions",
          path.as_ref()
        );
      }
      _ => (),
    })?;
    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
      .map_err(|err| warn!("Failed to parse {:?} file, reason: {}", path.as_ref(), err))
  }
}
