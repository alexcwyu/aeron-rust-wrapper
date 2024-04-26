
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

}}