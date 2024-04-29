
#include "rust/cxx.h"
#include <memory>
#include "FragmentAssembler.h"

namespace aeron { namespace fragment_assembler {

inline std::unique_ptr<aeron::FragmentAssembler> newInstance(const rust::Fn<void(aeron::concurrent::AtomicBuffer const &, std::int32_t, std::int32_t, aeron::concurrent::logbuffer::Header const &)> fragmentHandler, std::size_t initialBufferLength) {
     return std::unique_ptr<aeron::FragmentAssembler>(new aeron::FragmentAssembler(fragmentHandler, initialBufferLength));
}

inline  void deleteSessionBuffer(const std::unique_ptr<aeron::FragmentAssembler> &fragment_assembler, std::int32_t sessionId){
    fragment_assembler->deleteSessionBuffer(sessionId);
}

}}