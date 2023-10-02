use std::collections::VecDeque;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use amxml::dom::{new_document, NodePtr};
use glib::warn;
use resolve_path::PathResolveExt;

use crate::error::UnwrapOrError;
use crate::G_LOG_DOMAIN;

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
static IDE_CODE_PATH: &str = "value/RecentProjectMetaInfo/option[@name=\"productionCode\"]";

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct RecentProject {
  name: String,
  path: PathBuf,
  icon: Option<PathBuf>,
  ide_code: String,
}

#[derive(Debug)]
pub struct RecentProjectsParser {
  nodes: VecDeque<NodePtr>,
}

impl RecentProjectsParser {
  pub fn from_file<T: AsRef<Path>>(path: T) -> Result<RecentProjectsParser, ()> {
    let xml = read_to_string(path).unwrap_or_error("Failed to read recent projects XML file")?;
    let document =
      new_document(&xml).unwrap_or_error("Failed to parse recent projects XML file")?;
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

    Ok(RecentProjectsParser { nodes })
  }
}

impl Iterator for RecentProjectsParser {
  type Item = RecentProject;

  fn next(&mut self) -> Option<Self::Item> {
    let raw_node = match self.nodes.pop_front() {
      Some(v) => v,
      None => return None,
    };

    // Extract project's path from the XML node
    let path = raw_node
      .attribute_value("value")
      .or(raw_node.attribute_value("key"))
      .map(|raw_path| raw_path.replace("$USER_HOME$", "~").resolve().to_path_buf())
      .and_then(|project_path| {
        if project_path.is_file() {
          project_path.parent().map(Path::to_path_buf)
        } else {
          Some(project_path)
        }
      });

    // Handle errors while extracting project's path
    let path = match path {
      Some(v) => v,
      None => {
        warn!("Failed to resolve project path from XML node: {raw_node:?}");
        return None;
      }
    };

    // Handle project's path validation
    match path.try_exists() {
      Ok(false) => {
        warn!("Ignoring XML node {raw_node:?}, path doesn't exists");
        return None;
      }
      Err(_) => {
        warn!("Ignoring XML node {raw_node:?}, insufficient permissions to access the path");
        return None;
      }
      _ => {}
    }

    // Resolve project's name using project's path
    let name_file = path.join(".idea/.name");
    let name = if name_file.is_file() {
      read_to_string(name_file)
        .ok()
        .map(|raw_name| raw_name.replace('\n', ""))
        .or(path.file_name().map(|v| v.to_string_lossy().to_string()))
    } else {
      path.file_name().map(|v| v.to_string_lossy().to_string())
    };

    // Handle errors while resolving project's name
    let name = match name {
      Some(v) => v,
      None => {
        warn!("Failed to resolve project name from XML node: {raw_node:?}");
        return None;
      }
    };

    // Extract project's IDE from the XML node and handle errors
    let ide_code = match raw_node
      .get_first_node(IDE_CODE_PATH)
      .and_then(|node| node.attribute_value("value"))
    {
      Some(v) => v,
      None => {
        warn!("Failed to extract IDE code from XML node: {raw_node:?}");
        return None;
      }
    };

    // Search for project's icon in project path and handle errors while building a matcher
    let icon = match globmatch::Builder::new(".idea/icon.*").build(&path) {
      Ok(matcher) => matcher.into_iter().flatten().next(),
      Err(_) => {
        warn!("Failed to build glob matcher for XML node: {raw_node:?}");
        None
      }
    };

    Some(RecentProject {
      name,
      path,
      icon,
      ide_code,
    })
  }
}
