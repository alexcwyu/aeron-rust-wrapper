
#include "rust/cxx.h"
#include <memory>
#include "Publication.h"

namespace aeron {
void say_hello_publication() {
    std::cout << "Hello, world from Publication!" << std::endl;
}
}