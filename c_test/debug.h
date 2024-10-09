// This contains the debug functions for the C test programs
// and for the asm test programs

#ifndef DEBUG_H
#define DEBUG_H

#define null ((void*)0)

#define true 1
#define false 0

#define UART(offset) (*(volatile unsigned char*)(0x10000000 + offset))

// used to print a string to the UART
void print(const char* s);
// used to print a single character to the UART
void printc(char c);
// used to print an integer to the UART
void print_int(int i);
// used to print a hex integer to the UART
void print_hex(int i);
// used to print a binary integer to the UART
void print_bin(int i);
// used to print a float to the UART
// void print_float(float f);
// used to print a double to the UART
// void print_double(double d);
// used to print a pointer to the UART
void print_ptr(void* p);
// used to print a boolean to the UART
void print_bool(int b);
// overall format print function
void rprintf(const char* format, ...);

// used to exit the program with an error message if something goes wrong
// just uses exit and print under the hood
void panic(const char *s) __attribute__((noreturn));
// used to exit the program with an exit code
void exit(int code) __attribute__((noreturn));


#define assert(cond) do { if (!(cond)) panic("Assertion failed: " ## cond ## "\n"); } while (0)
#define assert_eq(a, b) do { if ((a) != (b)) panic("Assertion failed: " ## a ## " != " ## b ## "\n"); } while (0)

#endif