	.file	"fib.c"
	.option nopic
	.attribute arch, "rv32i2p0_m2p0_a2p0"
	.attribute unaligned_access, 0
	.attribute stack_align, 16
	.text
	.align	2
	.globl	fib
	.type	fib, @function
fib:
	addi	sp,sp,-128
	sw	s2,112(sp)
	sw	ra,124(sp)
	sw	s0,120(sp)
	sw	s1,116(sp)
	sw	s3,108(sp)
	sw	s4,104(sp)
	sw	s5,100(sp)
	sw	s6,96(sp)
	sw	s7,92(sp)
	sw	s8,88(sp)
	sw	s9,84(sp)
	sw	s10,80(sp)
	sw	s11,76(sp)
	mv	s2,a0
	beq	a0,zero,.L54
	li	a5,1
	beq	a0,a5,.L54
	addi	a5,a0,-1
	li	s2,0
	sw	a5,4(sp)
	addi	a2,a0,-2
	li	s0,1
	mv	t2,s2
.L3:
	beq	a5,zero,.L4
	beq	a5,s0,.L5
	addi	a3,a2,-1
	mv	t4,a2
	li	s10,0
	sw	a2,8(sp)
	mv	s11,a3
	sw	t2,12(sp)
.L6:
	beq	t4,zero,.L7
	beq	t4,s0,.L9
	addi	a1,s11,-1
	sw	s10,16(sp)
	li	s9,0
	sw	s11,20(sp)
	sw	t4,24(sp)
	mv	s10,a1
.L10:
	beq	s11,zero,.L11
	beq	s11,s0,.L13
	mv	t3,s10
	addi	a2,s10,-1
	li	a5,0
	sw	s9,28(sp)
	sw	s10,32(sp)
	sw	s11,36(sp)
	mv	s9,a5
	mv	s10,a2
	mv	s11,t3
.L14:
	beq	s11,zero,.L15
	beq	s11,s0,.L17
	mv	t1,s10
	addi	a1,s10,-1
	li	a4,0
	sw	s9,40(sp)
	sw	s10,44(sp)
	sw	s11,48(sp)
	mv	s9,a4
	mv	s10,a1
	mv	s11,t1
.L18:
	beq	s11,zero,.L19
	beq	s11,s0,.L21
	li	s4,0
	mv	a3,s11
	addi	s3,s10,-1
	mv	s11,s4
	mv	a4,s10
	mv	s4,s10
	mv	a5,s9
.L22:
	beq	s4,zero,.L23
	beq	s4,s0,.L25
	addi	s2,s3,-1
	mv	s5,s3
	li	s1,0
.L26:
	beq	s5,zero,.L27
	beq	s5,s0,.L29
	mv	a2,s5
	addi	s10,s2,-1
	mv	s9,s2
	mv	s5,s3
	li	s6,0
	mv	s3,a4
	mv	a4,s2
	mv	s2,s1
	mv	s1,a5
	mv	a5,s4
	mv	s4,a3
	mv	a3,a2
.L30:
	mv	s8,s9
	beq	s9,zero,.L31
	beq	s9,s0,.L33
	sw	s10,52(sp)
	sw	s9,56(sp)
	li	s7,0
	mv	s10,a4
	mv	s9,a5
.L34:
	addi	a0,s8,-1
	sw	a3,60(sp)
	addi	s8,s8,-2
	call	fib
	lw	a3,60(sp)
	add	s7,s7,a0
	beq	s8,zero,.L76
	bne	s8,s0,.L34
	mv	a5,s9
	mv	a4,s10
	lw	s9,56(sp)
	lw	s10,52(sp)
	addi	s7,s7,1
.L35:
	add	s6,s6,s7
	beq	s10,zero,.L77
	bne	s10,s0,.L31
.L33:
	mv	a2,a3
	addi	s6,s6,1
	mv	a3,s4
	mv	s4,a5
	mv	a5,s1
	mv	s1,s2
	mv	s2,a4
	mv	a4,s3
	mv	s3,s5
	mv	s5,a2
.L32:
	add	s1,s1,s6
	beq	s2,zero,.L28
	bne	s2,s0,.L27
.L29:
	addi	s1,s1,1
.L28:
	add	s11,s11,s1
	bne	s3,zero,.L39
	mv	s4,s11
	mv	s9,a5
	mv	s10,a4
	mv	s11,a3
	add	s9,s9,s4
	bne	s10,zero,.L40
.L80:
	mv	a4,s9
	lw	s10,44(sp)
	lw	s9,40(sp)
	lw	s11,48(sp)
	add	s9,s9,a4
	beq	s10,zero,.L78
.L41:
	beq	s10,s0,.L17
.L15:
	addi	s11,s11,-2
	addi	s10,s10,-2
	j	.L14
