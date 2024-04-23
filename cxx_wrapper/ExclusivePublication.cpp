
#include "rust/cxx.h"
#include <memory>
#include "ExclusivePublication.h"

namespace aeron {
void say_hello_exclusive_publication() {
    std::cout << "Hello, world from ExclusivePublication!" << std::endl;
}
}