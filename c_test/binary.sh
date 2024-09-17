#! /bin/bash

riscv32-unknown-linux-gnu-gcc -S -nostdlib -march="rv32id" test.c
riscv32-unknown-linux-gnu-gcc -march="rv32id" -Wl,-Ttext=0x80000000 -nostdlib -o test test.s
riscv32-unknown-linux-gnu-objcopy -O binary test test.bin
