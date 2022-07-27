use std::ffi::CString;
use std::ptr;

use jsc_sys::{JSClassCreate, JSClassDefinition, JSClassRef, JSClassRelease, JSObjectMake};

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

  pub fn with_attribute(mut self, attribute: ClassAttribute) -> Self {
    self.inner.attributes = attribute as u32;
    Self { inner: self.inner }
  }

  pub fn into_class(self) -> Result<Class, JscError> {
    let class_ref = unsafe { JSClassCreate(&self.inner as *const JSClassDefinition) };
    if class_ref.is_null() {
      return Err(JscError::CreateClassError);
    }
    Ok(Class { inner: class_ref })
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

impl Drop for Class {
  fn drop(&mut self) {
    unsafe { JSClassRelease(self.inner) };
  }
}
