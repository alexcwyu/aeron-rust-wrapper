#pragma once
#include <memory>
#include <string>

namespace example{
class Object {
public:
  Object(const std::string & name);
  ~Object();
  void sayHi() const;
private:
  std::string m_name;
};

std::shared_ptr<Object> createShared(const std::string & name);
std::unique_ptr<Object> createUnique(const std::string & name);
}