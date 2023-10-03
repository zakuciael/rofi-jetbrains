macro_rules! ensure {
  ($code:expr, $($arg:tt)+) => {
    match $code {
      Some(v) => v,
      None => {
        return Some(Err(format!($($arg)+)))
      }
    }
  };
}

pub(crate) use ensure;
