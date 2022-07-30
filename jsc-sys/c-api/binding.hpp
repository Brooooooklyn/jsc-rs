#if defined(_MSC_VER)
#define NOMINMAX // Prevent windows.h from defining min and max macros.
#include <Windows.h>
#endif

#include <cstdint>
#include <PrivateHeaders/JavaScriptCore/JSExportMacros.h>

#include <PrivateHeaders/JavaScriptCore/APICast.h>
#include <PrivateHeaders/JavaScriptCore/JSArray.h>
#include <PrivateHeaders/JavaScriptCore/JSCJSValue.h>
#include <PrivateHeaders/JavaScriptCore/JSGlobalObject.h>
#include <PrivateHeaders/JavaScriptCore/OpaqueJSString.h>
#include <PrivateHeaders/JavaScriptCore/ScriptArguments.h>
#include <PrivateHeaders/JavaScriptCore/Symbol.h>
#include <wtf/text/StringImpl.h>
#include <wtf/Vector.h>

typedef struct WTFStringImpl WTFStringImpl;

struct WTFString
{
  WTFStringImpl *inner;
  bool is_utf8;
  const unsigned char *characters8;
  const char16_t *characters16;
  uint32_t length;
};

extern "C"
{
  bool jsc_value_is_int(JSValueRef value);
  WTFString jsc_string_to_wtf_string(JSStringRef s);
  WTFString jsc_symbol_desc_string(JSValueRef value);
  JSStringRef jsc_string_from_static_rust_str(const char *str);
  void jsc_wtf_string_release(WTFStringImpl *inner);
}
