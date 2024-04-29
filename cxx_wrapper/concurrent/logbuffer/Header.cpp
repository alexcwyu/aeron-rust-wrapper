
#include "rust/cxx.h"
#include <memory>
#include "concurrent/AtomicBuffer.h"
#include "concurrent/logbuffer/Header.h"
#include "concurrent/logbuffer/DataFrameHeader.h"
#include "concurrent/logbuffer/LogBufferDescriptor.h"


namespace aeron { namespace concurrent { namespace logbuffer{  namespace header{

inline std::unique_ptr<aeron::concurrent::logbuffer::Header> newInstance() {
     return std::unique_ptr<aeron::concurrent::logbuffer::Header>(new aeron::concurrent::logbuffer::Header(0, 0, nullptr));
}

inline std::unique_ptr<aeron::concurrent::logbuffer::Header> newInstance(std::int32_t initialTermId, std::int32_t positionBitsToShift) {
     return std::unique_ptr<aeron::concurrent::logbuffer::Header>(new aeron::concurrent::logbuffer::Header(initialTermId, positionBitsToShift, nullptr));
}

inline std::uint16_t getType(const aeron::concurrent::logbuffer::Header & header){

   aeron::concurrent::logbuffer::Header& h = const_cast<aeron::concurrent::logbuffer::Header&>(header);
    return h.type();
}

inline void getBuffer(const aeron::concurrent::logbuffer::Header & header, const std::unique_ptr<AtomicBuffer> &buffer){
    aeron::concurrent::logbuffer::Header& h = const_cast<aeron::concurrent::logbuffer::Header&>(header);
    buffer->wrap(h.buffer());
}

inline void copyFrom(aeron::concurrent::logbuffer::Header & header,
const aeron::concurrent::logbuffer::Header & srcHeader){
   header.copyFrom(srcHeader);
}

inline void fragmentedFrameLength(aeron::concurrent::logbuffer::Header & header,
std::int32_t fragmentedFrameLength){
   header.fragmentedFrameLength(fragmentedFrameLength);
}

inline void initialTermId(aeron::concurrent::logbuffer::Header & header,
std::int32_t initialTermId){
   header.initialTermId(initialTermId);
}

inline void offset(aeron::concurrent::logbuffer::Header & header,
util::index_t offset){
   header.offset(offset);
}

inline void buffer(aeron::concurrent::logbuffer::Header & header,
aeron::concurrent::AtomicBuffer &buffer){
   header.buffer(buffer);
}

}}}}