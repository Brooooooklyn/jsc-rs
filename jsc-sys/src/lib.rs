#[cfg(not(target_os = "windows"))]
extern crate compiler_builtins;

use std::{fmt::Display, os::raw::c_char, slice};

mod binding;

pub use binding::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WTFStringImpl {
  _unused: [u8; 0],
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WTString {
  inner: *mut WTFStringImpl,
  pub is_utf8: bool,
  pub characters8: *const u8,
  pub characters16: *const u16,
  pub length: u32,
}

impl Display for WTString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.is_utf8 {
      write!(
        f,
        "{}",
        String::from_utf8_lossy(unsafe {
          slice::from_raw_parts(self.characters8, self.length as usize)
        })
      )
    } else {
      write!(
        f,
        "{}",
        String::from_utf16_lossy(unsafe {
          slice::from_raw_parts(self.characters16, self.length as usize)
        })
      )
    }
  }
}

impl Drop for WTString {
  fn drop(&mut self) {
    unsafe {
      jsc_wtf_string_release(self.inner);
    }
  }
}

extern "C" {
  pub fn jsc_value_is_int(value: JSValueRef) -> bool;
  pub fn jsc_string_to_wtf_string(string: JSStringRef) -> WTString;
  /// The created String will be leaked in runtime
  /// It's used for create Object Property attached to GlobalObject
  pub fn jsc_string_from_static_rust_str(string: *const c_char) -> JSStringRef;
  pub fn jsc_symbol_desc_string(symbol: JSValueRef) -> WTString;
  fn jsc_wtf_string_release(inner: *mut WTFStringImpl);
}
