use glib::{debug, GlibLogger, GlibLoggerDomain, GlibLoggerFormat};
use log::LevelFilter;
use rofi_mode::{export_mode, Action, Api, Event, Matcher};

use crate::config::Config;
use crate::error::UnwrapOrError;

mod config;
mod error;
mod rofi;
mod traits;

pub static G_LOG_DOMAIN: &str = "Modes.JetBrains";

static GLIB_LOGGER: GlibLogger =
  GlibLogger::new(GlibLoggerFormat::Plain, GlibLoggerDomain::CrateTarget);

export_mode!(Mode<'_>);

struct Mode<'rofi> {
  api: Api<'rofi>,
  config: Config,
  entries: Vec<String>,
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

    debug!("Parsing config file...");
    let config = Config::default();

    Ok(Self {
      api,
      config,
      entries: vec!["Test str".to_string()],
    })
  }

  fn entries(&mut self) -> usize {
    self.entries.len()
  }

  fn entry_content(&self, line: usize) -> rofi_mode::String {
    self.entries[line].clone().into()
  }

  fn react(&mut self, event: Event, input: &mut rofi_mode::String) -> Action {
    Action::Exit
  }

  fn matches(&self, line: usize, matcher: Matcher<'_>) -> bool {
    matcher.matches(self.entries[line].as_str())
  }
}
