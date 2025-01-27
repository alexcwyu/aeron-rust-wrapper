
#include "rust/cxx.h"
#include <memory>
#include "LogBuffers.h"

namespace aeron { namespace logbuffers {
inline std::unique_ptr<aeron::concurrent::AtomicBuffer> atomicBuffer(aeron::LogBuffers & logbuffers, int index){
auto wrapped_buffer = new aeron::concurrent::AtomicBuffer();
    wrapped_buffer->wrap(logbuffers.atomicBuffer(index));
    return std::unique_ptr<AtomicBuffer>(wrapped_buffer);
}

}}