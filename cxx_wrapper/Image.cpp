
#include "rust/cxx.h"
#include <memory>
#include "Image.h"

namespace aeron {

//fragment_handler_t
int image_poll(aeron::Image &image, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return image.poll(fragmentHandler, fragmentLimit);
}

//fragment_handler_t
int image_bounded_poll(aeron::Image &image, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, std::int64_t limitPosition, int fragmentLimit) {
    return image.boundedPoll(fragmentHandler, limitPosition, fragmentLimit);
}

//controlled_poll_fragment_handler_t
int image_controlled_poll(aeron::Image &image, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return image.controlledPoll(fragmentHandler, fragmentLimit);
}

//controlled_poll_fragment_handler_t
int image_bounded_controlled_poll(aeron::Image &image, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, std::int64_t limitPosition, int fragmentLimit) {
    return image.boundedControlledPoll(fragmentHandler, limitPosition, fragmentLimit);
}

//controlled_poll_fragment_handler_t
std::int64_t image_controlled_peek(aeron::Image &image, std::int64_t initialPosition, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler,  std::int64_t limitPosition) {
    return image.controlledPeek(initialPosition, fragmentHandler, limitPosition);
}

//block_handler_t
int image_block_poll(aeron::Image &image, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, std::int32_t, std::int32_t)> blockHandler, int blockLengthLimit) {
    return image.blockPoll(blockHandler, blockLengthLimit);
}

void say_hello_image() {
    std::cout << "Hello, world from Image!" << std::endl;
}
}