
#include "rust/cxx.h"
#include <memory>
#include "Counter.h"

namespace aeron { namespace counter{
void sayHello() {
    std::cout << "Hello, world from Counter!" << std::endl;
}

}}