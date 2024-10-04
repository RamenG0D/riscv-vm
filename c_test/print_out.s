
main:
    li t1, 0x10000000
    # write 'H' (72) to address 0x10000000 (uart address, allows us to print to console)
    li t0, 72
    sb t0, 0(t1)
    li t0, 10
    sb t0, 0(t1)