
#include "debug.h"
#include <stdarg.h>

void rprintf(const char* format, ...) {
    // basic printf implementation but instead of using % to format, we use an &
    va_list args;
    va_start(args, format);

    while (*format) {
        if (*format == '&') {
            switch (*(++format)) {
                case 'd': {
                    int i = va_arg(args, int);
                    print_int(i);
                    break;
                }
                case 'x': {
                    int i = va_arg(args, int);
                    print_hex(i);
                    break;
                }
                case 'b': {
                    int i = va_arg(args, int);
                    print_bin(i);
                    break;
                }
                case 'p': {
                    void* p = va_arg(args, void*);
                    print_ptr(p);
                    break;
                }
                case 's': {
                    const char* s = va_arg(args, const char*);
                    print(s);
                    break;
                }
                case 'c': {
                    char c = va_arg(args, int);
                    printc(c);
                    break;
                }
                default: {
                    print("Unknown format {");
                    printc(*format);
                    print("}\n");
                    break;
                }
            }
        } else {
            printc(*format);
        }
        format++;
    }
}

void print(const char* s) {
    while (*s) {
        printc(*s);
        s++;
    }
}

void printc(char c) {
    UART(0) = c;
}

void print_bin(int i) {
    print("0b");
    for (int j = 31; j >= 0; j--) {
        printc((i & (1 << j)) ? '1' : '0');
    }
}

void print_bool(int b) {
    print(b ? "true" : "false");
}

void print_hex(int i) {
    print("0x");
    for (int j = 7; j >= 0; j--) {
        int nibble = (i >> (j * 4)) & 0xF;
        printc(nibble < 10 ? '0' + nibble : 'A' + nibble - 10);
    }
}

void print_ptr(void* p) {
    print_hex((int)p);
}

void print_int(int i) {
    if (i < 0) {
        print("-");
        i = -i;
    }
    if (i == 0) {
        print("0");
        return;
    }
    char buf[10];
    int j = 0;
    while (i) {
        buf[j++] = '0' + (i % 10);
        i /= 10;
    }
    while (j) {
        printc(buf[--j]);
    }
}

void crash_exit(void) __attribute__((noreturn, naked));

// THIS function is NAKED (asmeblly only) so everything inside must be done in assembly
void crash_exit(/* assume "int code" is stored in t0 */) {
    // get the code from t0
    asm volatile ("mv a0, t0":::"a0");
    // write the code (a0) to 0x00000000
    asm volatile ("sw a0, 0(zero)");
}

void exit(register int code) {
    // since this is naked we need an asm equivalent to
    print("Exit: "); print_int(code); print("\n");
    // load the code into t0
    asm volatile (
        "mv t0, %0"
        :
        : "r" (code)
        : "t0"
    );
    // call the crash_exit function
    crash_exit();
}
void panic(const char *s) {
    print("Panic: "); print(s); print("\n");
    exit(1);
}
