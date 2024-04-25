
#include "rust/cxx.h"
#include <memory>
#include "concurrent/AtomicBuffer.h"
#include "concurrent/CountersReader.h"

namespace aeron { namespace concurrent { namespace counter_reader{
void sayHello() {
    std::cout << "Hello, world from CounterReader!" << std::endl;
}

//std::shared_ptr<aeron::concurrent::AtomicBuffer> valuesBuffer(const aeron::concurrent::CountersReader & value) {
//    return std::make_shared<aeron::concurrent::AtomicBuffer>(value.valuesBuffer());
//}
//
//std::shared_ptr<aeron::concurrent::AtomicBuffer> metaDataBuffer(const aeron::concurrent::CountersReader & value) {
//    return std::make_shared<aeron::concurrent::AtomicBuffer>(value.metaDataBuffer());
//}


std::unique_ptr<aeron::concurrent::AtomicBuffer> valuesBuffer(const aeron::concurrent::CountersReader & value) {
    return std::make_unique<aeron::concurrent::AtomicBuffer>(value.valuesBuffer());
}

std::unique_ptr<aeron::concurrent::AtomicBuffer> metaDataBuffer(const aeron::concurrent::CountersReader & value) {
    return std::make_unique<aeron::concurrent::AtomicBuffer>(value.metaDataBuffer());
}

}}}