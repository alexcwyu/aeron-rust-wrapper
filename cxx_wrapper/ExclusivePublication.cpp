
#include "rust/cxx.h"
#include <memory>
#include "ExclusivePublication.h"

namespace aeron { namespace exclusive_publication{
void sayHello() {
    std::cout << "Hello, world from ExclusivePublication!" << std::endl;
}
}}