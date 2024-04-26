
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

inline rust::String version() {
    return Aeron::version();
}

//typedef std::function<void(Image &image)> on_available_image_t;
//typedef std::function<void(Image &image)> on_unavailable_image_t;
//inline std::int64_t addSubscription(const std::string &channel, std::int32_t streamId, const on_available_image_t &onAvailableImageHandler, const on_unavailable_image_t &onUnavailableImageHandler)
inline std::int64_t addSubscription(Aeron &aeron, const std::string &channel, std::int32_t streamId, const rust::Fn<void(Image &)> on_available_image_t, const rust::Fn<void(Image &)> on_unavailable_image_t) {
    return aeron.addSubscription(channel, streamId, on_available_image_t, on_unavailable_image_t);
}


//on_available_counter_t
//typedef std::function<void(CountersReader &countersReader,std::int64_t registrationId,std::int32_t counterId)> on_available_counter_t;
//inline std::int64_t addAvailableCounterHandler(Aeron &aeron, const on_available_counter_t &handler)
inline std::int64_t addAvailableCounterHandler(Aeron &aeron, const rust::Fn<void(CountersReader &, std::int64_t, std::int32_t)> handler)
{
    return aeron.addAvailableCounterHandler(handler);
}

//on_available_counter_t
//inline void removeAvailableCounterHandler(Aeron &aeron, const on_available_counter_t &handler)
inline void removeAvailableCounterHandler(Aeron &aeron, const rust::Fn<void(CountersReader &, std::int64_t, std::int32_t)> handler)
{
    aeron.removeAvailableCounterHandler(handler);
}

//on_unavailable_counter_t
//typedef std::function<void(CountersReader &countersReader,std::int64_t registrationId,std::int32_t counterId)> on_unavailable_counter_t;
//inline std::int64_t addUnavailableCounterHandler(Aeron &aeron, const on_unavailable_counter_t &handler)
inline std::int64_t addUnavailableCounterHandler(Aeron &aeron, const rust::Fn<void(CountersReader &, std::int64_t, std::int32_t)> handler)
{
   return aeron.addUnavailableCounterHandler(handler);

}

//on_unavailable_counter_t
//inline void removeUnavailableCounterHandler(Aeron &aeron, const on_unavailable_counter_t &handler)
inline void removeUnavailableCounterHandler(Aeron &aeron, const rust::Fn<void(CountersReader &, std::int64_t, std::int32_t)> handler)
{
    aeron.removeUnavailableCounterHandler(handler);
}

//on_close_client_t
//typedef std::function<void()> on_close_client_t;
//inline std::int64_t addCloseClientHandler(Aeron &aeron, const on_close_client_t &handler)
inline std::int64_t addCloseClientHandler(Aeron &aeron, const rust::Fn<void()> handler)
{
    return aeron.addCloseClientHandler(handler);
}

//on_close_client_t
//inline void removeCloseClientHandler(const on_close_client_t &handler)
inline void removeCloseClientHandler(Aeron &aeron, const rust::Fn<void()> handler)
{
    return aeron.removeCloseClientHandler(handler);
}



}
