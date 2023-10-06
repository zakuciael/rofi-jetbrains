use std::os::unix::process::CommandExt;
use std::process;
use std::process::Command;
use std::sync::Arc;

use glib::{debug, warn, GlibLogger, GlibLoggerDomain, GlibLoggerFormat};
use itertools::Itertools;
use log::LevelFilter;
use rofi_mode::cairo::Surface;
use rofi_mode::{export_mode, Action, Api, Event, Matcher};

use crate::config::Config;
use crate::error::MapToErrorLog;
use crate::ide::IDE;
use crate::recent_project::{RecentProject, RecentProjectsParser};

mod config;
mod error;
mod ide;
mod macros;
mod recent_project;
mod rofi;
mod traits;

pub static G_LOG_DOMAIN: &str = "Modes.JetBrains";

static GLIB_LOGGER: GlibLogger =
  GlibLogger::new(GlibLoggerFormat::Plain, GlibLoggerDomain::CrateTarget);

static RECENT_PROJECTS_GLOB_PATTERN: &str = "./**/options/{recentProjects,recentSolutions}.xml";

export_mode!(Mode<'_>);

struct Mode<'rofi> {
  api: Api<'rofi>,
  config: Config,
  found_projects: Vec<Arc<RecentProject>>,
  query: Option<IDE>,
  entries: Vec<Arc<RecentProject>>,
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

    debug!("Searching for recent project..");
    let matchers = vec![&config.configs_path, &config.android_studio_config_path]
      .iter()
      .map(|config_path| {
        globmatch::Builder::new(RECENT_PROJECTS_GLOB_PATTERN)
          .build(config_path)
          .map_to_error_log(format!(
            "Failed to setup glob matcher for recent projects, {config_path:?} is an invalid path"
          ))
      })
      .collect::<Result<Vec<_>, _>>()?;

    let found_projects = matchers
      .into_iter()
      .flat_map(|matcher| matcher.into_iter().flatten())
      .flat_map(|entry| {
        debug!("Reading recent projects XML file {entry:?}..");
        RecentProjectsParser::from_file(entry)
      })
      .flatten()
      .filter_map(|result| match result {
        // Log errors returned by the RecentProjectsParser's iterator and skip those entries
        Ok(v) => Some(v),
        Err(err) => {
          warn!("{}", err);
          None
        }
      })
      .filter(|project| {
        if project
          .ide
          .get_shell_script(&config.shell_scripts_path)
          .is_none()
        {
          warn!("Ignoring project {:?}, IDE is not installed", project.path);
          return false;
        }

        true
      })
      .sorted_by(|a, b| Ord::cmp(&b.last_opened, &a.last_opened))
      .unique()
      .map(Arc::new)
      .collect::<Vec<_>>();

    let entries = found_projects.iter().map(Arc::clone).collect::<Vec<_>>();

    Ok(Self {
      api,
      config,
      found_projects,
      entries,
      query: None,
    })
  }

  fn entries(&mut self) -> usize {
    self.entries.len()
  }

  fn entry_content(&self, line: usize) -> rofi_mode::String {
    self.entries[line].name.clone().into()
  }

  fn entry_icon(&mut self, line: usize, size: u32) -> Option<Surface> {
    // TODO: Resolve IDE icon from bin folder
    let project = &self.entries[line];

    if let Some(icon) = project
      .icon
      .as_ref()
      .map(|path| path.to_string_lossy().to_string())
    {
      return self.api.query_icon(&icon, size).wait(&mut self.api);
    }

    self
      .api
      .query_icon(&project.ide.get_data().icon_name, size)
      .wait(&mut self.api)
  }

  fn react(&mut self, event: Event, input: &mut rofi_mode::String) -> Action {
    debug!("Received event {:?} with input {:?}", event, input);
    // TODO: Handle IDE aliases

    match event {
      Event::Ok { selected, .. } => {
        let project = &self.entries[selected];

        if let Some(shell_script) = project
          .ide
          .get_shell_script(&self.config.shell_scripts_path)
        {
          Command::new(shell_script)
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
        }

        Action::Exit
      }
      Event::CustomInput { alt, .. } => {
        if self.query.is_none() || alt {
          debug!("Attempting to set results into query-mode..");
          if let Some(Some(ide)) = input.split(' ').next().map(IDE::from_code) {
            self.query = Some(ide);
            self.entries = self
              .found_projects
              .iter()
              .filter(|project| &project.ide == self.query.as_ref().unwrap())
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
          self.entries = self.found_projects.iter().map(Arc::clone).collect();

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
