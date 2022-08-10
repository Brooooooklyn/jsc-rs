#include "binding.hpp"

inline WTFString toWTFString(WTF::String s)
{
  auto is_utf8 = s.is8Bit();
  auto characters8 = is_utf8 ? s.characters8() : nullptr;
  auto characters16 = is_utf8 ? nullptr : s.characters16();
  auto length = s.length();
  auto inner = reinterpret_cast<WTFStringImpl *>(s.releaseImpl().leakRef());
  WTFString result{
      inner,
      is_utf8,
      characters8,
      characters16,
      length,
  };
  return result;
}

extern "C"
{
  bool jsc_value_is_int(JSValueRef value)
  {
    auto jsValue = toJS(value);
    return jsValue.isAnyInt();
  }

  int32_t jsc_value_as_int(JSValueRef value)
  {
    auto jsValue = toJS(value);
    return jsValue.asInt32();
  }

  WTFString jsc_symbol_desc_string(JSValueRef value)
  {
    auto jsValue = toJS(value);
    auto desc = asSymbol(jsValue)->descriptiveString();
    return toWTFString(desc);
  }

  WTFString jsc_string_to_wtf_string(JSStringRef s)
  {
    auto ss = reinterpret_cast<OpaqueJSString *>(s);
    auto wtf_string = ss->string();
    return toWTFString(wtf_string);
  }

  JSStringRef jsc_string_from_static_rust_str(const char *str)
  {
    auto wtf_string = WTF::String::fromUTF8(str);
    return OpaqueJSString::tryCreate(wtf_string).leakRef();
  }

  void jsc_wtf_string_release(WTFStringImpl *inner)
  {
    WTF::StringImpl::destroy(reinterpret_cast<WTF::StringImpl *>(inner));
  }
}
