use std::ptr;

use jsc_sys::{
  JSGlobalContextRef, JSValueIsArray, JSValueIsDate, JSValueIsEqual, JSValueIsUndefined, JSValueRef,
};

pub trait JsValue {
  fn ctx(&self) -> JSGlobalContextRef;
  fn raw(&self) -> JSValueRef;

  fn is_undefined(&self) -> bool {
    unsafe { JSValueIsUndefined(self.ctx(), self.raw()) }
  }

  fn is_array(&self) -> bool {
    unsafe { JSValueIsArray(self.ctx(), self.raw()) }
  }

  fn is_date(&self) -> bool {
    unsafe { JSValueIsDate(self.ctx(), self.raw()) }
  }

  fn is_equal<T: JsValue>(&self, other: &T) -> bool {
    let mut exception = ptr::null();
    unsafe { JSValueIsEqual(self.ctx(), self.raw(), other.raw(), &mut exception) }
  }
}
