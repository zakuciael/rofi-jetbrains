use std::collections::VecDeque;
use std::fs::read_to_string;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use amxml::dom::{new_document, NodePtr};
use chrono::{DateTime, Local, NaiveDateTime};
use resolve_path::PathResolveExt;

use crate::ide::data::IDEData;
use crate::macros::ensure;
use crate::traits::MapToErrorLog;

static BASE_PATHS: [&str; 4] = [
  ".//component[@name=\"RecentProjectsManager\"][1]",
  ".//component[@name=\"RecentDirectoryProjectsManager\"][1]",
  ".//component[@name=\"RiderRecentProjectsManager\"][1]",
  ".//component[@name=\"RiderRecentDirectoryProjectsManager\"][1]",
];
static ENTRY_PATHS: [&str; 3] = [
  "option[@name=\"recentPaths\"]/list/option",
  "option[@name=\"additionalInfo\"]/map/entry",
  "option[@name=\"groups\"]/list/ProjectGroup/option[@name=\"projects\"]/list/option",
];

static LAST_OPENED_TIMESTAMP_PATH: &str =
  "value/RecentProjectMetaInfo/option[@name=\"projectOpenTimestamp\"]";

#[derive(Debug, Clone)]
pub struct RecentProject {
  pub name: String,
  pub path: PathBuf,
  pub icon: Option<PathBuf>,
  pub ide: Arc<IDEData>,
  pub last_opened: DateTime<Local>,
}

#[derive(Debug)]
pub struct RecentProjectsParser {
  ide: Arc<IDEData>,
  nodes: VecDeque<NodePtr>,
}

impl PartialEq for RecentProject {
  fn eq(&self, other: &Self) -> bool {
    // We can ignore "name", "icon" as it should be always evaluated to the same value given the same project path
    // Also don't compare the "last_opened" prop as it might vary between entries for the same project
    // Instead let's update the value to the most recent timestamp
    self.path == other.path && self.ide.ide_type == other.ide.ide_type
  }
}

impl Eq for RecentProject {}

impl Hash for RecentProject {
  fn hash<H: Hasher>(&self, state: &mut H) {
    // See notes for the "PartialEq" implementation to read why only these props are hashed
    Hash::hash(&self.path, state);
    Hash::hash(&self.ide.ide_type, state);
  }
}

impl RecentProjectsParser {
  pub fn from_file<T: AsRef<Path>>(path: T, ide: Arc<IDEData>) -> Result<RecentProjectsParser, ()> {
    let xml = read_to_string(path).map_to_error_log("Failed to read recent projects XML file")?;
    let document =
      new_document(&xml).map_to_error_log("Failed to parse recent projects XML file")?;
    let root = document.root_element();

    let nodes = BASE_PATHS
      .iter()
      .filter_map(|base_path| root.get_first_node(base_path))
      .flat_map(|base_node| {
        ENTRY_PATHS
          .iter()
          .filter_map(move |entry_path| base_node.get_nodeset(entry_path).ok())
      })
      .flatten()
      .collect::<VecDeque<_>>();

    Ok(RecentProjectsParser { ide, nodes })
  }
}

impl Iterator for RecentProjectsParser {
  type Item = Result<RecentProject, String>;

  fn next(&mut self) -> Option<Self::Item> {
    let raw_node = match self.nodes.pop_front() {
      Some(v) => v,
      None => return None,
    };

    let mut name: Option<String> = None;

    // Extract project's path and optionally its name (from the .sln file)
    let path = ensure!(
      raw_node
        .attribute_value("value")
        .or(raw_node.attribute_value("key"))
        .map(|raw_path| {
          let path = raw_path.replace("$USER_HOME$", "~").resolve().to_path_buf();
          if path.is_file() {
            name = path
              .file_stem()
              .map(|name| name.to_string_lossy().to_string());
          }

          path
        }),
      "Failed to resolve project path from XML node: {raw_node:?}"
    );

    // Validate if project's path exists
    match path.try_exists() {
      Ok(false) => {
        return Some(Err(format!(
          "Ignoring XML node {raw_node:?}, path doesn't exists"
        )));
      }
      Err(_) => {
        return Some(Err(format!(
          "Ignoring XML node {raw_node:?}, insufficient permissions to access the path"
        )));
      }
      _ => {}
    }

    // Resolve project's name
    let name = ensure!(
      match read_to_string(path.join(".idea/.name")) {
        Ok(raw_name) => Some(raw_name.replace('\n', "")),
        Err(_) => {
          name.or_else(|| path.file_name().map(|v| v.to_string_lossy().to_string()))
        }
      },
      "Failed to resolve project name from XML node: {raw_node:?}"
    );

    // Extract project's last opened timestamp
    let last_opened = ensure!(
      raw_node
        .get_first_node(LAST_OPENED_TIMESTAMP_PATH)
        .and_then(|node| node.attribute_value("value"))
        .map(|raw| raw.parse::<i64>().ok())
        .and_then(|timestamp| {
          timestamp.map(|timestamp| {
            NaiveDateTime::from_timestamp_millis(timestamp)
              .as_ref()
              .map(NaiveDateTime::and_utc)
              .map(|utc| utc.with_timezone(&Local))
          })
        })
        .flatten(),
      "Failed to extract last opened time from XML node: {raw_node:?}"
    );

    // Resolve project's custom icon from project's path
    let icon = ensure!(
      globmatch::Builder::new(".idea/icon.*")
        .build(&path)
        .ok()
        .map(|matcher| matcher.into_iter().flatten().next()),
      "Failed to build glob matcher for XML node: {raw_node:?}"
    );

    Some(Ok(RecentProject {
      name,
      path,
      icon,
      ide: self.ide.clone(),
      last_opened,
    }))
  }
}
