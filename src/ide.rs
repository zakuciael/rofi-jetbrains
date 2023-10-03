use std::collections::HashMap;

use lazy_static::lazy_static;

lazy_static! {
  static ref IDE_DATA: HashMap<IDE, IDEData> = {
    let mut map = HashMap::new();
    map.insert(IDE::Rider, IDEData::new(vec!["rider"], "rider"));
    map.insert(
      IDE::AndroidStudio,
      IDEData::new(vec!["studio"], "android-studio"),
    );
    map.insert(IDE::CLion, IDEData::new(vec!["clion"], "clion"));
    map.insert(IDE::RustRover, IDEData::new(vec!["rustrover"], "rustrover"));
    map.insert(IDE::WebStorm, IDEData::new(vec!["webstorm"], "webstorm"));
    map.insert(IDE::RubyMine, IDEData::new(vec!["rubymine"], "rubymine"));
    map.insert(
      IDE::PyCharmCommiunity,
      IDEData::new(vec!["pycharm"], "pycharm-community"),
    );
    map.insert(IDE::PyCharm, IDEData::new(vec!["pycharm"], "pycharm"));
    map.insert(IDE::PHPStorm, IDEData::new(vec!["phpstorm"], "phpstorm"));
    map.insert(IDE::MPS, IDEData::new(vec!["mps"], "mps"));
    map.insert(
      IDE::IntelliJIDEA,
      IDEData::new(vec!["idea"], "intellij-idea"),
    );
    map.insert(
      IDE::IntelliJIDEACommiunity,
      IDEData::new(vec!["idea"], "intellij-idea-community"),
    );
    map.insert(IDE::DataSpell, IDEData::new(vec!["dataspell"], "dataspell"));
    map.insert(IDE::DataGrip, IDEData::new(vec!["datagrip"], "datagrip"));
    map.insert(IDE::Aqua, IDEData::new(vec!["aqua"], "aqua"));
    map.insert(IDE::GoLand, IDEData::new(vec!["goland"], "goland"));
    map
  };
}

// static IDE_DATA: HashMap<IDE, IDEData> = HashMap::from([(IDE::Rider, IDEData::new(vec![""], ""))]);

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IDE {
  Aqua,
  CLion,
  IntelliJIDEA,
  IntelliJIDEACommiunity,
  PHPStorm,
  PyCharm,
  PyCharmCommiunity,
  Rider,
  WebStorm,
  GoLand,
  DataGrip,
  DataSpell,
  RubyMine,
  AndroidStudio,
  RustRover,
  MPS,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IDEData {
  // TODO: Add an option to specify installation path prefix
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
    &(*IDE_DATA)[self]
  }

  pub fn from_code<T: AsRef<str>>(code: T) -> Option<Self> {
    // TODO: Uncomment IDEs when either the icon fetching is implemented or my theme adds those icons
    let enum_value = match code.as_ref() {
      "RD" => IDE::Rider,
      "AI" => IDE::AndroidStudio,
      "CL" => IDE::CLion,
      "RR" => IDE::RustRover,
      "WS" => IDE::WebStorm,
      "RM" => IDE::RubyMine,
      "PC" => IDE::PyCharmCommiunity,
      "PY" => IDE::PyCharm,
      "PS" => IDE::PHPStorm,
      // "MPS" => IDE::MPS,
      "IU" => IDE::IntelliJIDEA,
      "IC" => IDE::IntelliJIDEACommiunity,
      // "DS" => IDE::DataSpell,
      "DB" => IDE::DataGrip,
      // "QA" => IDE::Aqua,
      "GO" => IDE::GoLand,
      _ => return None,
    };

    Some(enum_value)
  }
}
