use std::ffi::{c_char, c_void};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub enum XrmOptionType {
  String = 0,
  Integer = 1,
  SignedInteger = 2,
  Boolean = 3,
  Char = 4,
}

extern "C" {
  pub fn config_parser_add_option(
    xrm_type: XrmOptionType,
    key: *mut c_char,
    value: *mut *mut c_void,
    comment: *mut c_char,
  );
}
