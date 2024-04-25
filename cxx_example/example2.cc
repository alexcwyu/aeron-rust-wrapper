#include "aeron-rust-wrapper/cxx_wrapper/example.h"
#include "aeron-rust-wrapper/cxx_wrapper/example2.h"
#include <iostream>


namespace example2{
Object2::Object2(const std::string & name):
m_name(name),
m_obj1(example::Object(name+"-obj1")),
m_obj2(example::Object(name+"-obj2"))
{ std::cout << "construct Object2-"<<m_name<< std::endl; }

//Object2::Object2(const std::string & name){
//m_name = name;
//m_obj1 = example::Object(name+"-obj1");
//m_obj2 = example::Object(name+"-obj2");
//std::cout << "construct Object2-"<<m_name<< std::endl;
//}


Object2::~Object2() { std::cout << "~Object2-"<<m_name << std::endl; }

void Object2::sayHi() const { std::cout << "Hi from Object2-"<<m_name << std::endl; }

//std::shared_ptr<Object2> createShared(const std::string & name) {
//  return std::make_shared<Object2>(Object2(name));
//}
//std::unique_ptr<Object2> createUnique(const std::string & name) {
//  return std::make_unique<Object2>(Object2(name));
//}

std::shared_ptr<Object2> createShared(const std::string & name) {
  return std::shared_ptr<Object2>(new Object2(name));
}
std::unique_ptr<Object2> createUnique(const std::string & name) {
  return std::unique_ptr<Object2>(new Object2(name));
}
}