#!/bin/bash

riscv32-unknown-linux-gnu-gcc -O3 -march="rv32i" -S -nostdlib $1.c
riscv32-unknown-linux-gnu-gcc -O3 -march="rv32i" -Wl,-Ttext=0x80000000 -nostdlib -o $1 $1.s
riscv32-unknown-linux-gnu-objcopy -O binary $1 $1.bin

rm $1.s
rm $1
