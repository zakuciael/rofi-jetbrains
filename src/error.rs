use glib::error;

use crate::G_LOG_DOMAIN;

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
