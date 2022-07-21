use std::ffi::CString;
use std::ptr;

use jsc_sys::{
  JSContextGetGlobalObject, JSContextGroupCreate, JSContextGroupRef, JSContextGroupRelease,
  JSContextGroupRetain, JSEvaluateScript, JSGarbageCollect, JSGlobalContextCreateInGroup,
  JSGlobalContextRef, JSGlobalContextRelease, JSGlobalContextRetain,
  JSObjectCallAsFunctionCallback, JSObjectMakeFunctionWithCallback, JSStringCreateWithUTF8CString,
  JSStringRef, JSValueRef,
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
  pub fn new() -> Result<Self, JscError> {
    Self::create(None)
  }

  pub fn with_global_class(class: Class) -> Result<Self, JscError> {
    Self::create(Some(class))
  }

  fn create(global_class: Option<Class>) -> Result<Self, JscError> {
    let group = unsafe { JSContextGroupCreate() };
    if group.is_null() {
      return Err(JscError::CreateContextGroupError);
    }
    unsafe { JSContextGroupRetain(group) };
    let inner = unsafe {
      JSGlobalContextCreateInGroup(
        group,
        global_class.map(|c| c.raw()).unwrap_or(ptr::null_mut()),
      )
    };
    if inner.is_null() {
      return Err(JscError::CreateGlobalContextError);
    }
    unsafe { JSGlobalContextRetain(inner) };
    Ok(Context { group, inner })
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
    let js_name = self.create_string(name)?;
    Ok(Object {
      inner: unsafe { JSObjectMakeFunctionWithCallback(self.inner, js_name, callback) },
      ctx: self.inner,
    })
  }

  pub fn gc(&mut self) {
    unsafe { JSGarbageCollect(self.inner) };
  }

  pub fn eval(&self, script: String) -> Result<JSValueRef, JscError> {
    let script = self.create_string(script)?;
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
    Ok(output)
  }

  fn create_string<T: Into<Vec<u8>>>(&self, string: T) -> Result<JSStringRef, JscError> {
    let c_string = CString::new(string).map_err(JscError::from)?;
    Ok(unsafe { JSStringCreateWithUTF8CString(c_string.as_ptr()) })
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
