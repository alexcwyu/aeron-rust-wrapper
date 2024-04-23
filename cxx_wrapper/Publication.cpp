
#include "rust/cxx.h"
#include <memory>
#include "Publication.h"

namespace aeron { namespace publication {
void sayHello() {
    std::cout << "Hello, world from Publication!" << std::endl;
}
}}