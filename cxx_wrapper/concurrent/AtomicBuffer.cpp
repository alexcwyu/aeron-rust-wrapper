
#include "rust/cxx.h"
#include <memory>
#include "concurrent/AtomicBuffer.h"

namespace aeron { namespace concurrent { namespace atomic_buffer{
inline std::unique_ptr<aeron::concurrent::AtomicBuffer> newInstance(std::uint8_t *buffer, std::size_t length) {
     return std::unique_ptr<aeron::concurrent::AtomicBuffer>(new aeron::concurrent::AtomicBuffer(buffer, length));
}

inline std::unique_ptr<aeron::concurrent::AtomicBuffer> wrapAtomicBuffer(const aeron::concurrent::AtomicBuffer &buffer){
    auto wrapped_buffer = new aeron::concurrent::AtomicBuffer();
    wrapped_buffer->wrap(buffer);
    return std::unique_ptr<AtomicBuffer>(wrapped_buffer);
}

inline rust::String getString(const aeron::concurrent::AtomicBuffer &buffer, util::index_t offset){
    return buffer.getString(offset);
    }


inline rust::String getStringWithoutLength(const aeron::concurrent::AtomicBuffer &buffer, util::index_t offset, std::size_t length){
    return buffer.getStringWithoutLength(offset, length);
    }


}}}