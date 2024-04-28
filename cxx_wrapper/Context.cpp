
#include "rust/cxx.h"
#include <memory>
#include <string>
#include <iostream>
#include "Context.h"

namespace aeron { namespace context{

inline std::unique_ptr<aeron::Context> newInstance(const rust::String clientName) {
    std::string clientNameStr = std::string(clientName);
    Context * context = new aeron::Context();
    context->clientName(clientNameStr);
    return std::unique_ptr<aeron::Context>(context);
}


inline void conclude(const std::unique_ptr<aeron::Context> &context) {
    context->conclude();
}

inline void aeronDir(const std::unique_ptr<aeron::Context> &context, const std::string &directory){
    context->aeronDir(directory);
}

inline void clientName(const std::unique_ptr<aeron::Context> &context, const std::string &clientName){
    context->clientName(clientName);
}

inline void idleSleepDuration(const std::unique_ptr<aeron::Context> &context, long value){
    context->idleSleepDuration(value);
}

inline void mediaDriverTimeout(const std::unique_ptr<aeron::Context> &context, long value){
    context->mediaDriverTimeout(value);
}

inline void resourceLingerTimeout(const std::unique_ptr<aeron::Context> &context, long value){
    context->resourceLingerTimeout(value);
}

void useConductorAgentInvoker(const std::unique_ptr<aeron::Context> &context, bool useConductorAgentInvoker){
    context->useConductorAgentInvoker(useConductorAgentInvoker);
}

inline void preTouchMappedMemory(const std::unique_ptr<aeron::Context> &context, bool preTouchMappedMemory){
    context->preTouchMappedMemory(preTouchMappedMemory);
}

inline void newPublicationHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void(const std::string &channel, std::int32_t streamId, std::int32_t sessionId, std::int64_t correlationId)> handler) {
    context->newPublicationHandler(handler);
}

inline void newExclusivePublicationHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void(const std::string &channel, std::int32_t streamId, std::int32_t sessionId, std::int64_t correlationId)> handler) {
    context->newExclusivePublicationHandler(handler);
}

inline void newSubscriptionHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void(const std::string &channel, std::int32_t streamId, std::int64_t correlationId)> handler) {
    context->newSubscriptionHandler(handler);
}

inline void availableImageHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void(Image &image)> handler) {
    context->availableImageHandler(handler);
}

inline void unavailableImageHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void(Image &image)> handler) {
    context->unavailableImageHandler(handler);
}

inline void availableCounterHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void(CountersReader &countersReader, std::int64_t registrationId, std::int32_t counterId)> handler) {
    context->availableCounterHandler(handler);
}

inline void unavailableCounterHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void(CountersReader &countersReader, std::int64_t registrationId, std::int32_t counterId)> handler) {
    context->unavailableCounterHandler(handler);
}


inline void closeClientHandler(const std::unique_ptr<aeron::Context> &context, rust::Fn<void()> handler) {
    context->closeClientHandler(handler);
}

inline bool requestDriverTermination(const std::string &directory, const std::uint8_t *tokenBuffer, std::size_t tokenLength){
    return aeron::Context::requestDriverTermination(directory, tokenBuffer, tokenLength);
}

inline rust::String defaultAeronPath() {
    return aeron::Context::defaultAeronPath();
}

inline rust::String dirName(const aeron::Context &context) {
    return context.dirName();
}

inline rust::String cncFileName(const aeron::Context &context) {
    rust::String str = context.dirName() + std::string(1, AERON_FILE_SEP) + CncFileDescriptor::CNC_FILE;
    return str;
}
}}