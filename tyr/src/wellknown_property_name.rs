use jsc_safe::sys::{jsc_string_from_static_rust_str, JSStringRef};

thread_local! {
  pub static LENGTH: JSStringRef = unsafe { jsc_string_from_static_rust_str("length\0".as_ptr() as *const _) };
}
