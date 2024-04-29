
#include "rust/cxx.h"
#include <memory>
#include "BufferBuilder.h"

namespace aeron { namespace buffer_builder {

inline std::unique_ptr<aeron::BufferBuilder> newInstance(std::uint32_t initialCapacity) {
     return std::unique_ptr<aeron::BufferBuilder>(new aeron::BufferBuilder(initialCapacity));
}

inline void limit(const std::unique_ptr<aeron::BufferBuilder> &builder, std::uint32_t limit){
    builder->limit(limit);
}

inline void nextTermOffset(const std::unique_ptr<aeron::BufferBuilder> &builder, util::index_t offset){
    builder->nextTermOffset(offset);
}

inline void reset(const std::unique_ptr<aeron::BufferBuilder> &builder){
    builder->reset();
}

inline void compact(const std::unique_ptr<aeron::BufferBuilder> &builder){
    builder->compact();
}

inline void append(const std::unique_ptr<aeron::BufferBuilder> &builder, const aeron::concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length){


    aeron::concurrent::AtomicBuffer& b = const_cast<aeron::concurrent::AtomicBuffer&>(buffer);
    builder->append(b, offset, length);
}

inline void captureHeader(const std::unique_ptr<aeron::BufferBuilder> &builder, const Header &header){
    builder->captureHeader(header);
}

inline std::shared_ptr<aeron::concurrent::logbuffer::Header> completeHeader(const std::unique_ptr<aeron::BufferBuilder> &builder, const Header &header){
    return std::make_shared<aeron::concurrent::logbuffer::Header>(builder->completeHeader(header));
}

inline std::shared_ptr<aeron::concurrent::logbuffer::Header> completeHeader(const std::unique_ptr<aeron::BufferBuilder> &builder){
    return std::make_shared<aeron::concurrent::logbuffer::Header>(builder->completeHeader());
}

}}