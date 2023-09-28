use std::ffi::c_char;
use std::ptr;

use glib::debug;

use crate::rofi::ffi::config_parser_add_option;
pub use crate::rofi::ffi::XrmOptionType;
use crate::rofi::xrmoptions::traits::XrmOption;
use crate::traits::ToNullTerminatedString;
use crate::G_LOG_DOMAIN;

pub mod impls;
pub mod traits;

pub fn config_parse_option<T: XrmOption>(key: &str, comment: &str) -> Option<T> {
  let key = key.to_nul_terminated();
  let comment = comment.to_nul_terminated();
  let mut val_ptr = ptr::null_mut();

  unsafe {
    debug!(
      "Reading config value \"{}\" as {:?}",
      key[0..key.len() - 1].to_owned(),
      T::xrm_type()
    );
    config_parser_add_option(
      T::xrm_type(),
      key.as_ptr() as *mut c_char,
      &mut val_ptr,
      comment.as_ptr() as *mut c_char,
    );

    if !val_ptr.is_null() {
      debug!("Config value found at {:?} address", &val_ptr);
      return Some(T::convert_ptr(val_ptr));
    }
  }

  debug!("Config value not set");
  None
}
