#pragma once
#include <memory>
#include <string>
#include "example.h"


namespace example2{
class Object2 {
public:
  Object2(const std::string & name);
  ~Object2();
  void sayHi() const;
  void sayHi2();

  std::shared_ptr<example::Object> getObj1() const{
     return std::make_shared<example::Object>(m_obj1);
  }


  std::shared_ptr<example::Object> getObj1Mut(){
     return std::make_shared<example::Object>(m_obj1);
  }


  std::unique_ptr<example::Object> getObj2() const{
     return std::make_unique<example::Object>(m_obj2);
  }

  std::unique_ptr<example::Object> getObj2Mut(){
     return std::make_unique<example::Object>(m_obj2);
  }

private:
std::string m_name;
    example::Object m_obj1;
    example::Object m_obj2;
};

std::shared_ptr<Object2> createShared(const std::string & name);
std::unique_ptr<Object2> createUnique(const std::string & name);
}