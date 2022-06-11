#include <cmath>
#include <iostream>
template <typename Number> Number quartic_root(Number x) {
  return sqrt(sqrt(x));
}
int main() { std::cout << quartic_root("Hello"); }
