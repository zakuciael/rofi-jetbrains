use std::collections::HashMap;
use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;
use std::sync::Arc;

use glib::{debug, warn, GlibLogger, GlibLoggerDomain, GlibLoggerFormat};
use itertools::Itertools;
use log::LevelFilter;
use resolve_path::PathResolveExt;
use rofi_mode::cairo::Surface;
use rofi_mode::{export_mode, Action, Api, Event, Matcher};
use strum::IntoEnumIterator;
use wax::{Glob, LinkBehavior, WalkEntry};

use crate::config::Config;
use crate::ide::data::IDEData;
use crate::ide::product_info::IDEProductInfo;
use crate::ide::properties::IDEProperties;
use crate::ide::IDEType;
use crate::recent_project::{RecentProject, RecentProjectsParser};
use crate::traits::MapToErrorLog;

mod config;
mod ide;
mod macros;
mod recent_project;
mod rofi;
mod traits;

pub static G_LOG_DOMAIN: &str = "Modes.JetBrains";

static GLIB_LOGGER: GlibLogger =
  GlibLogger::new(GlibLoggerFormat::Plain, GlibLoggerDomain::CrateTarget);

static RECENT_PROJECTS_GLOB_PATTERN: &str = "options/{recentProjects,recentSolutions}.xml";
static PRODUCT_INFO_GLOB_PATTERN: &str = "**/product-info.json";

export_mode!(Mode<'_>);

struct Mode<'rofi> {
  api: Api<'rofi>,
  projects: Vec<Arc<RecentProject>>,
  query: Option<IDEType>,
  entries: Vec<Arc<RecentProject>>,
  icon_cache: HashMap<IDEType, String>,
  aliases: HashMap<String, IDEType>,
}

