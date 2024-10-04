#!/bin/bash

prefix ()
{
  "riscv32-unknown-elf-$@"
}

prefix gcc -O3 -march="rv32ima" -S -nostdlib $1.c
prefix gcc -O3 -march="rv32ima" -nostdlib -Wl,-Ttext=0x80000000 -o $1 $1.s
prefix objcopy -O binary $1 $1.bin

# rm $1.s
rm $1
