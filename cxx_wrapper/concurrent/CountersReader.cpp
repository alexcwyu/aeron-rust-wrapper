
#include "rust/cxx.h"
#include <memory>
#include <string>
#include "concurrent/AtomicBuffer.h"
#include "concurrent/CountersReader.h"

namespace aeron { namespace concurrent { namespace counter_reader{

inline std::unique_ptr<aeron::concurrent::CountersReader> newInstance(const AtomicBuffer &metadataBuffer, const AtomicBuffer &valuesBuffer){
    return std::make_unique<aeron::concurrent::CountersReader>(metadataBuffer, valuesBuffer);
}

inline void getValuesBuffer(const aeron::concurrent::CountersReader & reader, const std::unique_ptr<AtomicBuffer> &buffer){
    buffer->wrap(reader.valuesBuffer());
}

inline void getMetaDataBuffer(const aeron::concurrent::CountersReader & reader, const std::unique_ptr<AtomicBuffer> &buffer){
    buffer->wrap(reader.metaDataBuffer());
}


inline rust::String getCounterLabel(const aeron::concurrent::CountersReader & reader, std::int32_t id){
    return  rust::String(reader.getCounterLabel(id));
}

inline void forEach(const aeron::concurrent::CountersReader & reader, rust::Fn<void(std::int32_t, std::int32_t, aeron::concurrent::AtomicBuffer const &, const std::string&)> function) {
    return reader.forEach(function);
}

}}}