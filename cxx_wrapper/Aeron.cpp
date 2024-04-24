
#include "rust/cxx.h"
#include <memory>
#include "Aeron.h"
#include "Publication.h"
#include "concurrent/AtomicBuffer.h"

namespace aeron { namespace aeron{
void sayHello() {
    std::cout << "Hello, world from Aeron!" << std::endl;
}
}}