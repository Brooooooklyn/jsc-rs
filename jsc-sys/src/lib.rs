#[cfg(not(target_os = "windows"))]
extern crate compiler_builtins;

use std::fmt::Display;
use std::slice;

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
  is_utf8: bool,
  characters8: *const u8,
  characters16: *const u16,
  length: u32,
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
  pub fn jsc_symbol_desc_string(symbol: JSValueRef) -> WTString;
  fn jsc_wtf_string_release(inner: *mut WTFStringImpl);
}
