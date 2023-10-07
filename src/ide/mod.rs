pub mod data;
mod de;
pub mod product_info;
pub mod properties;

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IDEType {
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

impl IDEType {
  pub fn from_product_code<T: AsRef<str>>(code: T) -> Option<Self> {
    let enum_value = match code.as_ref() {
      "RD" => IDEType::Rider,
      "AI" => IDEType::AndroidStudio,
      "CL" => IDEType::CLion,
      "RR" => IDEType::RustRover,
      "WS" => IDEType::WebStorm,
      "RM" => IDEType::RubyMine,
      "PC" => IDEType::PyCharmCommiunity,
      "PY" => IDEType::PyCharm,
      "PS" => IDEType::PHPStorm,
      "MPS" => IDEType::MPS,
      "IU" => IDEType::IntelliJIDEA,
      "IC" => IDEType::IntelliJIDEACommiunity,
      "DS" => IDEType::DataSpell,
      "DB" => IDEType::DataGrip,
      "QA" => IDEType::Aqua,
      "GO" => IDEType::GoLand,
      _ => return None,
    };

    Some(enum_value)
  }
}