impl<'rofi> rofi_mode::Mode<'rofi> for Mode<'rofi> {
  const NAME: &'static str = "jetbrains\0";

  fn init(api: Api<'rofi>) -> Result<Self, ()> {
    log::set_logger(&GLIB_LOGGER)
      .unwrap_or_else(|_| panic!("Another instance of the logger is already initialized"));
    log::set_max_level(LevelFilter::Debug);
    debug!("Starting..");

    debug!("Parsing config options...");
    let config = Config::from_rofi();

    let predefined_custom_aliases = vec![
      ("php".to_string(), IDEType::PHPStorm),
      ("py".to_string(), IDEType::PyCharm),
      ("web".to_string(), IDEType::WebStorm),
      ("go".to_string(), IDEType::GoLand),
      ("ruby".to_string(), IDEType::RubyMine),
      ("studio".to_string(), IDEType::AndroidStudio),
      ("android".to_string(), IDEType::AndroidStudio),
      ("rust".to_string(), IDEType::RustRover),
    ];

    let mut aliases = HashMap::<String, IDEType>::new();
    aliases.extend(IDEType::iter().map(|v| v.get_default_alias()));
    aliases.extend(predefined_custom_aliases);
    aliases.extend(config.custom_aliases.iter().cloned());

    debug!("Searching for installed IDEs..");
    let glob = Glob::new(PRODUCT_INFO_GLOB_PATTERN)
      .map_to_error_log("Failed to setup glob matcher for IDE product info")?;

    debug!("Looking for \"idea.properties\" under the user's home directory..");
    let home_properties = IDEProperties::from_file("~/idea.properties".resolve());

    if home_properties.is_none() {
      debug!("File was not found, skipping..");
    }

    let ides = glob
      .walk_with_behavior(&config.install_dir, LinkBehavior::ReadTarget)
      .flatten()
      .map(WalkEntry::into_path)
      .flat_map(|entry| -> Result<_, ()> {
        debug!("Parsing IDE data from {:?} file", &entry);
        let install_dir = entry.parent().map_to_error_log(format!(
          "Failed to resolve the parent directory for {:?} file ",
          &entry
        ))?;
        let product_info = IDEProductInfo::from_file(&entry)?;
        let config_path = home_properties
          .as_ref()
          .and_then(|props| props.config_path.clone())
          .or_else(|| {
            IDEProperties::from_file(install_dir.join("bin/idea.properties"))
              .and_then(|props| props.config_path)
          })
          .unwrap_or_else(|| {
            (if product_info.ide_type == IDEType::AndroidStudio {
              "~/.config/Google"
            } else {
              "~/.config/JetBrains"
            })
            .resolve()
            .to_path_buf()
          });

        Ok(IDEData::from_product_info(
          &product_info,
          install_dir,
          config_path,
        ))
      })
      .map(Arc::new)
      .collect::<Vec<_>>();

    debug!("Searching for recent project..");
    let mut projects = vec![];

    for ide in ides.iter() {
      let glob = Glob::new(RECENT_PROJECTS_GLOB_PATTERN).map_to_error_log(format!(
        "Failed to setup glob matcher for recent projects, {:?} is an invalid path",
        &ide.config_path
      ))?;

      projects.extend(
        glob
          .walk_with_behavior(&ide.config_path, LinkBehavior::ReadTarget)
          .flatten()
          .map(WalkEntry::into_path)
          .flat_map(|entry| {
            debug!("Reading recent projects XML file {entry:?}..");
            RecentProjectsParser::from_file(entry, ide.clone())
          })
          .flatten()
          .filter_map(|result| match result {
            // Log errors returned by the RecentProjectsParser's iterator and skip those entries
            Ok(v) => Some(v),
            Err(err) => {
              warn!("{}", err);
              None
            }
          }),
      );
    }

    let projects = projects
      .into_iter()
      .sorted_by(|a, b| Ord::cmp(&b.last_opened, &a.last_opened))
      .unique()
      .map(Arc::new)
      .collect::<Vec<_>>();

    let entries = projects.iter().map(Arc::clone).collect::<Vec<_>>();

    Ok(Self {
      api,
      projects,
      entries,
      aliases,
      query: None,
      icon_cache: HashMap::new(),
    })
  }

  fn entries(&mut self) -> usize {
    self.entries.len()
  }

  fn entry_content(&self, line: usize) -> rofi_mode::String {
    self.entries[line].name.clone().into()
  }

  fn entry_icon(&mut self, line: usize, size: u32) -> Option<Surface> {
    let project = &self.entries[line];

    if let Some(icon) = project
      .icon
      .as_ref()
      .map(|path| path.to_string_lossy().to_string())
    {
      return self.api.query_icon(&icon, size).wait(&mut self.api);
    }

    let ide = project.ide.clone();
    if let Some(icon_name) = self.icon_cache.get(&ide.ide_type) {
      self.api.query_icon(icon_name, size).wait(&mut self.api)
    } else {
      self
        .api
        .query_icon(&ide.icon_name, size)
        .wait(&mut self.api)
        .map(|icon| (icon, ide.icon_name.to_owned()))
        .or_else(|| {
          let icon_name = project.ide.icon_name.replace("jetbrains-", "");
          self
            .api
            .query_icon(&icon_name, size)
            .wait(&mut self.api)
            .map(|icon| (icon, icon_name))
        })
        .or_else(|| {
          let icon_name = project.ide.fallback_icon_path.to_string_lossy().to_string();
          self
            .api
            .query_icon(&icon_name, size)
            .wait(&mut self.api)
            .map(|icon| (icon, icon_name))
        })
        .map(|(icon, icon_name)| {
          debug!(
            "Caching icon name for {:?} to {:?}",
            &ide.ide_type, &icon_name
          );
          self.icon_cache.insert(ide.ide_type.clone(), icon_name);

          icon
        })
    }
  }

  fn react(&mut self, event: Event, input: &mut rofi_mode::String) -> Action {
    debug!("Received event {:?} with input {:?}", event, input);

    match event {
      Event::Ok { selected, .. } => {
        let project = &self.entries[selected];

        Command::new(&project.ide.launcher_path)
          .arg(&project.path)
          .stdout(process::Stdio::null())
          .stderr(process::Stdio::null())
          .process_group(0)
          .spawn()
          .map_to_error_log(format!(
            "Failed to spawn IDE with the project: {:?}",
            &project.path
          ))
          .unwrap();

        Action::Exit
      }
      Event::CustomInput { alt, .. } => {
        if self.query.is_none() || alt {
          let query = input.split(' ').next().unwrap();
          debug!(
            "Attempting to set results into query-mode using {:?} as query..",
            query
          );

          if let Some(ide) = self
            .aliases
            .get(query)
            .cloned()
            .or_else(|| IDEType::from_product_code(query))
          {
            self.query = Some(ide);
            self.entries = self
              .projects
              .iter()
              .filter(|project| &project.ide.ide_type == self.query.as_ref().unwrap())
              .map(Arc::clone)
              .collect();

            debug!(
              "Results set to query-mode, displaying results for IDE: {:?}",
              self.query.as_ref().unwrap()
            );
            Action::Reset
          } else {
            debug!("Aborting change, no valid IDE found in the input");
            Action::Reload
          }
        } else {
          self.query = None;
          self.entries = self.projects.iter().map(Arc::clone).collect();

          debug!("Results set into normal mode, requested by user");
          Action::Reload
        }
      }
      Event::Cancel { .. } => Action::Exit,
      _ => Action::Reload,
    }
  }

  fn matches(&self, line: usize, matcher: Matcher<'_>) -> bool {
    // TODO: Better matching for user input
    matcher.matches(&self.entries[line].name)
  }
}
