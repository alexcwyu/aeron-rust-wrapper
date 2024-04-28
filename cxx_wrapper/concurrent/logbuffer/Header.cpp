
#include "rust/cxx.h"
#include <memory>
#include "concurrent/AtomicBuffer.h"
#include "concurrent/logbuffer/Header.h"
#include "concurrent/logbuffer/DataFrameHeader.h"
#include "concurrent/logbuffer/LogBufferDescriptor.h"


namespace aeron { namespace concurrent { namespace logbuffer{  namespace header{


inline std::uint16_t getType(const aeron::concurrent::logbuffer::Header & header){

   aeron::concurrent::logbuffer::Header& h = const_cast<aeron::concurrent::logbuffer::Header&>(header);
    return h.type();
}

inline void getBuffer(const aeron::concurrent::logbuffer::Header & header, const std::unique_ptr<AtomicBuffer> &buffer){
    aeron::concurrent::logbuffer::Header& h = const_cast<aeron::concurrent::logbuffer::Header&>(header);
    buffer->wrap(h.buffer());
}

//std::unique_ptr<aeron::concurrent::AtomicBuffer> buffer(aeron::concurrent::logbuffer::Header & header){
//auto wrapped_buffer = new aeron::concurrent::AtomicBuffer();
//    wrapped_buffer->wrap(header.buffer());
//    return std::unique_ptr<AtomicBuffer>(wrapped_buffer);
//}

}}}}