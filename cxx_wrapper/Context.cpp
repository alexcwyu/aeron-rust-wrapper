
#include "rust/cxx.h"
#include <memory>
#include "Context.h"

namespace aeron {
void say_hello_context() {
    std::cout << "Hello, world from Context!" << std::endl;
}
}