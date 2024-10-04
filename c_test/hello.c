
void putstr(const char* str);

int main(void) {
    putstr("Hello, World! From RSICV!\n");

    return 0;
}

// uart is at 0x10000000 and is the UART device which allows us to print to the console
static volatile char* uart = (volatile char*)0x10000000;

int strlenc(const char* const str) {
    int len;
    for(len = 0; str[len] != '\0'; len++)
        ;
    return len;
}

void putstr(const char* str) {
    while(*str != '\0') *uart = *str++;
}
