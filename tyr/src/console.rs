use std::{collections::HashSet, ptr, slice};

use jsc_safe::{
  sys::*, ClassAttribute, ClassDefinition, Context, JscError, Object, PropertyAttributes,
};

use crate::wellknown_property_name;

pub fn create(ctx: &Context) -> Result<Object, JscError> {
  let mut console = ClassDefinition::default()
    .with_c_name(crate::c_str("Console\0"))
    .with_attribute(ClassAttribute::NoAutomaticPrototype)
    .into_class()
    .make_object(&ctx);
  let log_fn = ctx.create_function("log", Some(log))?;
  let info_fn = ctx.create_function("info", Some(log))?;
  let warn_fn = ctx.create_function("warn", Some(log))?;
  let error_fn = ctx.create_function("error", Some(log))?;
  console.set_property("log", &log_fn, PropertyAttributes::None)?;
  console.set_property("info", &info_fn, PropertyAttributes::None)?;
  console.set_property("warn", &warn_fn, PropertyAttributes::None)?;
  console.set_property("error", &error_fn, PropertyAttributes::None)?;
  Ok(console)
}

pub unsafe extern "C" fn log(
  ctx: JSContextRef,
  _function: JSObjectRef,
  _this: JSObjectRef,
  argument_count: usize,
  arguments: *const JSValueRef,
  exception: *mut JSValueRef,
) -> JSValueRef {
  let args = slice::from_raw_parts(arguments, argument_count);
  let mut reference_set = HashSet::default();
  args.iter().for_each(|arg| {
    let serde_value = js_value_to_console(ctx, *arg, &mut reference_set, exception);
    match &serde_value {
      serde_json::Value::String(s) => println!("{s}"),
      serde_json::Value::Null => println!("null"),
      serde_json::Value::Bool(b) => println!("{b}"),
      serde_json::Value::Number(n) => println!("{n}"),
      serde_json::Value::Array(_) | serde_json::Value::Object(_) => {
        println!("{}", serde_json::to_string_pretty(&serde_value).unwrap())
      }
    }
  });
  JSValueMakeUndefined(ctx)
}

pub unsafe fn js_value_to_console(
  ctx: JSContextRef,
  value: JSValueRef,
  reference_set: &mut HashSet<JSValueRef>,
  exception: *mut JSValueRef,
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
      if JSValueIsArray(ctx, value) {
        let length = JSObjectGetProperty(
          ctx,
          value as *mut _,
          wellknown_property_name::LENGTH.with(|s| *s),
          exception,
        );
        let len = JSValueToNumber(ctx, length, exception) as usize;
        let mut array = Vec::with_capacity(len);
        for index in 0..len {
          let element = JSObjectGetPropertyAtIndex(ctx, value as *mut _, index as u32, exception);
          let element_value = js_value_to_console(ctx, element, reference_set, exception);
          reference_set.insert(element);
          reference_set.insert(value);
          array.push(element_value);
        }
        return serde_json::Value::Array(array);
      }
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
          value_to_display[name] = js_value_to_console(ctx, value, reference_set, exception);
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
