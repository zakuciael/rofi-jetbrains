macro_rules! ensure_option {
  ($code:expr, $($arg:tt)+) => {
    match $code {
      Some(v) => v,
      None => {
        return Some(Err(format!($($arg)+)))
      }
    }
  };
}

macro_rules! ensure_result {
  ($code:expr, $($arg:tt)+) => {
    match $code {
      Some(v) => v,
      None => {
        return Err(format!($($arg)+))
      }
    }
  };
}

macro_rules! wrap_icon_request {
  ($code:expr) => {
    match $code {
      Ok(v) => Some(v),
      Err(err) => {
        use std::error::Error;
        glib::warn!("{}:\n\t{}", err, err.source().unwrap());
        None
      }
    }
  };
}

pub(crate) use ensure_option;
pub(crate) use ensure_result;
pub(crate) use wrap_icon_request;
