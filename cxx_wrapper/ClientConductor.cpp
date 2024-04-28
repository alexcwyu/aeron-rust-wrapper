
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



inline void onStart(const std::shared_ptr<aeron::ClientConductor> &conductor) {
    conductor->onStart();
}

inline int doWork(const std::shared_ptr<aeron::ClientConductor> &conductor) {
    return conductor->doWork();
}

inline void onClose(const std::shared_ptr<aeron::ClientConductor> &conductor) {
    conductor->onClose();
}

inline std::int64_t addPublication(const std::shared_ptr<aeron::ClientConductor> &conductor,
 const rust::String channel, std::int32_t streamId) {
    std::string channelStr = std::string(channel);
    return conductor->addPublication(channelStr, streamId);
}


inline std::shared_ptr<Publication> findPublication(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    return conductor->findPublication(registrationId);
}

inline void releasePublication(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    conductor->releasePublication(registrationId);
}

inline std::int64_t addExclusivePublication(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::String channel, std::int32_t streamId) {
    std::string channelStr = std::string(channel);
    return conductor->addExclusivePublication(channelStr, streamId);
}


inline std::shared_ptr<ExclusivePublication> findExclusivePublication(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    return conductor->findExclusivePublication(registrationId);
}

inline void releaseExclusivePublication(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    conductor->releaseExclusivePublication(registrationId);
}

inline std::shared_ptr<Subscription> findSubscription(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    return conductor->findSubscription(registrationId);
}


inline std::int64_t addCounter(const std::shared_ptr<aeron::ClientConductor> &conductor,
        std::int32_t typeId,
        const std::uint8_t *keyBuffer,
        std::size_t keyLength,
        const rust::String label){
        std::string labelStr = std::string(label);
    return conductor->addCounter(typeId, keyBuffer, keyLength, labelStr);
}

inline std::shared_ptr<Counter> findCounter(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    return conductor->findCounter(registrationId);
}

inline void releaseCounter(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    conductor->releaseCounter(registrationId);
}

inline bool findDestinationResponse(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t correlationId){
    return conductor->findDestinationResponse(correlationId);
}

//
inline void onNewPublication(
    const std::shared_ptr<aeron::ClientConductor> &conductor,
        std::int64_t registrationId,
        std::int64_t originalRegistrationId,
        std::int32_t streamId,
        std::int32_t sessionId,
        std::int32_t publicationLimitCounterId,
        std::int32_t channelStatusIndicatorId,
        const std::string &logFilename){
        conductor->onNewPublication(registrationId, originalRegistrationId, streamId, sessionId,
        publicationLimitCounterId, channelStatusIndicatorId, logFilename);
      }

//
inline void onNewExclusivePublication(
    const std::shared_ptr<aeron::ClientConductor> &conductor,
        std::int64_t registrationId,
        std::int64_t originalRegistrationId,
        std::int32_t streamId,
        std::int32_t sessionId,
        std::int32_t publicationLimitCounterId,
        std::int32_t channelStatusIndicatorId,
        const std::string &logFilename){
        conductor->onNewExclusivePublication(registrationId, originalRegistrationId, streamId, sessionId,
        publicationLimitCounterId, channelStatusIndicatorId, logFilename);
        }

inline void onSubscriptionReady(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId, std::int32_t channelStatusId){
conductor->onSubscriptionReady(registrationId, channelStatusId);

}

inline void onOperationSuccess(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t correlationId){
conductor->onOperationSuccess(correlationId);
}

//
inline void onChannelEndpointErrorResponse(const std::shared_ptr<aeron::ClientConductor> &conductor,
    std::int32_t channelStatusId, const std::string &errorMessage){
    conductor->onChannelEndpointErrorResponse(channelStatusId, errorMessage);
    }

//
inline void onErrorResponse(const std::shared_ptr<aeron::ClientConductor> &conductor,
        std::int64_t offendingCommandCorrelationId, std::int32_t errorCode, const std::string &errorMessage){
        conductor->onErrorResponse(offendingCommandCorrelationId, errorCode, errorMessage);
        }

