#![allow(non_upper_case_globals)]

use std::ffi::CString;
use std::ptr;

use bitflags::bitflags;
use jsc_sys::{
  JSGlobalContextRef, JSObjectRef, JSObjectSetProperty, JSStringCreateWithUTF8CString, JSValueRef,
};

use crate::JsValue;

bitflags! {
  pub struct PropertyAttributes: u32 {
    const None = 0;
    const ReadOnly = 2;
    const DontEnum = 4;
    const DontDelete = 8;
  }
}

pub struct Object {
  pub(crate) ctx: JSGlobalContextRef,
  pub(crate) inner: JSObjectRef,
}

impl JsValue for Object {
  fn ctx(&self) -> JSGlobalContextRef {
    self.ctx
  }

  fn raw(&self) -> JSValueRef {
    self.inner
  }
}

impl Object {
  pub fn set_property<N: Into<Vec<u8>>, T: JsValue>(
    &mut self,
    property: N,
    value: &T,
    attr: PropertyAttributes,
  ) -> Result<(), JSValueRef> {
    let property = CString::new(property).unwrap();
    let mut exception = ptr::null();
    unsafe {
      let property_js = JSStringCreateWithUTF8CString(property.as_ptr());
      JSObjectSetProperty(
        self.ctx,
        self.inner,
        property_js,
        value.raw(),
        attr.bits,
        &mut exception,
      );
    };
    if exception.is_null() {
      Ok(())
    } else {
      Err(exception)
    }
  }
}
