// This contains the debug functions for the C test programs
// and for the asm test programs

#ifndef DEBUG_H
#define DEBUG_H

#define null ((void*)0)

#define true 1
#define false 0

#define UART(offset) (*(volatile char*)(0x10000000 + (offset)))

#define ATTRIBUTES __attribute__((always_inline))
#define DEBUG_FN static inline
#define FN_DEFINE(...) \
    DEBUG_FN __VA_ARGS__ ATTRIBUTES; \
    DEBUG_FN __VA_ARGS__

// used to print a single character to the UART
FN_DEFINE(void printc(char c)) {
    UART(0) = c;
}

// used to print a string to the UART
FN_DEFINE(void print(const char* s)) {
    for (int i = 0; s[i]; i++) {
        printc(s[i]);
    }
}

// used to print a binary integer to the UART
FN_DEFINE(void print_bin(int i)) {
    print("0b");
    for (int j = 31; j >= 0; j--) {
        printc((i & (1 << j)) ? '1' : '0');
    }
}

// used to print a boolean to the UART
FN_DEFINE(void print_bool(int b)) {
    print(b ? "true" : "false");
}

// used to print a hex integer to the UART
FN_DEFINE(void print_hex(unsigned int i)) {
    print("0x");
    for (int j = 7; j >= 0; j--) {
        int nibble = (i >> (j * 4)) & 0xF;
        printc(nibble < 10 ? '0' + nibble : 'A' + nibble - 10);
    }
}

// used to print a pointer to the UART
FN_DEFINE(void print_ptr(void* p)) {
    // disables clangs error "bad_reinterpret_cast_small_int"
    print_hex((unsigned int)(unsigned long long)p);
}

// used to print an integer to the UART
FN_DEFINE(void print_int(int i)) {
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

// used to print a float to the UART
// FN_DEFINE(void print_float(float f);
// used to print a double to the UART
// FN_DEFINE(void print_double(double d);

// used to exit the program with an exit code
DEBUG_FN void exit(int code) __attribute__((noreturn)) ATTRIBUTES;
DEBUG_FN void exit(int code) {
    // since this is naked we need an asm equivalent to
    print("Exit: "); print_int(code); printc('\n');
    // load the code into t0
    asm volatile (
        "mv t0, %0"
        :
        : "r" (code)
        : "t0"
    );
    // call the crash_exit function
    // write the code (t0) to 0x00000000
    asm volatile ("sw t0, 0(zero)");
}

// used to exit the program with an error message if something goes wrong
// just uses exit and print under the hood
DEBUG_FN void panic(const char *s) __attribute__((noreturn)) ATTRIBUTES;
DEBUG_FN void panic(const char *s) {
    print("Panicing!\n");
    print(s);
    exit(1);
}

// used to mimic the behavior of the C function printf
void printf(const char* fmt, ...);

#define assert(cond) if (!(cond)) { \
    printf("Assertion failed: %s\n", #cond); \
    exit(1); \
}
#define assert_eq(a, b) if ((a) != (b)) { \
    printf("Assertion failed: %s != %s\n", #a, #b); \
    exit(1); \
}

#endif