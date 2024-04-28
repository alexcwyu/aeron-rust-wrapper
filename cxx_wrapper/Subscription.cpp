
#include "rust/cxx.h"
#include <memory>
#include "Subscription.h"
#include "Image.h"

namespace aeron { namespace subscription{

//class Images
//{
//public:
//Images(std::shared_ptr<Image>* images, std::size_t size) : m_images(images), m_size(size) {}
//
//
//Images(std::pair<Image::array_t, std::size_t> pair) : m_images(pair.first), m_size(pair.second) {}
//
//
//std::size_t size() const
//{
//    return m_size;
//}
//
//std::shared_ptr<Image> getImage(std::size_t index) const
//{
//    if (index >= m_size)
//    {
//        throw std::out_of_range("index out of range");
//    }
//
//    return m_images[index];
//}
//
//std::shared_ptr<Image>* getImages() const
//{
//    return m_images;
//}
//
//private:
//std::shared_ptr<Image>* m_images;
//std::size_t m_size;
//};


inline rust::String channel(const std::shared_ptr<aeron::Subscription> &subscription){
    return rust::String(subscription->channel());
}


inline std::int64_t addDestination(const std::shared_ptr<aeron::Subscription> &subscription,const rust::String endpointChannel){
    return subscription->addDestination(std::string(endpointChannel));
}

inline std::int64_t removeDestination(const std::shared_ptr<aeron::Subscription> &subscription,const rust::String endpointChannel){
    return subscription->removeDestination(std::string(endpointChannel));
}

inline bool findDestinationResponse(const std::shared_ptr<aeron::Subscription> &subscription, std::int64_t correlationId){
    return subscription->findDestinationResponse(correlationId);
}

inline int poll(const std::shared_ptr<aeron::Subscription> &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription->poll(fragmentHandler, fragmentLimit);
}

//controlled_poll_fragment_handler_t
inline int controlledPoll(const std::shared_ptr<aeron::Subscription> &subscription, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription->controlledPoll(fragmentHandler, fragmentLimit);
}

//block_handler_t
inline int blockPoll(const std::shared_ptr<aeron::Subscription> &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, std::int32_t, std::int32_t)> blockHandler, int blockLengthLimit) {
    return subscription->blockPoll(blockHandler, blockLengthLimit);
}

inline int forEachImage(const std::shared_ptr<aeron::Subscription> &subscription, rust::Fn<void(aeron::Image &)> imageHandler) {
    return subscription->forEachImage(imageHandler);
}

//inline std::shared_ptr<Image> * addImage(const std::shared_ptr<aeron::Subscription> &subscription, std::shared_ptr<Image> image){
//    return subscription->addImage(image);
//}
//
//inline std::unique_ptr<Images> removeImage(const std::shared_ptr<aeron::Subscription> &subscription, std::int64_t correlationId){
//    return std::unique_ptr<Images>(new Images(subscription->removeImage(correlationId)));
//}
//
//inline std::unique_ptr<Images> closeAndRemoveImages(const std::shared_ptr<aeron::Subscription> &subscription){
//    return std::unique_ptr<Images>(new Images(subscription->closeAndRemoveImages()));
//}

inline void addImage(const std::shared_ptr<aeron::Subscription> &subscription, std::shared_ptr<Image> image){
    subscription->addImage(image);
}

inline void removeImage(const std::shared_ptr<aeron::Subscription> &subscription, std::int64_t correlationId){
    subscription->removeImage(correlationId);
}

inline void closeAndRemoveImages(const std::shared_ptr<aeron::Subscription> &subscription){
    subscription->closeAndRemoveImages();
}


inline rust::String tryResolveChannelEndpointPort(const std::shared_ptr<aeron::Subscription> &subscription){
    return rust::String(subscription->tryResolveChannelEndpointPort());
}

inline rust::String resolvedEndpoint(const std::shared_ptr<aeron::Subscription> &subscription){
    return rust::String(subscription->resolvedEndpoint());
}

        //std::vector<std::string> localSocketAddresses() const

        //std::string tryResolveChannelEndpointPort() const

        //std::string resolvedEndpoint() const

}}