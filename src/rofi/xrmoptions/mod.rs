use std::ptr;

use libc::c_char;

use crate::rofi::ffi::config_parser_add_option;
pub use crate::rofi::ffi::XrmOptionType;
use crate::rofi::xrmoptions::traits::XrmOption;
use crate::traits::ToNullTerminatedString;

pub mod impls;
pub mod traits;

pub fn config_parse_option<T: XrmOption>(key: &str, comment: &str) -> Option<T> {
  let key = key.to_nul_terminated();
  let comment = comment.to_nul_terminated();
  let mut val_ptr = ptr::null_mut();

  unsafe {
    config_parser_add_option(
      T::xrm_type(),
      key.as_ptr() as *mut c_char,
      &mut val_ptr,
      comment.as_ptr() as *mut c_char,
    );

    if !val_ptr.is_null() {
      return Some(T::convert_ptr(val_ptr));
    }
  }

  None
}
