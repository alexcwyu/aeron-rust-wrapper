
#include "rust/cxx.h"
#include <memory>
#include <string>
#include "Aeron.h"
#include "Counter.h"
#include "Context.h"
#include "LogBuffers.h"
#include "Image.h"
#include "Publication.h"
#include "concurrent/AtomicBuffer.h"

namespace aeron { namespace client_conductor {

inline std::int64_t addSubscription(ClientConductor & clientConductor, const std::string &channel, std::int32_t streamId, const rust::Fn<void(Image &)> on_available_image_t, const rust::Fn<void(Image &)> on_unavailable_image_t) {
    return clientConductor.addSubscription(channel, streamId, on_available_image_t, on_unavailable_image_t);
}


    inline std::int64_t addAvailableCounterHandler(ClientConductor & clientConductor, const rust::Fn<void(
                                                                                         CountersReader &countersReader,
                                                                                         std::int64_t registrationId,
                                                                                         std::int32_t counterId)> handler){
    return clientConductor.addAvailableCounterHandler(handler);
    }

    inline void removeAvailableCounterHandler(ClientConductor & clientConductor, const rust::Fn<void(
                                                                                    CountersReader &countersReader,
                                                                                    std::int64_t registrationId,
                                                                                    std::int32_t counterId)> handler){

    clientConductor.removeAvailableCounterHandler(handler);
    }

    inline std::int64_t addUnavailableCounterHandler(ClientConductor & clientConductor, const rust::Fn<void(
                                                                                           CountersReader &countersReader,
                                                                                           std::int64_t registrationId,
                                                                                           std::int32_t counterId)> handler){

    return clientConductor.addUnavailableCounterHandler(handler);
    }

    inline void removeUnavailableCounterHandler(ClientConductor & clientConductor, const rust::Fn<void(
                                                                                      CountersReader &countersReader,
                                                                                      std::int64_t registrationId,
                                                                                      std::int32_t counterId)> handler){

    clientConductor.removeUnavailableCounterHandler(handler);
    }

    inline std::int64_t addCloseClientHandler(ClientConductor & clientConductor, const rust::Fn<void()> handler){
    return clientConductor.addCloseClientHandler(handler);
    }

    inline void removeCloseClientHandler(ClientConductor & clientConductor, const rust::Fn<void()> handler){
    clientConductor.removeCloseClientHandler(handler);
    }


}}
