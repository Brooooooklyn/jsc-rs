use std::collections::HashSet;
use std::ptr;
use std::slice;

use jsc_safe::sys::*;

pub unsafe extern "C" fn console_log(
  ctx: JSContextRef,
  _function: JSObjectRef,
  _this: JSObjectRef,
  argument_count: usize,
  arguments: *const JSValueRef,
  _exception: *mut JSValueRef,
) -> JSValueRef {
  let args = slice::from_raw_parts(arguments, argument_count);
  let mut reference_set = HashSet::default();
  args.iter().for_each(|arg| {
    println!(
      "{}",
      serde_json::to_string_pretty(&js_value_to_console(ctx, *arg, &mut reference_set)).unwrap()
    );
  });
  JSValueMakeUndefined(ctx)
}

unsafe fn js_value_to_console(
  ctx: JSContextRef,
  value: JSValueRef,
  reference_set: &mut HashSet<JSValueRef>,
) -> serde_json::Value {
  let js_type = JSValueGetType(ctx, value);
  match js_type {
    0 => serde_json::Value::String("undefined".to_owned()),
    1 => serde_json::Value::Null,
    2 => serde_json::Value::Bool(JSValueToBoolean(ctx, value)),
    3 => serde_json::Value::Number(if jsc_value_is_int(value) {
      serde_json::Number::from(JSValueToNumber(ctx, value, ptr::null_mut()) as i64)
    } else {
      serde_json::Number::from_f64(JSValueToNumber(ctx, value, ptr::null_mut())).unwrap()
    }),
    4 => {
      let js_string = JSValueToStringCopy(ctx, value, ptr::null_mut());
      let string_length = JSStringGetLength(js_string);
      let utf16_ptr = JSStringGetCharactersPtr(js_string);
      serde_json::Value::String(String::from_utf16_lossy(slice::from_raw_parts(
        utf16_ptr,
        string_length,
      )))
    }
    5 => {
      reference_set.insert(value);
      let names_array = JSObjectCopyPropertyNames(ctx, value as *mut _);
      let names_array_length = JSPropertyNameArrayGetCount(names_array);
      let mut value_to_display = serde_json::Value::Object(serde_json::Map::default());
      for index in 0..names_array_length {
        let name_string = JSPropertyNameArrayGetNameAtIndex(names_array, index);
        let name_wtf = jsc_string_to_wtf_string(name_string);
        let name = format!("{name_wtf}");
        let value = JSObjectGetProperty(ctx, value as *mut _, name_string, ptr::null_mut());
        if reference_set.contains(&value) {
          value_to_display[name] = serde_json::Value::String("<circular reference>".to_owned());
        } else {
          reference_set.insert(value);
          value_to_display[name] = js_value_to_console(ctx, value, reference_set);
        }
      }
      value_to_display
    }
    6 => {
      let desc = jsc_symbol_desc_string(value);
      serde_json::Value::String(format!("{desc}"))
    }
    _ => unreachable!(),
  }
}
