use crate::rofi::xrmoptions::XrmOptionType;
use std::ffi::c_void;

pub trait XrmOptionConverter {
  fn xrm_type() -> XrmOptionType;

  fn convert(value: *mut c_void) -> Self;
}
