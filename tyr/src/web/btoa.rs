use std::slice;

use encoding_rs::mem::convert_utf16_to_str;
use jsc_safe::sys::*;

pub unsafe extern "C" fn btoa(
  ctx: JSContextRef,
  _function: JSObjectRef,
  _this: JSObjectRef,
  argument_count: usize,
  arguments: *const JSValueRef,
  exception: *mut JSValueRef,
) -> JSValueRef {
  if argument_count == 0 {
    return JSValueMakeUndefined(ctx);
  }
  let args = slice::from_raw_parts(arguments, argument_count);
  let string_to_encode = args[0];
  if !JSValueIsString(ctx, string_to_encode) {
    return JSValueMakeUndefined(ctx);
  }
  let string_to_encode = JSValueToStringCopy(ctx, string_to_encode, exception);
  let wtf_string = jsc_string_to_wtf_string(string_to_encode);
  let len = wtf_string.length as usize;
  let (str_to_encode, container_to_drop) = if wtf_string.is_utf8 {
    (
      slice::from_raw_parts(wtf_string.characters8, len),
      String::new(),
    )
  } else {
    let mut output = String::from_utf8_unchecked(vec![0; len * 3]);
    let new_length = convert_utf16_to_str(
      slice::from_raw_parts(wtf_string.characters16, len),
      &mut output,
    );
    (slice::from_raw_parts(output.as_ptr(), new_length), output)
  };
  let b64 = base64_simd::Base64::URL_SAFE;
  let mut output = vec![0; len * 3];
  let output_buf = base64_simd::OutBuf::new(output.as_mut_slice());
  let output_ptr = {
    if let Ok(o) = b64.encode(str_to_encode, output_buf) {
      let l = o.len();
      let o = slice::from_raw_parts_mut(o.as_mut_ptr(), l + 1);
      o[l] = b'\0';
      output.as_ptr()
    } else {
      std::ptr::null_mut()
    }
  };
  if output_ptr.is_null() {
    return JSValueMakeUndefined(ctx);
  }
  drop(container_to_drop);
  JSValueMakeString(ctx, JSStringCreateWithUTF8CString(output_ptr as *const _))
}
