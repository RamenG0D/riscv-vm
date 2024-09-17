
int main(void) {
    *(char*)0x80000000 = 10;
    return 31;
}
