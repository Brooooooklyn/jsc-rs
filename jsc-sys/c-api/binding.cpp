#include "binding.hpp"

extern "C"
{
  void jsc_create_arraybuffer(size_t numElements, unsigned elementByteSize, jsc_arraybuffer **result)
  {
    auto ab = JSC::ArrayBuffer::create(numElements, elementByteSize);
    *result = reinterpret_cast<jsc_arraybuffer *>(&ab.leakRef());
  }

  void jsc_unref_arraybuffer(jsc_arraybuffer *arraybuffer)
  {
    auto ab = reinterpret_cast<JSC::ArrayBuffer *>(arraybuffer);
    ab->deref();
  }
}