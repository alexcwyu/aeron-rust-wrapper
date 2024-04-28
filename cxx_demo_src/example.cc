#include "aeron-rust-wrapper/cxx_demo_include/example.h"
#include <iostream>


namespace example{
//Object::Object(const std::string & name): m_name(name) {
//std::cout << "construct Object-"<<m_name << std::endl;
//}

Object::Object(const std::string & name){
m_name=name;
std::cout << "construct Object-"<<m_name << std::endl;
}

Object::~Object() { std::cout << "~Object-"<<m_name  << std::endl; }

void Object::sayHi() const { std::cout << "Hi from Object-"<<m_name  << std::endl; }
void Object::sayHi2() { std::cout << "Hi2 from Object-"<<m_name  << std::endl; }

//std::shared_ptr<Object> createShared(const std::string & name) {
//  return std::make_shared<Object>(Object(name));
//}
//
//std::unique_ptr<Object> createUnique(const std::string & name) {
//  return std::make_unique<Object>(Object(name));
//}

std::shared_ptr<Object> createShared(const std::string & name) {
  return std::shared_ptr<Object>(new Object(name));
}

std::unique_ptr<Object> createUnique(const std::string & name) {
  return std::unique_ptr<Object>(new Object(name));
}
}