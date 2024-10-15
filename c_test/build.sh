#!/bin/bash

prefix ()
{
  "riscv32-unknown-elf-$@"
}

IncludeFlag="-I."
CFlags="-march=rv32ima -nostdlib -O3"

# get the filename without the extension
C_FILE=$1
FNAME=$(basename $1 .c)
ASM=$FNAME.s

prefix gcc $CFlags $IncludeFlag -S $C_FILE debug.c
prefix gcc $CFlags $IncludeFlag -flto -Wl,-Ttext=0x80000000 -o $FNAME $ASM debug.s
prefix objdump -M no-aliases -SD $FNAME > $FNAME.asm
prefix objcopy -O binary $FNAME $FNAME.bin

rm $ASM debug.s
rm $FNAME
