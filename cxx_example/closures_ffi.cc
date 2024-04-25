#include "aeron-rust-wrapper/cxx_wrapper/closures_ffi.h"
#include "aeron-rust-wrapper/src/closures_ffi.rs.h"

#include <iostream>
#include <string>
#include <memory>
#include <functional>


namespace closures_ffi{

void hello(){
    std::cout<<"hello from C++"<<std::endl;
}

void cppcb(int result){
    std::cout<<"callback from C++: result = "<<result<<std::endl;
}

void simple_add_two_numbers2(int a, int b, const AddCallbackPtr cb)
{
    int result = a + b;
    std::cout<<"calling in C2, a = "<<a<<", b = "<<b<<", result = "<<result<< std::endl;
    cb(result);
}

void simple_add_two_numbers3(int a, int b, const AddCallback & cb)
{
    int result = a + b;
    std::cout<<"calling in C3, a = "<<a<<", b = "<<b<<", result = "<<result<< std::endl;
    cb(result);
}

void simple_add_two_numbers1(int a, int b, const rust::Fn<void(std::int32_t)> cb)
{
    hello();
    int result = a + b;
    std::cout<<"calling in C1, a = "<<a<<", b = "<<b<<", result = "<<result<< std::endl;
    cb(result);
    simple_add_two_numbers2(a, b, cppcb);
    simple_add_two_numbers3(a, b, cb);
}


void better_add_two_numbers(int a, int b, const rust::Fn<void(std::int32_t, Counter &)> cb, Counter & counter)
{
    std::int32_t result = a + b;
    std::cout<<"calling in C, a = "<<a<<", b = "<<b<<", result = "<<result<< std::endl;
    //counter.add_result(result);
    cb(result, counter);
}

void better_add_two_numbers2(int a, int b, const rust::Fn<void(std::int32_t, c_void*)> cb, c_void* data)
{
    std::int32_t result = a + b;
    std::cout<<"calling in C, a = "<<a<<", b = "<<b<<", result = "<<result<< std::endl;
    cb(result, data);
}


void void_ptr(void * data){
}

}