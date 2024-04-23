
#include "rust/cxx.h"
#include <memory>
#include "Subscription.h"

namespace aeron {
void say_hello_subscription() {
    std::cout << "Hello, world from Subscription!" << std::endl;
}

int subscription_poll(aeron::Subscription &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription.poll(fragmentHandler, fragmentLimit);
}

//controlled_poll_fragment_handler_t
int subscription_controlled_poll(aeron::Subscription &subscription, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription.controlledPoll(fragmentHandler, fragmentLimit);
}

//block_handler_t
int subscription_block_poll(aeron::Subscription &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, std::int32_t, std::int32_t)> blockHandler, int blockLengthLimit) {
    return subscription.blockPoll(blockHandler, blockLengthLimit);
}
}