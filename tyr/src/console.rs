use std::ptr;
use std::slice;

use jsc::sys::{
  JSContextRef, JSObjectRef, JSStringGetCharactersPtr, JSStringGetLength, JSType_kJSTypeString,
  JSValueGetType, JSValueMakeUndefined, JSValueRef, JSValueToStringCopy,
};

pub unsafe extern "C" fn console_log(
  ctx: JSContextRef,
  _function: JSObjectRef,
  _this: JSObjectRef,
  argument_count: usize,
  arguments: *const JSValueRef,
  _exception: *mut JSValueRef,
) -> JSValueRef {
  let args = slice::from_raw_parts(arguments, argument_count);
  args.iter().for_each(|arg| {
    let js_type = JSValueGetType(ctx, *arg);
    if js_type == JSType_kJSTypeString {
      let js_string = JSValueToStringCopy(ctx, *arg, ptr::null_mut());
      let string_length = JSStringGetLength(js_string);
      let utf16_ptr = JSStringGetCharactersPtr(js_string);
      if let Ok(s) = String::from_utf16(slice::from_raw_parts(utf16_ptr, string_length)) {
        println!("{}", s);
      }
    }
  });
  JSValueMakeUndefined(ctx)
}
