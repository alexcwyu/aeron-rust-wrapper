
#include "rust/cxx.h"
#include <memory>
#include <string>
#include "Aeron.h"
#include "Context.h"
#include "Image.h"
#include "Publication.h"
#include "concurrent/AtomicBuffer.h"

namespace aeron {
void sayHello() {
    std::cout << "Hello, world from Aeron!" << std::endl;
}

inline std::shared_ptr<Aeron> connect(Context &context) {
    return Aeron::connect(context);
}


inline std::shared_ptr<Aeron> connect() {
    return Aeron::connect();
}

inline std::int64_t addSubscription(Aeron &aeron, const std::string &channel, std::int32_t streamId, const rust::Fn<void(Image &)> on_available_image_t, const rust::Fn<void(Image &)> on_unavailable_image_t) {
    return aeron.addSubscription(channel, streamId, on_available_image_t, on_unavailable_image_t);
}
}