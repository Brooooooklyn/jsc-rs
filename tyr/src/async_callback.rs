use std::fmt::Display;

use jsc_safe::sys::*;

pub struct AsyncCallback {
  func: JSValueRef,
  this: JSValueRef,
  args: *const JSValueRef,
  len: usize,
  exception: *mut JSValueRef,
}

impl Display for AsyncCallback {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "AsyncCallback")
  }
}

unsafe impl Send for AsyncCallback {}

impl AsyncCallback {
  pub fn new(
    func: JSValueRef,
    this: JSValueRef,
    args: *const JSValueRef,
    len: usize,
    exception: *mut JSValueRef,
  ) -> Self {
    Self {
      func,
      this,
      args,
      len,
      exception,
    }
  }

  pub fn call(self, ctx: JSGlobalContextRef) -> JSValueRef {
    unsafe {
      JSObjectCallAsFunction(
        ctx,
        self.func as *mut _,
        self.this as *mut _,
        self.len,
        self.args,
        self.exception,
      )
    }
  }
}
