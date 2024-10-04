	.file	"hello.c"
	.option nopic
	.attribute arch, "rv32i2p0_m2p0_a2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.section	.rodata
	.align	2
.LC0:
	.string	"Hello, World! From RSICV!\n"
	.text
	.align	2
	.globl	main
	.type	main, @function
main:
	addi	sp,sp,-16
	sw	ra,12(sp)
	sw	s0,8(sp)
	addi	s0,sp,16
	lui	a5,%hi(.LC0)
	addi	a0,a5,%lo(.LC0)
	call	putstr
	li	a5,0
	mv	a0,a5
	lw	ra,12(sp)
	lw	s0,8(sp)
	addi	sp,sp,16
	jr	ra
	.size	main, .-main
	.section	.sdata,"aw"
	.align	2
	.type	uart, @object
	.size	uart, 4
uart:
	.word	268435456
	.text
	.align	2
	.globl	strlenc
	.type	strlenc, @function
strlenc:
	addi	sp,sp,-48
	sw	s0,44(sp)
	addi	s0,sp,48
	sw	a0,-36(s0)
	sw	zero,-20(s0)
	j	.L4
.L5:
	lw	a5,-20(s0)
	addi	a5,a5,1
	sw	a5,-20(s0)
.L4:
	lw	a5,-20(s0)
	lw	a4,-36(s0)
	add	a5,a4,a5
	lbu	a5,0(a5)
	bne	a5,zero,.L5
	lw	a5,-20(s0)
	mv	a0,a5
	lw	s0,44(sp)
	addi	sp,sp,48
	jr	ra
	.size	strlenc, .-strlenc
	.align	2
	.globl	putstr
	.type	putstr, @function
putstr:
	addi	sp,sp,-32
	sw	s0,28(sp)
	addi	s0,sp,32
	sw	a0,-20(s0)
	j	.L8
.L9:
	lw	a5,-20(s0)
	addi	a4,a5,1
	sw	a4,-20(s0)
	lui	a4,%hi(uart)
	lw	a4,%lo(uart)(a4)
	lbu	a5,0(a5)
	sb	a5,0(a4)
.L8:
	lw	a5,-20(s0)
	lbu	a5,0(a5)
	bne	a5,zero,.L9
	nop
	nop
	lw	s0,28(sp)
	addi	sp,sp,32
	jr	ra
	.size	putstr, .-putstr
	.ident	"GCC: (GNU) 11.1.0"
