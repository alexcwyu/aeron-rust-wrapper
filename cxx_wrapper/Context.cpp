
#include "rust/cxx.h"
#include <memory>
#include "Context.h"

namespace aeron { namespace context{
void sayHello() {
    std::cout << "Hello, world from Context!" << std::endl;
}


aeron::Context & newPublicationHandler(aeron::Context &context, rust::Fn<void(const std::string &channel, std::int32_t streamId, std::int32_t sessionId, std::int64_t correlationId)> handler) {
    return context.newPublicationHandler(handler);
}

aeron::Context & newExclusivePublicationHandler(aeron::Context &context, rust::Fn<void(const std::string &channel, std::int32_t streamId, std::int32_t sessionId, std::int64_t correlationId)> handler) {
    return context.newExclusivePublicationHandler(handler);
}

aeron::Context & newSubscriptionHandler(aeron::Context &context, rust::Fn<void(const std::string &channel, std::int32_t streamId, std::int64_t correlationId)> handler) {
    return context.newSubscriptionHandler(handler);
}

aeron::Context & availableImageHandler(aeron::Context &context, rust::Fn<void(Image &image)> handler) {
    return context.availableImageHandler(handler);
}

aeron::Context & unavailableImageHandler(aeron::Context &context, rust::Fn<void(Image &image)> handler) {
    return context.unavailableImageHandler(handler);
}

aeron::Context & availableCounterHandler(aeron::Context &context, rust::Fn<void(CountersReader &countersReader, std::int64_t registrationId, std::int32_t counterId)> handler) {
    return context.availableCounterHandler(handler);
}

aeron::Context & unavailableCounterHandler(aeron::Context &context, rust::Fn<void(CountersReader &countersReader, std::int64_t registrationId, std::int32_t counterId)> handler) {
    return context.unavailableCounterHandler(handler);
}


aeron::Context & closeClientHandler(aeron::Context &context, rust::Fn<void()> handler) {
    return context.closeClientHandler(handler);
}

bool requestDriverTermination(const std::string &directory, const std::uint8_t *tokenBuffer, std::size_t tokenLength){
    return aeron::Context::requestDriverTermination(directory, tokenBuffer, tokenLength);
}

rust::String defaultAeronPath() {
    return aeron::Context::defaultAeronPath();
}


}}