
void putstr(volatile char* uart, const char* str);

int main() {
    // uart is at 0x10000000 and is the UART device which allows us to print to the console
    volatile char* uart = (volatile char*)0x10000000;
    const char* hello = "Hello, world!\n";
    putstr(uart, hello);
}

void putstr(volatile char* uart, const char* str) {
    for (int i = 0; str[i] != '\0'; i++) {
        uart[0] = str[i];
    }
}
