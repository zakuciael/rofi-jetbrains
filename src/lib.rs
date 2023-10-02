use glib::{debug, GlibLogger, GlibLoggerDomain, GlibLoggerFormat};
use itertools::Itertools;
use log::LevelFilter;
use rofi_mode::cairo::Surface;
use rofi_mode::{export_mode, Action, Api, Event, Matcher};

use crate::config::Config;
use crate::error::UnwrapOrError;
use crate::recent_project::{RecentProject, RecentProjectsParser};

mod config;
mod error;
mod ide;
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
  entries: Vec<RecentProject>,
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

    // TODO: Sort by last opened time
    debug!("Searching for recent project..");
    let entries = vec![&config.configs_path, &config.android_studio_config_path]
      .iter()
      .flat_map(|path| {
        globmatch::Builder::new(RECENT_PROJECTS_GLOB_PATTERN)
          .build(path)
          .unwrap_or_error(format!(
            "Unable to find recent projects, {path:?} is invalid"
          ))
      })
      .flat_map(|matcher| {
        matcher
          .into_iter()
          .flatten()
          .flat_map(|entry| {
            debug!("Reading recent projects XML file {entry:?}..");
            RecentProjectsParser::from_file(entry)
          })
          .flatten()
      })
      .dedup()
      .collect::<Vec<_>>();

    Ok(Self {
      api,
      config,
      entries,
    })
  }

  fn entries(&mut self) -> usize {
    self.entries.len()
  }

  fn entry_content(&self, line: usize) -> rofi_mode::String {
    let project = &self.entries[line];
    project.name.clone().into()
  }

  fn entry_icon(&mut self, line: usize, size: u32) -> Option<Surface> {
    // TODO: Resolve IDE icon from bin folder
    let project = &self.entries[line];
    let data = project.ide.get_data();

    self
      .api
      .query_icon(&data.icon_name, size)
      .wait(&mut self.api)
  }

  fn react(&mut self, _event: Event, _input: &mut rofi_mode::String) -> Action {
    // TODO: Handle user input
    Action::Exit
  }

  fn matches(&self, line: usize, matcher: Matcher<'_>) -> bool {
    // TODO: Better matching for user input
    matcher.matches(self.entries[line].name.as_str())
  }

  fn preprocess_input(&mut self, input: &str) -> rofi_mode::String {
    // TODO: Handle IDE queries and aliases
    input.into()
  }
}
