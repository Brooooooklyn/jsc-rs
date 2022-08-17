use std::{ffi::CString, os::raw::c_char, ptr};

use jsc_sys::{JSClassCreate, JSClassDefinition, JSClassRef, JSObjectMake};

use crate::{Context, JscError, Object};

#[repr(u32)]
pub enum ClassAttribute {
  None = jsc_sys::kJSClassAttributeNone,
  NoAutomaticPrototype = jsc_sys::kJSClassAttributeNoAutomaticPrototype,
}

pub struct ClassDefinition {
  inner: JSClassDefinition,
}

impl Default for ClassDefinition {
  fn default() -> Self {
    Self {
      inner: JSClassDefinition {
        version: 0,
        attributes: ClassAttribute::None as u32,
        className: ptr::null(),
        parentClass: ptr::null_mut(),
        staticValues: ptr::null(),
        staticFunctions: ptr::null(),
        initialize: None,
        finalize: None,
        hasProperty: None,
        getProperty: None,
        setProperty: None,
        deleteProperty: None,
        getPropertyNames: None,
        callAsFunction: None,
        callAsConstructor: None,
        hasInstance: None,
        convertToType: None,
      },
    }
  }
}

impl ClassDefinition {
  pub fn with_name<S: Into<Vec<u8>>>(mut self, name: S) -> Result<Self, JscError> {
    let c_name = CString::new(name)?;
    self.inner.className = c_name.as_ptr();
    Ok(Self { inner: self.inner })
  }

  pub fn with_c_name(mut self, c_name: *const c_char) -> Self {
    self.inner.className = c_name;
    Self { inner: self.inner }
  }

  pub fn with_attribute(mut self, attribute: ClassAttribute) -> Self {
    self.inner.attributes = attribute as u32;
    Self { inner: self.inner }
  }

  pub fn into_class(self) -> Class {
    Class {
      inner: unsafe { JSClassCreate(&self.inner as *const JSClassDefinition) },
    }
  }
}

pub struct Class {
  inner: JSClassRef,
}

impl Class {
  pub fn make_object(self, ctx: &Context) -> Object {
    unsafe { JSObjectMake(ctx.inner, self.inner, ptr::null_mut()) };
    Object {
      ctx: ctx.inner,
      inner: unsafe { JSObjectMake(ctx.inner, self.inner, ptr::null_mut()) },
    }
  }

  pub fn raw(&self) -> JSClassRef {
    self.inner
  }
}
