
#include "rust/cxx.h"
#include <memory>
#include "Subscription.h"
#include "Image.h"

namespace aeron { namespace subscription{

class Images
{
public:
Images(std::shared_ptr<Image>* images, std::size_t size) : m_images(images), m_size(size) {}


Images(std::pair<Image::array_t, std::size_t> pair) : m_images(pair.first), m_size(pair.second) {}


std::size_t size() const
{
    return m_size;
}

std::shared_ptr<Image> getImage(std::size_t index) const
{
    if (index >= m_size)
    {
        throw std::out_of_range("index out of range");
    }

    return m_images[index];
}

std::shared_ptr<Image>* getImages() const
{
    return m_images;
}

private:
std::shared_ptr<Image>* m_images;
std::size_t m_size;
};

void sayHello() {
    std::cout << "Hello, world from Subscription!" << std::endl;
}

int poll(aeron::Subscription &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription.poll(fragmentHandler, fragmentLimit);
}

//controlled_poll_fragment_handler_t
int controlledPoll(aeron::Subscription &subscription, rust::Fn<aeron::ControlledPollAction(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, int fragmentLimit) {
    return subscription.controlledPoll(fragmentHandler, fragmentLimit);
}

//block_handler_t
int blockPoll(aeron::Subscription &subscription, rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, std::int32_t, std::int32_t)> blockHandler, int blockLengthLimit) {
    return subscription.blockPoll(blockHandler, blockLengthLimit);
}

int forEachImage(aeron::Subscription &subscription, rust::Fn<void(aeron::Image &)> imageHandler) {
    return subscription.forEachImage(imageHandler);
}

std::unique_ptr<Images> removeImage(aeron::Subscription &subscription, std::int64_t correlationId){
    return std::unique_ptr<Images>(new Images(subscription.removeImage(correlationId)));
}

std::unique_ptr<Images> closeAndRemoveImages(aeron::Subscription &subscription){
    return std::unique_ptr<Images>(new Images(subscription.closeAndRemoveImages()));
}

        //std::vector<std::string> localSocketAddresses() const

        //std::string tryResolveChannelEndpointPort() const

        //std::string resolvedEndpoint() const

}}