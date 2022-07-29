use std::ffi::CString;
use std::ptr;

use jsc_sys::{
  JSContextGetGlobalObject, JSContextGroupCreate, JSContextGroupRef, JSContextGroupRelease,
  JSEvaluateScript, JSGarbageCollect, JSGlobalContextCreateInGroup, JSGlobalContextRef,
  JSGlobalContextRelease, JSObjectCallAsFunctionCallback, JSObjectMakeFunctionWithCallback,
  JSStringCreateWithUTF8CString, JSStringRef, JSValueRef,
};

mod class;
mod error;
mod object;
mod value;

pub use jsc_sys as sys;

pub use class::*;
pub use error::*;
pub use object::*;
pub use value::*;

pub struct Context {
  group: JSContextGroupRef,
  pub(crate) inner: JSGlobalContextRef,
}

impl Context {
  pub fn new() -> Self {
    Self::create(None)
  }

  pub fn with_global_class(class: Class) -> Self {
    Self::create(Some(class))
  }

  fn create(global_class: Option<Class>) -> Self {
    let group = unsafe { JSContextGroupCreate() };
    let inner = unsafe {
      JSGlobalContextCreateInGroup(
        group,
        global_class.map(|c| c.raw()).unwrap_or(ptr::null_mut()),
      )
    };
    Context { group, inner }
  }

  pub fn global(&self) -> Object {
    Object {
      ctx: self.inner,
      inner: unsafe { JSContextGetGlobalObject(self.inner) },
    }
  }

  pub fn create_function<N: Into<Vec<u8>>>(
    &self,
    name: N,
    callback: JSObjectCallAsFunctionCallback,
  ) -> Result<Object, JscError> {
    let js_name = self.create_string(name);
    Ok(Object {
      inner: unsafe { JSObjectMakeFunctionWithCallback(self.inner, js_name, callback) },
      ctx: self.inner,
    })
  }

  pub fn gc(&mut self) {
    unsafe { JSGarbageCollect(self.inner) };
  }

  pub fn eval(&self, script: String) -> Result<JSValueRef, JSValueRef> {
    let script = self.create_string(script);
    let mut exception = ptr::null();
    let output = unsafe {
      JSEvaluateScript(
        self.inner,
        script,
        JSContextGetGlobalObject(self.inner),
        ptr::null_mut(),
        0,
        &mut exception,
      )
    };
    if exception.is_null() {
      Ok(output)
    } else {
      Ok(exception)
    }
  }

  pub fn raw(&self) -> JSGlobalContextRef {
    self.inner
  }

  #[inline]
  fn create_string<T: Into<Vec<u8>>>(&self, string: T) -> JSStringRef {
    let c_string = CString::new(string).expect("Source string contains invalid UTF-8");
    unsafe { JSStringCreateWithUTF8CString(c_string.as_ptr()) }
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
