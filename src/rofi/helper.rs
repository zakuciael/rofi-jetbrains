use std::borrow::Cow;
use std::ffi::{c_char, CStr};
use std::ptr;

pub fn find_arg_str(key: &str) -> Option<String> {
  let key = if key.ends_with("\0") {
    Cow::Borrowed(key)
  } else {
    Cow::Owned(key.to_owned() + "\0")
  };
  let key_ptr = key.as_ptr() as *const c_char;
  let mut val_ptr: *mut c_char = ptr::null_mut();

  unsafe {
    if rofi_mode::ffi::helper::find_arg_str(key_ptr, &mut val_ptr) == 0 {
      return None;
    }

    if !val_ptr.is_null() {
      let str = CStr::from_ptr(val_ptr);
      return Some(String::from_utf8_lossy(str.to_bytes()).to_string());
    }

    None
  }
}
