use crate::rofi::xrmoptions::XrmOptionType;
use std::ffi::c_void;

pub trait XrmOption {
  fn xrm_type() -> XrmOptionType;

  fn convert_ptr(ptr: *mut c_void) -> Self;
}
