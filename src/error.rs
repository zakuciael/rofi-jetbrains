use glib::error;

use crate::G_LOG_DOMAIN;

pub trait UnwrapOrError<T> {
  fn unwrap_or_error(self, msg: &str) -> Result<T, ()>;
}

impl<T> UnwrapOrError<T> for Option<T> {
  fn unwrap_or_error(self, msg: &str) -> Result<T, ()> {
    match self {
      Some(v) => Ok(v),
      None => {
        error!("{}", msg);
        Err(())
      }
    }
  }
}

impl<T, E> UnwrapOrError<T> for Result<T, E> {
  fn unwrap_or_error(self, msg: &str) -> Result<T, ()> {
    match self {
      Ok(v) => Ok(v),
      Err(_) => {
        error!("{}", msg);
        Err(())
      }
    }
  }
}
