
#include "rust/cxx.h"
#include <memory>
#include "concurrent/AtomicBuffer.h"

namespace aeron { namespace concurrent {
std::unique_ptr<aeron::concurrent::AtomicBuffer> new_atomic_buffer(std::uint8_t *buffer, std::size_t length) {
     return std::unique_ptr<aeron::concurrent::AtomicBuffer>(new aeron::concurrent::AtomicBuffer(buffer, length));
}

std::unique_ptr<aeron::concurrent::AtomicBuffer> wrap_atomic_buffer(const aeron::concurrent::AtomicBuffer &buffer){
    auto wrapped_buffer = new aeron::concurrent::AtomicBuffer();
    wrapped_buffer->wrap(buffer);
    return std::unique_ptr<AtomicBuffer>(wrapped_buffer);
}

}}