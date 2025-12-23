use glib::error;
use std::borrow::Cow;
use std::fmt::Display;
use std::process::exit;

use crate::G_LOG_DOMAIN;

pub trait ToNullTerminatedString<'a> {
  fn to_nul_terminated(&'a self) -> Cow<'a, str>;
}

impl<'a, T: AsRef<str>> ToNullTerminatedString<'a> for T {
  fn to_nul_terminated(&'a self) -> Cow<'a, str> {
    let val = self.as_ref();

    if val.ends_with('\0') {
      Cow::Borrowed(val)
    } else {
      Cow::Owned(val.to_string() + "\0")
    }
  }
}

pub trait MapToErrorLog<T> {
  fn map_to_error_log<M: AsRef<str>>(self, msg: M) -> Result<T, ()>;
}

impl<T> MapToErrorLog<T> for Option<T> {
  fn map_to_error_log<M: AsRef<str>>(self, msg: M) -> Result<T, ()> {
    match self {
      Some(v) => Ok(v),
      None => {
        error!("{}", msg.as_ref());
        Err(())
      }
    }
  }
}

impl<T, E> MapToErrorLog<T> for Result<T, E> {
  fn map_to_error_log<M: AsRef<str>>(self, msg: M) -> Result<T, ()> {
    match self {
      Ok(v) => Ok(v),
      Err(_) => {
        error!("{}", msg.as_ref());
        Err(())
      }
    }
  }
}

pub trait MapToErrorLogAndExit<T> {
  fn map_to_error_log_and_exit<M: AsRef<str>>(self, msg: M) -> T;
}

impl<T> MapToErrorLogAndExit<T> for Option<T> {
  fn map_to_error_log_and_exit<M: AsRef<str>>(self, msg: M) -> T {
    match self {
      Some(v) => v,
      None => {
        error!("{}", msg.as_ref());
        exit(1)
      }
    }
  }
}

impl<T, E> MapToErrorLogAndExit<T> for Result<T, E> {
  fn map_to_error_log_and_exit<M: AsRef<str>>(self, msg: M) -> T {
    match self {
      Ok(v) => v,
      Err(_) => {
        error!("{}", msg.as_ref());
        exit(1)
      }
    }
  }
}

pub trait ToErrorLogAndExit<T, E> {
  fn to_error_log_and_exit(self) -> T;
}

impl<T, E> ToErrorLogAndExit<T, E> for Result<T, E>
where
  E: Display,
{
  fn to_error_log_and_exit(self) -> T {
    self.unwrap_or_else(|err| {
      error!("{}", err);
      exit(1)
    })
  }
}
