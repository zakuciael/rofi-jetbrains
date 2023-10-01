use glib::error;

use crate::G_LOG_DOMAIN;

pub trait UnwrapOrError<T> {
  fn unwrap_or_error(self, msg: &str) -> T;
}

impl<T> UnwrapOrError<T> for Option<T> {
  fn unwrap_or_error(self, msg: &str) -> T {
    match self {
      Some(v) => v,
      None => {
        error!("{}", msg);
        panic!()
      }
    }
  }
}

impl<T, E> UnwrapOrError<T> for Result<T, E> {
  fn unwrap_or_error(self, msg: &str) -> T {
    match self {
      Ok(v) => v,
      Err(_) => {
        error!("{}", msg);
        panic!()
      }
    }
  }
}
