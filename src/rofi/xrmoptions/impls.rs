use std::ffi::{c_char, c_void, CStr};

use crate::rofi::ffi::XrmOptionType;
use crate::rofi::xrmoptions::traits::XrmOption;

impl XrmOption for String {
  fn xrm_type() -> XrmOptionType {
    XrmOptionType::String
  }

  fn convert_ptr(ptr: *mut c_void) -> Self {
    unsafe {
      CStr::from_ptr(ptr as *mut c_char)
        .to_string_lossy()
        .to_string()
    }
  }
}
