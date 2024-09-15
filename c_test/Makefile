
%.s:
	riscv32-unknown-linux-gnu-gcc -march="rv32i" -S test.c

test: test.s
	riscv32-unknown-linux-gnu-gcc -Wl,-Ttext=0x0 -nostdlib -o test test.s

test.bin: test
	riscv32-unknown-linux-gnu-objcopy -O binary test test.bin

clean:
	rm -f test test.s test.bin

.PHONY: clean test.bin
