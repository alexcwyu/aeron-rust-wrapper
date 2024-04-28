
#include "rust/cxx.h"
#include <memory>
#include "Publication.h"

namespace aeron { namespace publication {

//inline std::vector<concurrent::AtomicBuffer> create_buffer_vector() {
//    auto result = std::make_unique<std::vector<concurrent::AtomicBuffer>>();
//    return result;
//}

inline rust::String channel(const std::shared_ptr<aeron::Publication> &publication) {
    return rust::String(publication->channel());
}

inline std::int64_t addDestination(const std::shared_ptr<aeron::Publication> &publication,const rust::String endpointChannel){
    std::string endpointChannelStr = std::string(endpointChannel);
    return publication->addDestination(endpointChannelStr);
}

inline std::int64_t removeDestination(const std::shared_ptr<aeron::Publication> &publication,const rust::String endpointChannel){
    std::string endpointChannelStr = std::string(endpointChannel);
    return publication->removeDestination(endpointChannelStr);
}

inline bool findDestinationResponse(const std::shared_ptr<aeron::Publication> &publication, std::int64_t correlationId){
    return publication->findDestinationResponse(correlationId);
}

inline void close(const std::shared_ptr<aeron::Publication> &publication){
    publication->close();
}

 inline std::int64_t offer(const std::shared_ptr<aeron::Publication> &publication, const concurrent::AtomicBuffer &buffer, util::index_t offset, util::index_t length){
    return publication->offer(buffer, offset, length);
 }

inline std::int64_t offer(const std::shared_ptr<aeron::Publication> &publication, const concurrent::AtomicBuffer &buffer){
    return publication->offer(buffer);
}

inline std::int64_t tryClaim(const std::shared_ptr<aeron::Publication> &publication, util::index_t length, concurrent::logbuffer::BufferClaim &bufferClaim){
    return publication->tryClaim(length, bufferClaim);
}


inline std::int64_t offer(const std::shared_ptr<aeron::Publication> &publication,
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

inline std::int64_t offer(const std::shared_ptr<aeron::Publication> &publication,
               const std::vector<concurrent::AtomicBuffer> &buffers,
               const rust::Fn<std::int64_t(
                                 AtomicBuffer &termBuffer,
                                 util::index_t termOffset,
                                 util::index_t length)> reservedValueSupplier)
           {
              //std::vector<concurrent::AtomicBuffer> std_buffers;
              //std::copy(buffers.begin(), buffers.end(), std::back_inserter(std_buffers));
              return publication->offer(buffers.data(), buffers.size(), reservedValueSupplier);
           }
}}