#!/bin/bash

prefix ()
{
  "riscv32-unknown-elf-$@"
}

IncludeFlag="-I."
DEBUG_IMPL="debug.c"
CFlags="-march=rv32ima -nostdlib -O3"

prefix gcc $CFlags $IncludeFlag -S $1.c $DEBUG_IMPL
prefix gcc $CFlags $IncludeFlag -Wl,-Ttext=0x80000000 -o $1 *.s
prefix objcopy -O binary $1 $1.bin

rm $1.s
rm $1
