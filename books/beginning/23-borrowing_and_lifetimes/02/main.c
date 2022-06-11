#include <stdio.h>
int main() {
  int *ref_to_n;
  {
    int n = 12;
    ref_to_n = &n;
  }
  printf("%d", *ref_to_n);
  return 0;
}
