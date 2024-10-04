
int fib(int n);
// custom putchar function to print to the console
void cputchar(char c);
// convert a number to a string
const char* num2str(int n);

int main(void) {
    int i = fib(14);

    const char* str = num2str(i);
    while (*str) {
        cputchar(*str);
        str++;
    }
    cputchar('\n');

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

const char* num2str(int n) {
    static char out[10]; // increased size to 10 to accommodate null terminator
    const int MAX_DIGIT = 999999999;
    if (n > MAX_DIGIT) {
        return "ERROR: number too large";
    }

    int i = 8; // start from the end of the buffer
    out[9] = '\0'; // null terminator
    while (n > 0) {
        out[i] = '0' + (n % 10);
        n /= 10;
        i--;
    }

    return &out[i + 1]; // return the pointer to the start of the number
}

// uart is at 0x10000000 and is the UART device which allows us to print to the console
static volatile char* uart = (volatile char*) 0x10000000;

void cputchar(char c) {
    uart[0] = c;
}
