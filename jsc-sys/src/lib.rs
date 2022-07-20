#[cfg(not(target_os = "windows"))]
extern crate compiler_builtins;

mod binding;

pub use binding::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct JscArrayBuffer {
  _unused: [u8; 0],
}

extern "C" {
  pub fn jsc_create_arraybuffer(
    elements: usize,
    bytes: u32,
    array_buffer: *mut *mut JscArrayBuffer,
  );
  pub fn jsc_unref_arraybuffer(array_buffer: *mut JscArrayBuffer);
}
