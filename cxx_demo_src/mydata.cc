#include "aeron-rust-wrapper/cxx_demo_include/mydata.h"
#include "aeron-rust-wrapper/src/demo/mydata.rs.h"

namespace traits_demo{
BoxDynMyData::BoxDynMyData(BoxDynMyData &&other) noexcept : repr(other.repr) {
  other.repr = {0, 0};
}

BoxDynMyData::~BoxDynMyData() noexcept {
  if (repr != std::array<std::uintptr_t, 2>{0, 0}) {
    dyn_mydata_drop_in_place(this);
  }
}

void BoxDynMyData::traitfn() const noexcept {
  dyn_mydata_traitfn(*this);
}

void do_work() {
  std::cout << "do_work" << std::endl;
  auto mydata = read_data();
  mydata.traitfn();
}
}