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

pub(crate) use ensure_option;
pub(crate) use ensure_result;
