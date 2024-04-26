
#include "rust/cxx.h"
#include <memory>
#include "Publication.h"

namespace aeron { namespace publication {
void sayHello() {
    std::cout << "Hello, world from Publication!" << std::endl;
}

inline std::int64_t offer(
    aeron::Publication &publication,
        const concurrent::AtomicBuffer &buffer,
        util::index_t offset,
        util::index_t length,
        const rust::Fn<std::int64_t(
                  AtomicBuffer &termBuffer,
                  util::index_t termOffset,
                  util::index_t length)> reservedValueSupplier)
        {

        return publication.offer(buffer, offset, length, reservedValueSupplier);
        }

inline std::int64_t offer(
    aeron::Publication &publication,
               const std::vector<concurrent::AtomicBuffer> &buffers,
               const rust::Fn<std::int64_t(
                                 AtomicBuffer &termBuffer,
                                 util::index_t termOffset,
                                 util::index_t length)> reservedValueSupplier)
           {
return publication.offer(buffers.data(), buffers.size(), reservedValueSupplier);
           }
}}