#if defined(_MSC_VER)
#define NOMINMAX // Prevent windows.h from defining min and max macros.
#include <Windows.h>
#endif

#include <PrivateHeaders/JavaScriptCore/JSExportMacros.h>
#include <PrivateHeaders/JavaScriptCore/ArrayBuffer.h>

typedef struct jsc_arraybuffer jsc_arraybuffer;

extern "C"
{
  void jsc_create_arraybuffer(size_t numElements, unsigned elementByteSize, jsc_arraybuffer **result);
  void jsc_unref_arraybuffer(jsc_arraybuffer *arraybuffer);
}
