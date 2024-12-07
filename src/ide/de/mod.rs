use std::convert::Infallible;
use std::fmt::Formatter;
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;

use serde::de::{Error, Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

use crate::ide::IDEType;

struct IDEVisitor;

#[allow(clippy::needless_lifetimes)]
impl<'de> Visitor<'de> for IDEVisitor {
  type Value = IDEType;

  fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
    write!(formatter, "a string containing a valid product code")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: Error,
  {
    let ide = IDEType::from_product_code(v);

    if let Some(ide) = ide {
      Ok(ide)
    } else {
      Err(Error::invalid_value(Unexpected::Str(v), &self))
    }
  }
}

impl<'de> Deserialize<'de> for IDEType {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(IDEVisitor)
  }
}

struct EvaluatePropertyVisitor<T> {
  marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for EvaluatePropertyVisitor<T>
where
  T: Deserialize<'de> + AsRef<Path> + FromStr<Err = Infallible>,
{
  type Value = Option<T>;

  fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
    write!(formatter, "a string with valid property expressions")
  }

  #[inline]
  fn visit_none<E>(self) -> Result<Self::Value, E>
  where
    E: Error,
  {
    Ok(None)
  }

  #[inline]
  fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
  where
    D: Deserializer<'de>,
  {
    let value = T::deserialize(deserializer)?
      .as_ref()
      .to_string_lossy()
      .to_string();
    let home_dir =
      dirs::home_dir().ok_or(Error::custom("Failed to retrieve user home directory"))?;

    Ok(Some(
      T::from_str(&value.replace(
        "${user.home}",
        home_dir.to_string_lossy().to_string().as_ref(),
      ))
      .unwrap(),
    ))
  }

  #[inline]
  fn visit_unit<E>(self) -> Result<Self::Value, E>
  where
    E: Error,
  {
    Ok(None)
  }
}

pub fn evaluate_property<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
  T: Deserialize<'de> + AsRef<Path> + FromStr<Err = Infallible>,
  D: Deserializer<'de>,
{
  deserializer.deserialize_option(EvaluatePropertyVisitor {
    marker: PhantomData,
  })
}
