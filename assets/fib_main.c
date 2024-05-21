int fib_ref(int n) {
    if (n == 1) return 1;
    if (n == 2) return 1;
    return fib_ref(n - 1) + fib_ref(n - 2);
}

extern int fib(int);

#include <stdio.h>

int main() {
    int n;
    printf("Calculate fibonacci sequence to: ");
    scanf("%d", &n);
    printf("Reference Fib(%d) = %d\n", n, fib_ref(n));
    printf("rcc Fib(%d) = %d\n", n, fib(n));
    return 0;
}
