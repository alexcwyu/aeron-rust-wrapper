
#include "rust/cxx.h"
#include <memory>
#include "Aeron.h"

namespace aeron {
void say_hello_aeron() {
    std::cout << "Hello, world from Aeron!" << std::endl;
}
}