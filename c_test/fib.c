
int fib(int n);
// custom putchar function to print to the console
void cputchar(char c);

int main(void) {
    int i = fib(2);

    cputchar('0' + i);
    cputchar('\n');

    return 0;
}

int fib(int n) {
    if (n > 1) {
        return fib(n - 1);
    } else {
        return n;
    }
}

// uart is at 0x10000000 and is the UART device which allows us to print to the console
static volatile char* uart = (volatile char*) 0x10000000;

void cputchar(char c) {
    uart[0] = c;
}
