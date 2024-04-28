
#include "rust/cxx.h"
#include <memory>
#include "concurrent/AtomicBuffer.h"
#include "concurrent/logbuffer/BufferClaim.h"


namespace aeron { namespace concurrent { namespace logbuffer{ namespace buffer_claim{

//std::unique_ptr<aeron::concurrent::AtomicBuffer> buffer(aeron::concurrent::logbuffer::BufferClaim & buffer_claim){
//auto wrapped_buffer = new aeron::concurrent::AtomicBuffer();
//    wrapped_buffer->wrap(buffer_claim.buffer());
//    return std::unique_ptr<AtomicBuffer>(wrapped_buffer);
//}

inline void getBuffer(aeron::concurrent::logbuffer::BufferClaim & buffer_claim, const std::unique_ptr<AtomicBuffer> &buffer){
    buffer->wrap(buffer_claim.buffer());
}


inline void wrapRawBuffer(aeron::concurrent::logbuffer::BufferClaim & buffer_claim, std::uint8_t *buffer, util::index_t length) {
    buffer_claim.wrap(buffer, length);
}


inline std::unique_ptr<aeron::concurrent::logbuffer::BufferClaim> newInstance(){
    return std::make_unique<aeron::concurrent::logbuffer::BufferClaim>();
}

}}}}