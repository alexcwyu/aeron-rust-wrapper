
#include "rust/cxx.h"
#include <memory>
#include "ExclusivePublication.h"

namespace aeron { namespace exclusive_publication{

inline rust::String channel(const std::shared_ptr<aeron::ExclusivePublication> &publication) {
    return rust::String(publication->channel());
}

inline std::int64_t addDestination(const std::shared_ptr<aeron::ExclusivePublication> &publication,const rust::String endpointChannel){
    std::string endpointChannelStr = std::string(endpointChannel);
    return publication->addDestination(endpointChannelStr);
}

inline std::int64_t removeDestination(const std::shared_ptr<aeron::ExclusivePublication> &publication,const rust::String endpointChannel){
    std::string endpointChannelStr = std::string(endpointChannel);
    return publication->removeDestination(endpointChannelStr);
}

inline bool findDestinationResponse(const std::shared_ptr<aeron::ExclusivePublication> &publication, std::int64_t correlationId){
    return publication->findDestinationResponse(correlationId);
}

inline void close(const std::shared_ptr<aeron::ExclusivePublication> &publication){
    publication->close();
}

 inline std::int64_t offer(const std::shared_ptr<aeron::ExclusivePublication> &publication, const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length){
    return publication->offer(buffer, offset, length);
 }

inline std::int64_t offer(const std::shared_ptr<aeron::ExclusivePublication> &publication, const concurrent::AtomicBuffer &buffer){
    return publication->offer(buffer);
}

inline std::int64_t tryClaim(const std::shared_ptr<aeron::ExclusivePublication> &publication, util::index_t length, concurrent::logbuffer::BufferClaim &bufferClaim){
    return publication->tryClaim(length, bufferClaim);
}

inline std::int64_t offer(const std::shared_ptr<aeron::ExclusivePublication> &publication,
        const concurrent::AtomicBuffer &buffer,
        util::index_t offset,
        util::index_t length,
        const rust::Fn<std::int64_t(
                  AtomicBuffer &termBuffer,
                  util::index_t termOffset,
                  util::index_t length)> reservedValueSupplier)
        {

        return publication->offer(buffer, offset, length, reservedValueSupplier);
        }

inline std::int64_t offer(const std::shared_ptr<aeron::ExclusivePublication> &publication,
               const std::vector<concurrent::AtomicBuffer> &buffers,
               const rust::Fn<std::int64_t(
                                 AtomicBuffer &termBuffer,
                                 util::index_t termOffset,
                                 util::index_t length)> reservedValueSupplier)
           {
return publication->offer(buffers.data(), buffers.size(), reservedValueSupplier);
           }

}}