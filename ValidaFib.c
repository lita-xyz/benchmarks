int main () {
  int n = 25;
  int a = 0;
  int b = 1;
  int sum;
  unsigned i;
  for (i = 1; i < n; i++) {
    sum = a + b;
    a = b;
    b = sum;
    };
  __builtin_delendum_write(a);
  __builtin_delendum_write(b);
  return 0;
  }