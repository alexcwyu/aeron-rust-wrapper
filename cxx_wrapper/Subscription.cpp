
#include "rust/cxx.h"
#include <memory>
#include "Subscription.h"

namespace aeron { namespace subscription{
void sayHello() {
    std::cout << "Hello, world from Subscription!" << std::endl;
}

int poll(aeron::Subscription &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription.poll(fragmentHandler, fragmentLimit);
}

//controlled_poll_fragment_handler_t
int controlledPoll(aeron::Subscription &subscription, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription.controlledPoll(fragmentHandler, fragmentLimit);
}

//block_handler_t
int blockPoll(aeron::Subscription &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, std::int32_t, std::int32_t)> blockHandler, int blockLengthLimit) {
    return subscription.blockPoll(blockHandler, blockLengthLimit);
}
}}