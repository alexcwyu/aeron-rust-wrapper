
#include "rust/cxx.h"
#include <memory>
#include "LogBuffers.h"

namespace aeron {
void say_hello_log_buffers() {
    std::cout << "Hello, world from LogBuffers!" << std::endl;
}

std::unique_ptr<aeron::concurrent::AtomicBuffer> get_buffer_from_log_buffers(aeron::LogBuffers & logbuffers, int index){
auto wrapped_buffer = new aeron::concurrent::AtomicBuffer();
    wrapped_buffer->wrap(logbuffers.atomicBuffer(index));
    return std::unique_ptr<AtomicBuffer>(wrapped_buffer);
}
}