.L9:
	lw	a2,8(sp)
	lw	t2,12(sp)
	addi	s10,s10,1
.L8:
	add	t2,t2,s10
	beq	a2,zero,.L79
	bne	a2,s0,.L4
.L5:
	addi	s2,t2,1
.L54:
	lw	ra,124(sp)
	lw	s0,120(sp)
	lw	s1,116(sp)
	lw	s3,108(sp)
	lw	s4,104(sp)
	lw	s5,100(sp)
	lw	s6,96(sp)
	lw	s7,92(sp)
	lw	s8,88(sp)
	lw	s9,84(sp)
	lw	s10,80(sp)
	lw	s11,76(sp)
	mv	a0,s2
	lw	s2,112(sp)
	addi	sp,sp,128
	jr	ra
.L27:
	addi	s5,s5,-2
	addi	s2,s2,-2
	j	.L26
.L39:
	bne	s3,s0,.L23
.L25:
	mv	s4,s11
	mv	s9,a5
	addi	s4,s4,1
	mv	s10,a4
	mv	s11,a3
	add	s9,s9,s4
	beq	s10,zero,.L80
.L40:
	beq	s10,s0,.L21
.L19:
	addi	s11,s11,-2
	addi	s10,s10,-2
	j	.L18
.L23:
	addi	s4,s4,-2
	addi	s3,s3,-2
	j	.L22
.L77:
	mv	a2,a3
	mv	a3,s4
	mv	s4,a5
	mv	a5,s1
	mv	s1,s2
	mv	s2,a4
	mv	a4,s3
	mv	s3,s5
	mv	s5,a2
	j	.L32
.L76:
	mv	a5,s9
	mv	a4,s10
	lw	s9,56(sp)
	lw	s10,52(sp)
	j	.L35
.L21:
	mv	a4,s9
	lw	s10,44(sp)
	lw	s9,40(sp)
	addi	a4,a4,1
	lw	s11,48(sp)
	add	s9,s9,a4
	bne	s10,zero,.L41
.L78:
	mv	a5,s9
	lw	s10,32(sp)
	lw	s9,28(sp)
	lw	s11,36(sp)
	add	s9,s9,a5
	bne	s10,zero,.L42
.L81:
	lw	s10,16(sp)
	lw	s11,20(sp)
	lw	t4,24(sp)
	add	s10,s10,s9
	bne	s11,zero,.L43
.L82:
	lw	a2,8(sp)
	lw	t2,12(sp)
	j	.L8
.L31:
	addi	s9,s9,-2
	addi	s10,s10,-2
	j	.L30
.L17:
	mv	a5,s9
	lw	s10,32(sp)
	lw	s9,28(sp)
	addi	a5,a5,1
	lw	s11,36(sp)
	add	s9,s9,a5
	beq	s10,zero,.L81
.L42:
	beq	s10,s0,.L13
.L11:
	addi	s11,s11,-2
	addi	s10,s10,-2
	j	.L10
.L13:
	lw	s10,16(sp)
	lw	s11,20(sp)
	addi	s9,s9,1
	lw	t4,24(sp)
	add	s10,s10,s9
	beq	s11,zero,.L82
.L43:
	beq	s11,s0,.L9
.L7:
	addi	t4,t4,-2
	addi	s11,s11,-2
	j	.L6
.L4:
	lw	a5,4(sp)
	addi	a2,a2,-2
	addi	a5,a5,-2
	sw	a5,4(sp)
	j	.L3
.L79:
	mv	s2,t2
	j	.L54
	.size	fib, .-fib
	.section	.rodata.str1.4,"aMS",@progbits,1
	.align	2
.LC0:
	.string	"ERROR: number too large"
	.section	.text.startup,"ax",@progbits
	.align	2
	.globl	main
	.type	main, @function
main:
	addi	sp,sp,-16
	li	a0,14
	sw	ra,12(sp)
	call	fib
	li	a5,1000001536
	addi	a5,a5,-1537
	bgt	a0,a5,.L88
	lui	a5,%hi(.LANCHOR0)
	addi	a5,a5,%lo(.LANCHOR0)
	sb	zero,9(a5)
	ble	a0,zero,.L85
	li	a3,10
	rem	a4,a0,a3
	li	a2,9
	addi	a4,a4,48
	sb	a4,8(a5)
	div	a4,a0,a3
	ble	a0,a2,.L89
	li	a1,99
	rem	a2,a4,a3
	addi	a2,a2,48
	sb	a2,7(a5)
	div	a4,a4,a3
	ble	a0,a1,.L90
	li	a1,999
	rem	a2,a4,a3
	addi	a2,a2,48
	sb	a2,6(a5)
	div	a4,a4,a3
	ble	a0,a1,.L91
	li	a2,8192
	addi	a2,a2,1807
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,5(a5)
	div	a4,a4,a3
	ble	a0,a2,.L92
	li	a2,98304
	addi	a2,a2,1695
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,4(a5)
	div	a4,a4,a3
	ble	a0,a2,.L93
	li	a2,999424
	addi	a2,a2,575
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,3(a5)
	div	a4,a4,a3
	ble	a0,a2,.L94
	li	a2,9998336
	addi	a2,a2,1663
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,2(a5)
	div	a4,a4,a3
	ble	a0,a2,.L95
	li	a2,99999744
	addi	a2,a2,255
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,1(a5)
	div	a4,a4,a3
	ble	a0,a2,.L96
	addi	a4,a4,48
	sb	a4,0(a5)
	li	a4,-1
