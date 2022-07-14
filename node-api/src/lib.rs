#![allow(non_camel_case_types)]

use jsc_sys::{JSContextRef, JSValueMakeUndefined, JSValueRef};

pub type napi_env = JSContextRef;
pub type napi_value = JSValueRef;

pub fn napi_get_undefined(env: napi_env) -> napi_value {
  unsafe { JSValueMakeUndefined(env) }
}
