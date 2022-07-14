use std::ptr;

use jsc_sys::{
  JSContextGroupCreate, JSContextGroupRef, JSContextGroupRelease, JSGarbageCollect,
  JSGlobalContextCreateInGroup, JSGlobalContextRef, JSGlobalContextRelease,
};

pub struct Context {
  group: JSContextGroupRef,
  inner: JSGlobalContextRef,
}

impl Context {
  #[must_use]
  pub fn new() -> Context {
    let group = unsafe { JSContextGroupCreate() };
    let inner = unsafe { JSGlobalContextCreateInGroup(group, ptr::null_mut()) };
    Context { group, inner }
  }

  pub fn gc(&mut self) {
    unsafe { JSGarbageCollect(self.inner) };
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe {
      JSGlobalContextRelease(self.inner);
      JSContextGroupRelease(self.group);
    };
  }
}
