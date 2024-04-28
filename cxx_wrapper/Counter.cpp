
#include "rust/cxx.h"
#include <memory>
#include "Counter.h"

namespace aeron { namespace counter{

inline rust::String label(const std::shared_ptr<aeron::Counter> &counter) {
    return rust::String(counter->label());
}
inline void close(const std::shared_ptr<aeron::Counter> &counter){
    counter->close();
}

}}