inline void onAvailableImage(const std::shared_ptr<aeron::ClientConductor> &conductor,
        std::int64_t correlationId,
        std::int32_t sessionId,
        std::int32_t subscriberPositionId,
        std::int64_t subscriptionRegistrationId,
        const std::string &logFilename,
        const std::string &sourceIdentity){
        conductor->onAvailableImage(correlationId, sessionId, subscriberPositionId, subscriptionRegistrationId, logFilename, sourceIdentity);
        }

inline void onUnavailableImage(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t correlationId, std::int64_t subscriptionRegistrationId){
conductor->onUnavailableImage(correlationId, subscriptionRegistrationId);
}

inline void onAvailableCounter(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t registrationId, std::int32_t counterId){
conductor->onAvailableCounter(registrationId, counterId);
}

inline void onUnavailableCounter(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t registrationId, std::int32_t counterId){
conductor->onUnavailableCounter(registrationId, counterId);
}

inline void onClientTimeout(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t clientId){
conductor->onClientTimeout(clientId);
}

inline void closeAllResources(const std::shared_ptr<aeron::ClientConductor> &conductor, long long nowMs){
conductor->closeAllResources(nowMs);
}

inline std::int64_t addDestination(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t publicationRegistrationId, const rust::String endpointChannel){
std::string channelStr = std::string(endpointChannel);

return conductor->addDestination(publicationRegistrationId, channelStr);
}

inline std::int64_t removeDestination(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t publicationRegistrationId, const rust::String endpointChannel){
std::string channelStr = std::string(endpointChannel);

return conductor->removeDestination(publicationRegistrationId, channelStr);
}

inline std::int64_t addRcvDestination(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t subscriptionRegistrationId, const rust::String endpointChannel){
std::string channelStr = std::string(endpointChannel);

return conductor->addRcvDestination(subscriptionRegistrationId, channelStr);
}

inline std::int64_t removeRcvDestination(const std::shared_ptr<aeron::ClientConductor> &conductor,std::int64_t subscriptionRegistrationId, const rust::String endpointChannel){
std::string channelStr = std::string(endpointChannel);

return conductor->removeRcvDestination(subscriptionRegistrationId, channelStr);
}


    inline void removeAvailableCounterHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    conductor->removeAvailableCounterHandler(registrationId);
    }



    inline void removeAvailableCounterHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::Fn<void(
                                                                                    CountersReader &countersReader,
                                                                                    std::int64_t registrationId,
                                                                                    std::int32_t counterId)> handler){
    conductor->removeAvailableCounterHandler(handler);
    }



    inline void removeUnavailableCounterHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){

    conductor->removeUnavailableCounterHandler(registrationId);
    }


    inline void removeUnavailableCounterHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::Fn<void(
                                                                                      CountersReader &countersReader,
                                                                                      std::int64_t registrationId,
                                                                                      std::int32_t counterId)> handler){

    conductor->removeUnavailableCounterHandler(handler);
    }


   inline void removeCloseClientHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::Fn<void()> handler){
    conductor->removeCloseClientHandler(handler);
    }


    inline void removeCloseClientHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, std::int64_t registrationId){
    conductor->removeCloseClientHandler(registrationId);
    }



inline std::int64_t addSubscription(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::String channel, std::int32_t streamId, const rust::Fn<void(Image &)> on_available_image_t, const rust::Fn<void(Image &)> on_unavailable_image_t) {
    std::string channelStr = std::string(channel);

    return conductor->addSubscription(channelStr, streamId, on_available_image_t, on_unavailable_image_t);
}




    inline std::int64_t addAvailableCounterHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::Fn<void(
                                                                                         CountersReader &countersReader,
                                                                                         std::int64_t registrationId,
                                                                                         std::int32_t counterId)> handler){
    return conductor->addAvailableCounterHandler(handler);
    }



    inline std::int64_t addUnavailableCounterHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::Fn<void(
                                                                                           CountersReader &countersReader,
                                                                                           std::int64_t registrationId,
                                                                                           std::int32_t counterId)> handler){

    return conductor->addUnavailableCounterHandler(handler);
    }



    inline const CountersReader & countersReader(const std::shared_ptr<aeron::ClientConductor> &conductor){
        return conductor->countersReader();
    }

    inline std::int64_t addCloseClientHandler(const std::shared_ptr<aeron::ClientConductor> &conductor, const rust::Fn<void()> handler){
    return conductor->addCloseClientHandler(handler);
    }



}}
