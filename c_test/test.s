	.file	"test.c"
	.option nopic
	.text
	.align	2
	.globl	main
	.type	main, @function
main:
.LFB0:
	addi	sp,sp,-16
.LCFI0:
	sw	s0,12(sp)
.LCFI1:
	addi	s0,sp,16
.LCFI2:
	li	a5,31
	mv	a0,a5
	lw	s0,12(sp)
.LCFI3:
	addi	sp,sp,16
.LCFI4:
	jr	ra
.LFE0:
	.size	main, .-main
	.section	.eh_frame,"aw",@progbits
.Lframe1:
	.4byte	.LECIE1-.LSCIE1
.LSCIE1:
	.4byte	0
	.byte	0x3
	.string	"zR"
	.byte	0x1
	.byte	0x7c
	.byte	0x1
	.byte	0x1
	.byte	0x1b
	.byte	0xc
	.byte	0x2
	.byte	0
	.align	2
.LECIE1:
.LSFDE1:
	.4byte	.LEFDE1-.LASFDE1
.LASFDE1:
	.4byte	.LASFDE1-.Lframe1
	.4byte	.LFB0-.
	.4byte	.LFE0-.LFB0
	.byte	0
	.byte	0x4
	.4byte	.LCFI0-.LFB0
	.byte	0xe
	.byte	0x10
	.byte	0x4
	.4byte	.LCFI1-.LCFI0
	.byte	0x88
	.byte	0x1
	.byte	0x4
	.4byte	.LCFI2-.LCFI1
	.byte	0xc
	.byte	0x8
	.byte	0
	.byte	0x4
	.4byte	.LCFI3-.LCFI2
	.byte	0xc8
	.byte	0xc
	.byte	0x2
	.byte	0x10
	.byte	0x4
	.4byte	.LCFI4-.LCFI3
	.byte	0xe
	.byte	0
	.align	2
.LEFDE1:
	.ident	"GCC: (GNU) 13.2.0"
	.section	.note.GNU-stack,"",@progbits