.L86:
	addi	a4,a4,1
	add	a5,a5,a4
	lbu	a4,0(a5)
.L84:
	li	a3,268435456
.L87:
	sb	a4,0(a3)
	lbu	a4,1(a5)
	addi	a5,a5,1
	bne	a4,zero,.L87
.L85:
	li	a5,268435456
	li	a4,10
	sb	a4,0(a5)
	lw	ra,12(sp)
	li	a0,0
	addi	sp,sp,16
	jr	ra
.L88:
	lui	a5,%hi(.LC0)
	addi	a5,a5,%lo(.LC0)
	li	a4,69
	j	.L84
.L90:
	li	a4,6
	j	.L86
.L89:
	li	a4,7
	j	.L86
.L91:
	li	a4,5
	j	.L86
.L92:
	li	a4,4
	j	.L86
.L93:
	li	a4,3
	j	.L86
.L94:
	li	a4,2
	j	.L86
.L95:
	li	a4,1
	j	.L86
.L96:
	li	a4,0
	j	.L86
	.size	main, .-main
	.text
	.align	2
	.globl	num2str
	.type	num2str, @function
num2str:
	li	a5,1000001536
	addi	a5,a5,-1537
	bgt	a0,a5,.L104
	lui	a5,%hi(.LANCHOR0)
	addi	a5,a5,%lo(.LANCHOR0)
	sb	zero,9(a5)
	ble	a0,zero,.L105
	li	a3,10
	rem	a4,a0,a3
	li	a2,9
	addi	a4,a4,48
	sb	a4,8(a5)
	div	a4,a0,a3
	ble	a0,a2,.L106
	li	a1,99
	rem	a2,a4,a3
	addi	a2,a2,48
	sb	a2,7(a5)
	div	a4,a4,a3
	ble	a0,a1,.L107
	li	a1,999
	rem	a2,a4,a3
	addi	a2,a2,48
	sb	a2,6(a5)
	div	a4,a4,a3
	ble	a0,a1,.L108
	li	a2,8192
	addi	a2,a2,1807
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,5(a5)
	div	a4,a4,a3
	ble	a0,a2,.L109
	li	a2,98304
	addi	a2,a2,1695
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,4(a5)
	div	a4,a4,a3
	ble	a0,a2,.L110
	li	a2,999424
	addi	a2,a2,575
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,3(a5)
	div	a4,a4,a3
	ble	a0,a2,.L111
	li	a2,9998336
	addi	a2,a2,1663
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,2(a5)
	div	a4,a4,a3
	ble	a0,a2,.L112
	li	a2,99999744
	addi	a2,a2,255
	rem	a1,a4,a3
	addi	a1,a1,48
	sb	a1,1(a5)
	div	a4,a4,a3
	ble	a0,a2,.L113
	addi	a4,a4,48
	sb	a4,0(a5)
	li	a4,0
.L102:
	add	a0,a5,a4
	ret
.L104:
	lui	a5,%hi(.LC0)
	addi	a0,a5,%lo(.LC0)
	ret
.L107:
	li	a4,7
	j	.L102
.L105:
	li	a4,9
	j	.L102
.L106:
	li	a4,8
	j	.L102
.L108:
	li	a4,6
	j	.L102
.L109:
	li	a4,5
	j	.L102
.L110:
	li	a4,4
	j	.L102
.L111:
	li	a4,3
	j	.L102
.L112:
	li	a4,2
	j	.L102
.L113:
	li	a4,1
	j	.L102
	.size	num2str, .-num2str
	.align	2
	.globl	cputchar
	.type	cputchar, @function
cputchar:
	li	a5,268435456
	sb	a0,0(a5)
	ret
	.size	cputchar, .-cputchar
	.bss
	.align	2
	.set	.LANCHOR0,. + 0
	.type	out.0, @object
	.size	out.0, 10
out.0:
	.zero	10
	.ident	"GCC: (GNU) 11.1.0"
