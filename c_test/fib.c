
#include "debug.h"

int fib(int n);

int main(void) {
    int i = fib(14);

    // print value of i
    rprintf("Value: &d\n", i);

    return 0;
}

int fib(int n) {
    if (n == 0) {
        return 0;
    } else if (n == 1) {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
