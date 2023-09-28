use std::borrow::Cow;

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
