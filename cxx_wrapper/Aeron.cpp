
#include "rust/cxx.h"
#include <memory>
#include <string>
#include "Aeron.h"
#include "Counter.h"
#include "Context.h"
#include "Image.h"
#include "Publication.h"
#include "concurrent/AtomicBuffer.h"

namespace aeron {

inline std::shared_ptr<Aeron> newInstance(Context &context) {
    return std::shared_ptr<Aeron>(new Aeron(context));
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

inline bool isClosed(const std::shared_ptr<aeron::Aeron> &aeron) {
    return aeron->isClosed();
}

inline std::int64_t addPublication(const std::shared_ptr<aeron::Aeron> &aeron, const rust::String channel, std::int32_t streamId)
{
    std::string channelStr = std::string(channel);
    return aeron->addPublication(channelStr, streamId);
}


inline std::int64_t addExclusivePublication(const std::shared_ptr<aeron::Aeron> &aeron, const rust::String channel, std::int32_t streamId)
{
    std::string channelStr = std::string(channel);
    return aeron->addExclusivePublication(channelStr, streamId);
}


inline std::shared_ptr<Publication> findPublication(const std::shared_ptr<aeron::Aeron> &aeron, std::int64_t registrationId)
{
return aeron->findPublication(registrationId);
}

inline std::shared_ptr<ExclusivePublication> findExclusivePublication(const std::shared_ptr<aeron::Aeron> &aeron, std::int64_t registrationId)
{
return aeron->findExclusivePublication(registrationId);
}

inline std::int64_t addSubscription(const std::shared_ptr<aeron::Aeron> &aeron, const  rust::String channel, std::int32_t streamId)
{
std::string channelStr = std::string(channel);
    return aeron->addSubscription(channelStr, streamId);
}

inline std::shared_ptr<Subscription> findSubscription(const std::shared_ptr<aeron::Aeron> &aeron, std::int64_t registrationId)
{
    return aeron->findSubscription(registrationId);
}

inline std::int64_t nextCorrelationId(const std::shared_ptr<aeron::Aeron> &aeron)
{
    return aeron->nextCorrelationId();
}
    
inline std::int64_t addCounter(const std::shared_ptr<aeron::Aeron> &aeron, 
        std::int32_t typeId,
        const std::uint8_t *keyBuffer,
        std::size_t keyLength,
        const rust::String label)
{

    std::string labelStr = std::string(label);
    return aeron->addCounter(typeId, keyBuffer, keyLength,labelStr);
}

inline std::shared_ptr<Counter> findCounter(const std::shared_ptr<aeron::Aeron> &aeron, std::int64_t registrationId){
    return aeron->findCounter(registrationId);
}


//on_available_counter_t
//inline void removeAvailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, const on_available_counter_t &handler)
inline void removeAvailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron,  const rust::Fn<void(
aeron::concurrent::CountersReader &countersReader,
std::int64_t registrationId,
std::int32_t counterId)> handler){
aeron->removeAvailableCounterHandler(handler);
}

inline void removeAvailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, std::int64_t registrationId)
{
aeron->removeAvailableCounterHandler(registrationId);
}

 //on_unavailable_counter_t
 //inline void removeUnavailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, const on_unavailable_counter_t &handler)
inline void removeUnavailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, const rust::Fn<void(
aeron::concurrent::CountersReader &countersReader,
std::int64_t registrationId,
std::int32_t counterId)> handler){
                                                                                        
aeron->removeUnavailableCounterHandler(handler);
}
    
inline void removeUnavailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, std::int64_t registrationId)
{
aeron->removeUnavailableCounterHandler(registrationId);
}
    //on_close_client_t
    //inline void removeCloseClientHandler(const on_close_client_t &handler)
inline void removeCloseClientHandler(const std::shared_ptr<aeron::Aeron> &aeron, const rust::Fn<void()> handler){
                                                                                   aeron->removeCloseClientHandler(handler);
                                                                                   }
    
inline void removeCloseClientHandler(const std::shared_ptr<aeron::Aeron> &aeron, std::int64_t registrationId)
{
aeron->removeCloseClientHandler(registrationId);
}
    
inline std::shared_ptr<aeron::concurrent::CountersReader> countersReader(const std::shared_ptr<aeron::Aeron> &aeron)
{
return std::make_shared<aeron::concurrent::CountersReader>(aeron->countersReader());
}

inline aeron::Context &context(const std::shared_ptr<aeron::Aeron> &aeron)
{
return aeron->context();
}


//typedef std::function<void(Image &image)> on_available_image_t;
//typedef std::function<void(Image &image)> on_unavailable_image_t;
//inline std::int64_t addSubscription(const std::string &channel, std::int32_t streamId, const on_available_image_t &onAvailableImageHandler, const on_unavailable_image_t &onUnavailableImageHandler)
inline std::int64_t addSubscription(const std::shared_ptr<aeron::Aeron> &aeron, const std::string &channel, std::int32_t streamId, const rust::Fn<void(aeron::Image &)> on_available_image_t, const rust::Fn<void(aeron::Image &)> on_unavailable_image_t) {
    return aeron->addSubscription(channel, streamId, on_available_image_t, on_unavailable_image_t);
}


//on_available_counter_t
//typedef std::function<void(CountersReader &countersReader,std::int64_t registrationId,std::int32_t counterId)> on_available_counter_t;
//inline std::int64_t addAvailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, const on_available_counter_t &handler)
inline std::int64_t addAvailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, const rust::Fn<void(aeron::concurrent::CountersReader &, std::int64_t, std::int32_t)> handler)
{
    return aeron->addAvailableCounterHandler(handler);
}


//on_unavailable_counter_t
//typedef std::function<void(CountersReader &countersReader,std::int64_t registrationId,std::int32_t counterId)> on_unavailable_counter_t;
//inline std::int64_t addUnavailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, const on_unavailable_counter_t &handler)
inline std::int64_t addUnavailableCounterHandler(const std::shared_ptr<aeron::Aeron> &aeron, const rust::Fn<void(aeron::concurrent::CountersReader &, std::int64_t, std::int32_t)> handler)
{
   return aeron->addUnavailableCounterHandler(handler);

}



//on_close_client_t
//typedef std::function<void()> on_close_client_t;
//inline std::int64_t addCloseClientHandler(const std::shared_ptr<aeron::Aeron> &aeron, const on_close_client_t &handler)
inline std::int64_t addCloseClientHandler(const std::shared_ptr<aeron::Aeron> &aeron, const rust::Fn<void()> handler)
{
    return aeron->addCloseClientHandler(handler);
}


}
