use std::slice;

use jsc::sys::{
  JSContextRef, JSObjectRef, JSStringGetCharactersPtr, JSStringGetLength, JSType_kJSTypeString,
  JSValueGetType, JSValueMakeUndefined, JSValueRef,
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
      let string_length = JSStringGetLength(*arg as *mut _);
      let utf16_ptr = JSStringGetCharactersPtr(*arg as *mut _);
      if let Ok(s) = String::from_utf16(slice::from_raw_parts(utf16_ptr, string_length)) {
        println!("{}", s);
      }
    }
  });
  JSValueMakeUndefined(ctx)
}
