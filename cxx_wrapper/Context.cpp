
#include "rust/cxx.h"
#include <memory>
#include "Context.h"

namespace aeron { namespace context{
void sayHello() {
    std::cout << "Hello, world from Context!" << std::endl;
}
}}