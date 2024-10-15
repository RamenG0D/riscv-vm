
#include "debug.h"
#include <stdarg.h>

// used to mimic the behavior of the C function printf
void printf(const char* fmt, ...) {
    va_list args;
    va_start(args, fmt);
    for (int i = 0; fmt[i]; i++) {
        if (fmt[i] == '%') {
            i++;
            switch (fmt[i]) {
                case 'c': {
                    char c = va_arg(args, int);
                    printc(c);
                    break;
                }
                case 's': {
                    const char* s = va_arg(args, const char*);
                    print(s);
                    break;
                }
                case 'd': {
                    int d = va_arg(args, int);
                    print_int(d);
                    break;
                }
                case 'x': {
                    int x = va_arg(args, int);
                    print_hex(x);
                    break;
                }
                case 'p': {
                    void* p = va_arg(args, void*);
                    print_ptr(p);
                    break;
                }
                case 'b': {
                    int b = va_arg(args, int);
                    print_bool(b);
                    break;
                }
                default:
                    print("Unknown format specifier: %");
                    printc(fmt[i]);
                    break;
            }
        } else {
            printc(fmt[i]);
        }
    }
    va_end(args);
}
