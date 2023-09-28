use std::ffi::{c_char, c_void, CStr};

use glib::debug;

use crate::rofi::ffi::XrmOptionType;
use crate::rofi::xrmoptions::traits::XrmOption;
use crate::G_LOG_DOMAIN;

impl XrmOption for String {
  fn xrm_type() -> XrmOptionType {
    XrmOptionType::String
  }

  fn convert_ptr(ptr: *mut c_void) -> Self {
    unsafe {
      debug!("Converting pointer to C string..");
      CStr::from_ptr(ptr as *mut c_char)
        .to_string_lossy()
        .to_string()
    }
  }
}
