
#include "rust/cxx.h"
#include <iostream>
#include <string>
#include <memory>
#include <functional>


namespace closures_ffi{
using c_void = void;

struct Counter;

typedef void (*AddCallbackPtr)(int result);

typedef std::function<void(std::int32_t)> AddCallback;

typedef std::function<void(std::int32_t, void *)> AddCallbackWData;

void hello();

void cppcb(int result);

void simple_add_two_numbers2(int a, int b, const AddCallbackPtr cb);

void simple_add_two_numbers3(int a, int b, const AddCallback & cb);

void simple_add_two_numbers1(int a, int b, const rust::Fn<void(std::int32_t)> cb);


void better_add_two_numbers(int a, int b, const rust::Fn<void(std::int32_t, Counter &)> cb, Counter & counter);

void void_ptr(void * data);
void void_ptr2(void * data);
}