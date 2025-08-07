use std::fmt::{Display, Formatter};

use strum::EnumIter;

pub mod data;
mod de;
pub mod product_info;
pub mod properties;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum IDEType {
  Aqua,
  CLion,
  IntelliJIDEA,
  PHPStorm,
  PyCharm,
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

impl IDEType {
  pub fn from_product_code<T: AsRef<str>>(code: T) -> Option<Self> {
    let enum_value = match code.as_ref() {
      "RD" => IDEType::Rider,
      "AI" => IDEType::AndroidStudio,
      "CL" => IDEType::CLion,
      "RR" => IDEType::RustRover,
      "WS" => IDEType::WebStorm,
      "RM" => IDEType::RubyMine,
      "PY" | "PC" => IDEType::PyCharm,
      "PS" => IDEType::PHPStorm,
      "MPS" => IDEType::MPS,
      "IU" | "IC" => IDEType::IntelliJIDEA,
      "DS" => IDEType::DataSpell,
      "DB" => IDEType::DataGrip,
      "QA" => IDEType::Aqua,
      "GO" => IDEType::GoLand,
      _ => return None,
    };

    Some(enum_value)
  }

  pub fn get_default_alias(&self) -> (String, IDEType) {
    let alias = match self {
      IDEType::Aqua => "aqua",
      IDEType::CLion => "clion",
      IDEType::IntelliJIDEA => "idea",
      IDEType::PHPStorm => "phpstorm",
      IDEType::PyCharm => "pycharm",
      IDEType::Rider => "rider",
      IDEType::WebStorm => "webstorm",
      IDEType::GoLand => "goland",
      IDEType::DataGrip => "datagrip",
      IDEType::DataSpell => "dataspell",
      IDEType::RubyMine => "rubymine",
      IDEType::AndroidStudio => "android-studio",
      IDEType::RustRover => "rustrover",
      IDEType::MPS => "mps",
    };

    (alias.to_owned(), self.clone())
  }
}

impl Display for IDEType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let name = match self {
      IDEType::Aqua => "Aqua",
      IDEType::CLion => "CLion",
      IDEType::IntelliJIDEA => "IntelliJ IDEA",
      IDEType::PHPStorm => "PHPStorm",
      IDEType::PyCharm => "PyCharm",
      IDEType::Rider => "Rider",
      IDEType::WebStorm => "WebStorm",
      IDEType::GoLand => "GoLand",
      IDEType::DataGrip => "DataGrip",
      IDEType::DataSpell => "DataSpell",
      IDEType::RubyMine => "RubyMine",
      IDEType::AndroidStudio => "Android Studio",
      IDEType::RustRover => "RustRover",
      IDEType::MPS => "MPS",
    };

    write!(f, "{name}")
  }
}
