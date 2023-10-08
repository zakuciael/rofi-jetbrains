#![allow(clippy::redundant_closure_call)]
use std::ffi::{c_char, c_void, CStr};
use std::path::PathBuf;

use resolve_path::PathResolveExt;

use crate::rofi::ffi::XrmOptionType;
use crate::rofi::xrmoptions::traits::XrmOptionConverter;

macro_rules! impl_option_deserializer {
  ($rust_type:ty : $xrm_type:expr => $converter:expr) => {
    impl XrmOptionConverter for $rust_type {
      fn xrm_type() -> XrmOptionType {
        $xrm_type
      }

      fn convert(value: *mut c_void) -> Self {
        $converter(value)
      }
    }
  };
  ($rust_type:ty : $xrm_type:expr) => {
    impl XrmOptionConverter for $rust_type {
      fn xrm_type() -> XrmOptionType {
        $xrm_type
      }

      fn convert(value: *mut c_void) -> Self {
        value as $rust_type
      }
    }
  };
}

macro_rules! impl_option_vec_deserializer {
    ($($rust_type:ty),*) => (
      $(
        impl_option_deserializer!(Option<Vec<$rust_type>>: XrmOptionType::String => |value: *mut c_void| {
          Option::<Vec<String>>::convert(value).map(|vec| {
            vec.into_iter().flat_map(|v| v.parse::<$rust_type>()).collect::<Vec<$rust_type>>()
          })
        });
      )*
    );
}

impl_option_deserializer!(i32: XrmOptionType::SignedInteger);
impl_option_deserializer!(u32: XrmOptionType::UnsignedInteger);
impl_option_deserializer!(char: XrmOptionType::Char => |value| value as u8 as char);
impl_option_deserializer!(bool: XrmOptionType::Boolean => |value| (value as u8) > 0);
impl_option_deserializer!(Option<String>: XrmOptionType::String => |value: *mut c_void| {
  if !value.is_null() {
    unsafe {
      return Some(CStr::from_ptr(value as *mut c_char)
          .to_string_lossy()
          .to_string())
    }
  }

  None
});
impl_option_deserializer!(Option<PathBuf>: XrmOptionType::String => |value: *mut c_void| {
    Option::<String>::convert(value).map(|raw| raw.resolve().to_path_buf())
});
impl_option_deserializer!(Option<Vec<String>>: XrmOptionType::String => |value: *mut c_void| {
  Option::<String>::convert(value)
    .map(|raw| raw.split(',')
      .map(|r|r.to_owned())
      .collect::<Vec<String>>()
    )
});
impl_option_vec_deserializer!(i32, u32, PathBuf);
