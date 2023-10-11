use std::fs::File;
use std::io::{BufReader, ErrorKind};
use std::path::{Path, PathBuf};

use glib::warn;
use serde::Deserialize;

use crate::ide::de::evaluate_property;
use crate::G_LOG_DOMAIN;

#[derive(Deserialize, Debug)]
pub struct IDEProperties {
  #[serde(default)]
  #[serde(rename = "idea.config.path", deserialize_with = "evaluate_property")]
  pub config_path: Option<PathBuf>,
}

impl IDEProperties {
  pub fn from_file<T: AsRef<Path>>(path: T) -> Option<Self> {
    let file = File::open(path.as_ref())
      .map_err(|err| match err {
        _ if err.kind() == ErrorKind::NotFound => {}
        _ if err.kind() == ErrorKind::PermissionDenied => warn!(
          "Failed to read {:?} file, insufficient permissions",
          path.as_ref()
        ),
        _ => warn!("Failed to read {:?} file, reason: {}", path.as_ref(), err),
      })
      .ok()?;

    let reader = BufReader::new(file);

    serde_java_properties::from_reader(reader)
      .map_err(|err| warn!("Failed to parse {:?} file, reason: {}", path.as_ref(), err))
      .ok()
  }
}
