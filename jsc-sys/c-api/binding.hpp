#include <JavaScriptCore/runtime/JSExportMacros.h>
#include <PrivateHeaders/JavaScriptCore/ArrayBuffer.h>

typedef struct jsc_arraybuffer jsc_arraybuffer;

extern "C"
{
  void jsc_create_arraybuffer(size_t numElements, unsigned elementByteSize, jsc_arraybuffer **result);
  void jsc_unref_arraybuffer(jsc_arraybuffer *arraybuffer);
}
