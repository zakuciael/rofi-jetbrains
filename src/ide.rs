#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone)]
pub enum IDE {
  Aqua(IDEData),
  CLion(IDEData),
  IntelliJIDEA(IDEData),
  IntelliJIDEACommiunity(IDEData),
  PHPStorm(IDEData),
  PyCharm(IDEData),
  PyCharmCommiunity(IDEData),
  Rider(IDEData),
  WebStorm(IDEData),
  GoLand(IDEData),
  DataGrip(IDEData),
  DataSpell(IDEData),
  RubyMine(IDEData),
  AndroidStudio(IDEData),
  RustRover(IDEData),
  MPS(IDEData),
}

// TODO: Add an option to specify installation path prefix
#[derive(Debug, Clone)]
pub struct IDEData {
  pub shell_script_names: Vec<String>,
  pub icon_name: String,
}

impl IDEData {
  pub fn new<S: AsRef<str>, I: AsRef<str>>(shell_script_names: Vec<S>, icon_name: I) -> Self {
    Self {
      shell_script_names: shell_script_names
        .iter()
        .map(|v| v.as_ref().to_string())
        .collect(),
      icon_name: icon_name.as_ref().to_string(),
    }
  }
}

impl IDE {
  pub fn get_data(&self) -> &IDEData {
    match self {
      IDE::Aqua(v) => v,
      IDE::CLion(v) => v,
      IDE::IntelliJIDEA(v) => v,
      IDE::IntelliJIDEACommiunity(v) => v,
      IDE::PHPStorm(v) => v,
      IDE::PyCharm(v) => v,
      IDE::PyCharmCommiunity(v) => v,
      IDE::Rider(v) => v,
      IDE::WebStorm(v) => v,
      IDE::GoLand(v) => v,
      IDE::DataGrip(v) => v,
      IDE::DataSpell(v) => v,
      IDE::RubyMine(v) => v,
      IDE::AndroidStudio(v) => v,
      IDE::RustRover(v) => v,
      IDE::MPS(v) => v,
    }
  }

  pub fn from_code<T: AsRef<str>>(code: T) -> Option<Self> {
    // TODO: Uncomment IDEs when either the icon fetching is implemented or my theme adds those icons
    match code.as_ref() {
      "RD" => Some(IDE::Rider(IDEData::new(vec!["rider"], "rider"))),
      "AI" => Some(IDE::AndroidStudio(IDEData::new(
        vec!["studio"],
        "android-studio",
      ))),
      "CL" => Some(IDE::CLion(IDEData::new(vec!["clion"], "clion"))),
      "RR" => Some(IDE::RustRover(IDEData::new(vec!["rustrover"], "rustrover"))),
      "WS" => Some(IDE::WebStorm(IDEData::new(vec!["webstorm"], "webstorm"))),
      "RM" => Some(IDE::RubyMine(IDEData::new(vec!["rubymine"], "rubymine"))),
      "PC" => Some(IDE::PyCharmCommiunity(IDEData::new(
        vec!["pycharm"],
        "pycharm-community",
      ))),
      "PY" => Some(IDE::PyCharm(IDEData::new(vec!["pycharm"], "pycharm"))),
      "PS" => Some(IDE::PHPStorm(IDEData::new(vec!["phpstorm"], "phpstorm"))),
      // "MPS" => Some(IDE::MPS(IDEData::new(vec!["mps"], "mps"))),
      "IU" => Some(IDE::IntelliJIDEA(IDEData::new(
        vec!["idea"],
        "intellij-idea",
      ))),
      "IC" => Some(IDE::IntelliJIDEACommiunity(IDEData::new(
        vec!["idea"],
        "intellij-idea-community",
      ))),
      // "DS" => Some(IDE::DataSpell(IDEData::new(vec!["dataspell"], "dataspell"))),
      "DB" => Some(IDE::DataGrip(IDEData::new(vec!["datagrip"], "datagrip"))),
      // "QA" => Some(IDE::Aqua(IDEData::new(vec!["aqua"], "aqua"))),
      "GO" => Some(IDE::GoLand(IDEData::new(vec!["goland"], "goland"))),
      _ => None,
    }
  }
}
