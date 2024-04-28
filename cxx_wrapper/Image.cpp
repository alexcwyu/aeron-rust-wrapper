
#include "rust/cxx.h"
#include <memory>
#include "Image.h"

namespace aeron { namespace image{


inline void position(const std::shared_ptr<aeron::Image> &image, std::int64_t newPosition){
    image->position(newPosition);
}

inline void close(const std::shared_ptr<aeron::Image> &image){
    image->close();
}

inline rust::String sourceIdentity(const std::shared_ptr<aeron::Image> &image){
    return  rust::String(image->sourceIdentity());
}


//fragment_handler_t
inline int poll(const std::shared_ptr<aeron::Image> &image, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return image->poll(fragmentHandler, fragmentLimit);
}

//fragment_handler_t
inline int boundedPoll(const std::shared_ptr<aeron::Image> &image, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, std::int64_t limitPosition, int fragmentLimit) {
    return image->boundedPoll(fragmentHandler, limitPosition, fragmentLimit);
}

//controlled_poll_fragment_handler_t
inline int controlledPoll(const std::shared_ptr<aeron::Image> &image, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return image->controlledPoll(fragmentHandler, fragmentLimit);
}

//controlled_poll_fragment_handler_t
inline int boundedControlledPoll(const std::shared_ptr<aeron::Image> &image, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, std::int64_t limitPosition, int fragmentLimit) {
    return image->boundedControlledPoll(fragmentHandler, limitPosition, fragmentLimit);
}

//controlled_poll_fragment_handler_t
inline std::int64_t controlledPeek(const std::shared_ptr<aeron::Image> &image, std::int64_t initialPosition, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler,  std::int64_t limitPosition) {
    return image->controlledPeek(initialPosition, fragmentHandler, limitPosition);
}

//block_handler_t
inline int blockPoll(const std::shared_ptr<aeron::Image> &image, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, std::int32_t, std::int32_t)> blockHandler, int blockLengthLimit) {
    return image->blockPoll(blockHandler, blockLengthLimit);
}

}}