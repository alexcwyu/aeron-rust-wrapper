
#include "rust/cxx.h"
#include <memory>
#include "concurrent/AtomicBuffer.h"
#include "concurrent/logbuffer/Header.h"
#include "concurrent/logbuffer/DataFrameHeader.h"
#include "concurrent/logbuffer/LogBufferDescriptor.h"


namespace aeron { namespace concurrent { namespace logbuffer{  namespace header{

std::unique_ptr<aeron::concurrent::AtomicBuffer> buffer(aeron::concurrent::logbuffer::Header & header){
auto wrapped_buffer = new aeron::concurrent::AtomicBuffer();
    wrapped_buffer->wrap(header.buffer());
    return std::unique_ptr<AtomicBuffer>(wrapped_buffer);
}

void sayHello() {
    std::cout << "Hello, world from Header!" << std::endl;
}
}}}}