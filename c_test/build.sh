#! /bin/bash

riscv32-unknown-linux-gnu-gcc -S -nostdlib -march="rv32imad" $1.c
riscv32-unknown-linux-gnu-gcc -march="rv32imad" -Wl,-Ttext=0x80000000 -nostdlib -o $1 $1.s
riscv32-unknown-linux-gnu-objcopy -O binary $1 $1.bin

rm $1.s $1
