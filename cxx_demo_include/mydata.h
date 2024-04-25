#pragma once

#include "rust/cxx.h"
#include <array>
#include <cstdint>
#include <type_traits>
#include <iostream>


namespace traits_demo{
class BoxDynMyData {
public:
  BoxDynMyData(BoxDynMyData &&) noexcept;
  ~BoxDynMyData() noexcept;
  using IsRelocatable = std::true_type;

  void traitfn() const noexcept;

private:
  std::array<std::uintptr_t, 2> repr;
};

using PtrBoxDynMyData = BoxDynMyData*;


void do_work();
}