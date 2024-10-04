#!/bin/bash

prefix ()
{
  "riscv32-unknown-elf-$@"
}

prefix gcc -O0 -march="rv32ima" -nostdlib -Wl,-Ttext=0x80000000 -o $1 $1.S
prefix objcopy -O binary $1 $1.bin

rm $1
