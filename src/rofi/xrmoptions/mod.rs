use std::ffi::c_char;
use std::ptr;

use glib::debug;

use crate::rofi::ffi::config_parser_add_option;
pub use crate::rofi::ffi::XrmOptionType;
use crate::rofi::xrmoptions::traits::XrmOptionConverter;
use crate::traits::ToNullTerminatedString;
use crate::G_LOG_DOMAIN;

pub mod impls;
pub mod traits;

pub fn config_parse_option<T: XrmOptionConverter>(key: &str, comment: &str) -> T {
  debug!("Reading config value {:?}", key.to_owned());

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
  }

  debug!("Converting raw pointer into value..");
  T::convert(val_ptr)
}
