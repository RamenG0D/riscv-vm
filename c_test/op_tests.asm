
op_tests:     file format elf32-littleriscv


Disassembly of section .text:

80000000 <main>:
80000000:	ff010113          	addi	sp,sp,-16
80000004:	00112623          	sw	ra,12(sp)
80000008:	248000ef          	jal	ra,80000250 <test_bin_ops>
8000000c:	1d4000ef          	jal	ra,800001e0 <test_un_ops>
80000010:	014000ef          	jal	ra,80000024 <test_csrs>
80000014:	00c12083          	lw	ra,12(sp)
80000018:	00000513          	addi	a0,zero,0
8000001c:	01010113          	addi	sp,sp,16
80000020:	00008067          	jalr	zero,0(ra)

80000024 <test_csrs>:
80000024:	80001537          	lui	a0,0x80001
80000028:	fe010113          	addi	sp,sp,-32
8000002c:	8c450513          	addi	a0,a0,-1852 # 800008c4 <__global_pointer$+0xffffe614>
80000030:	00112e23          	sw	ra,28(sp)
80000034:	01212823          	sw	s2,16(sp)
80000038:	00812c23          	sw	s0,24(sp)
8000003c:	00912a23          	sw	s1,20(sp)
80000040:	01312623          	sw	s3,12(sp)
80000044:	3f0000ef          	jal	ra,80000434 <printf>
80000048:	80001937          	lui	s2,0x80001
8000004c:	800015b7          	lui	a1,0x80001
80000050:	8d858593          	addi	a1,a1,-1832 # 800008d8 <__global_pointer$+0xffffe628>
80000054:	8e090513          	addi	a0,s2,-1824 # 800008e0 <__global_pointer$+0xffffe630>
80000058:	3dc000ef          	jal	ra,80000434 <printf>
8000005c:	00500293          	addi	t0,zero,5
80000060:	30529073          	csrrw	zero,mtvec,t0
80000064:	30501473          	csrrw	s0,mtvec,zero
80000068:	800014b7          	lui	s1,0x80001
8000006c:	00500613          	addi	a2,zero,5
80000070:	00040593          	addi	a1,s0,0
80000074:	8f048513          	addi	a0,s1,-1808 # 800008f0 <__global_pointer$+0xffffe640>
80000078:	3bc000ef          	jal	ra,80000434 <printf>
8000007c:	00500793          	addi	a5,zero,5
80000080:	08f41463          	bne	s0,a5,80000108 <test_csrs+0xe4>
80000084:	800015b7          	lui	a1,0x80001
80000088:	93058593          	addi	a1,a1,-1744 # 80000930 <__global_pointer$+0xffffe680>
8000008c:	8e090513          	addi	a0,s2,-1824
80000090:	3a4000ef          	jal	ra,80000434 <printf>
80000094:	00500293          	addi	t0,zero,5
80000098:	3052a073          	csrrs	zero,mtvec,t0
8000009c:	305029f3          	csrrs	s3,mtvec,zero
800000a0:	00500613          	addi	a2,zero,5
800000a4:	00098593          	addi	a1,s3,0
800000a8:	8f048513          	addi	a0,s1,-1808
800000ac:	388000ef          	jal	ra,80000434 <printf>
800000b0:	0e899863          	bne	s3,s0,800001a0 <test_csrs+0x17c>
800000b4:	800015b7          	lui	a1,0x80001
800000b8:	93858593          	addi	a1,a1,-1736 # 80000938 <__global_pointer$+0xffffe688>
800000bc:	8e090513          	addi	a0,s2,-1824
800000c0:	374000ef          	jal	ra,80000434 <printf>
800000c4:	00500293          	addi	t0,zero,5
800000c8:	1802b073          	csrrc	zero,satp,t0
800000cc:	18003473          	csrrc	s0,satp,zero
800000d0:	00500613          	addi	a2,zero,5
800000d4:	00040593          	addi	a1,s0,0
800000d8:	8f048513          	addi	a0,s1,-1808
800000dc:	358000ef          	jal	ra,80000434 <printf>
800000e0:	09341063          	bne	s0,s3,80000160 <test_csrs+0x13c>
800000e4:	01812403          	lw	s0,24(sp)
800000e8:	01c12083          	lw	ra,28(sp)
800000ec:	01412483          	lw	s1,20(sp)
800000f0:	01012903          	lw	s2,16(sp)
800000f4:	00c12983          	lw	s3,12(sp)
800000f8:	80001537          	lui	a0,0x80001
800000fc:	94050513          	addi	a0,a0,-1728 # 80000940 <__global_pointer$+0xffffe690>
80000100:	02010113          	addi	sp,sp,32
80000104:	3300006f          	jal	zero,80000434 <printf>
80000108:	80001637          	lui	a2,0x80001
8000010c:	800015b7          	lui	a1,0x80001
80000110:	80001537          	lui	a0,0x80001
80000114:	90460613          	addi	a2,a2,-1788 # 80000904 <__global_pointer$+0xffffe654>
80000118:	90858593          	addi	a1,a1,-1784 # 80000908 <__global_pointer$+0xffffe658>
8000011c:	90c50513          	addi	a0,a0,-1780 # 8000090c <__global_pointer$+0xffffe65c>
80000120:	314000ef          	jal	ra,80000434 <printf>
80000124:	800017b7          	lui	a5,0x80001
80000128:	92978793          	addi	a5,a5,-1751 # 80000929 <__global_pointer$+0xffffe679>
8000012c:	04500713          	addi	a4,zero,69
80000130:	100006b7          	lui	a3,0x10000
80000134:	00e68023          	sb	a4,0(a3) # 10000000 <main-0x70000000>
80000138:	0007c703          	lbu	a4,0(a5)
8000013c:	00178793          	addi	a5,a5,1
80000140:	fe071ae3          	bne	a4,zero,80000134 <test_csrs+0x110>
80000144:	03100793          	addi	a5,zero,49
80000148:	00f68023          	sb	a5,0(a3)
8000014c:	00a00793          	addi	a5,zero,10
80000150:	00f68023          	sb	a5,0(a3)
80000154:	00100793          	addi	a5,zero,1
80000158:	00078293          	addi	t0,a5,0
8000015c:	00502023          	sw	t0,0(zero) # 0 <main-0x80000000>
80000160:	80001637          	lui	a2,0x80001
80000164:	800015b7          	lui	a1,0x80001
80000168:	80001537          	lui	a0,0x80001
8000016c:	90460613          	addi	a2,a2,-1788 # 80000904 <__global_pointer$+0xffffe654>
80000170:	90858593          	addi	a1,a1,-1784 # 80000908 <__global_pointer$+0xffffe658>
80000174:	90c50513          	addi	a0,a0,-1780 # 8000090c <__global_pointer$+0xffffe65c>
80000178:	2bc000ef          	jal	ra,80000434 <printf>
8000017c:	800017b7          	lui	a5,0x80001
80000180:	92978793          	addi	a5,a5,-1751 # 80000929 <__global_pointer$+0xffffe679>
80000184:	04500713          	addi	a4,zero,69
80000188:	100006b7          	lui	a3,0x10000
8000018c:	00e68023          	sb	a4,0(a3) # 10000000 <main-0x70000000>
80000190:	0007c703          	lbu	a4,0(a5)
80000194:	00178793          	addi	a5,a5,1
80000198:	fe071ae3          	bne	a4,zero,8000018c <test_csrs+0x168>
8000019c:	fa9ff06f          	jal	zero,80000144 <test_csrs+0x120>
800001a0:	80001637          	lui	a2,0x80001
800001a4:	800015b7          	lui	a1,0x80001
800001a8:	80001537          	lui	a0,0x80001
800001ac:	90460613          	addi	a2,a2,-1788 # 80000904 <__global_pointer$+0xffffe654>
800001b0:	90858593          	addi	a1,a1,-1784 # 80000908 <__global_pointer$+0xffffe658>
800001b4:	90c50513          	addi	a0,a0,-1780 # 8000090c <__global_pointer$+0xffffe65c>
800001b8:	27c000ef          	jal	ra,80000434 <printf>
800001bc:	800017b7          	lui	a5,0x80001
800001c0:	92978793          	addi	a5,a5,-1751 # 80000929 <__global_pointer$+0xffffe679>
800001c4:	04500713          	addi	a4,zero,69
800001c8:	100006b7          	lui	a3,0x10000
800001cc:	00e68023          	sb	a4,0(a3) # 10000000 <main-0x70000000>
800001d0:	0007c703          	lbu	a4,0(a5)
800001d4:	00178793          	addi	a5,a5,1
800001d8:	fe071ae3          	bne	a4,zero,800001cc <test_csrs+0x1a8>
800001dc:	f69ff06f          	jal	zero,80000144 <test_csrs+0x120>

800001e0 <test_un_ops>:
800001e0:	80001537          	lui	a0,0x80001
800001e4:	ff010113          	addi	sp,sp,-16
800001e8:	95450513          	addi	a0,a0,-1708 # 80000954 <__global_pointer$+0xffffe6a4>
800001ec:	00112623          	sw	ra,12(sp)
800001f0:	00812423          	sw	s0,8(sp)
800001f4:	240000ef          	jal	ra,80000434 <printf>
800001f8:	80001437          	lui	s0,0x80001
800001fc:	800015b7          	lui	a1,0x80001
80000200:	97040513          	addi	a0,s0,-1680 # 80000970 <__global_pointer$+0xffffe6c0>
80000204:	00500613          	addi	a2,zero,5
80000208:	96c58593          	addi	a1,a1,-1684 # 8000096c <__global_pointer$+0xffffe6bc>
8000020c:	228000ef          	jal	ra,80000434 <printf>
80000210:	800015b7          	lui	a1,0x80001
80000214:	97040513          	addi	a0,s0,-1680
80000218:	00500613          	addi	a2,zero,5
8000021c:	99458593          	addi	a1,a1,-1644 # 80000994 <__global_pointer$+0xffffe6e4>
80000220:	214000ef          	jal	ra,80000434 <printf>
80000224:	800015b7          	lui	a1,0x80001
80000228:	97040513          	addi	a0,s0,-1680
8000022c:	00100613          	addi	a2,zero,1
80000230:	99858593          	addi	a1,a1,-1640 # 80000998 <__global_pointer$+0xffffe6e8>
80000234:	200000ef          	jal	ra,80000434 <printf>
80000238:	00812403          	lw	s0,8(sp)
8000023c:	00c12083          	lw	ra,12(sp)
80000240:	80001537          	lui	a0,0x80001
80000244:	94050513          	addi	a0,a0,-1728 # 80000940 <__global_pointer$+0xffffe690>
80000248:	01010113          	addi	sp,sp,16
8000024c:	1e80006f          	jal	zero,80000434 <printf>

80000250 <test_bin_ops>:
80000250:	80001537          	lui	a0,0x80001
80000254:	ff010113          	addi	sp,sp,-16
80000258:	99c50513          	addi	a0,a0,-1636 # 8000099c <__global_pointer$+0xffffe6ec>
8000025c:	00112623          	sw	ra,12(sp)
80000260:	00812423          	sw	s0,8(sp)
80000264:	1d0000ef          	jal	ra,80000434 <printf>
80000268:	80001437          	lui	s0,0x80001
8000026c:	800015b7          	lui	a1,0x80001
80000270:	9b440513          	addi	a0,s0,-1612 # 800009b4 <__global_pointer$+0xffffe704>
80000274:	00500693          	addi	a3,zero,5
80000278:	00500613          	addi	a2,zero,5
8000027c:	9b058593          	addi	a1,a1,-1616 # 800009b0 <__global_pointer$+0xffffe700>
80000280:	1b4000ef          	jal	ra,80000434 <printf>
80000284:	800015b7          	lui	a1,0x80001
80000288:	9b440513          	addi	a0,s0,-1612
8000028c:	00500693          	addi	a3,zero,5
80000290:	00500613          	addi	a2,zero,5
80000294:	96c58593          	addi	a1,a1,-1684 # 8000096c <__global_pointer$+0xffffe6bc>
80000298:	19c000ef          	jal	ra,80000434 <printf>
8000029c:	800015b7          	lui	a1,0x80001
800002a0:	9b440513          	addi	a0,s0,-1612
800002a4:	00500693          	addi	a3,zero,5
800002a8:	00500613          	addi	a2,zero,5
800002ac:	9e458593          	addi	a1,a1,-1564 # 800009e4 <__global_pointer$+0xffffe734>
800002b0:	184000ef          	jal	ra,80000434 <printf>
800002b4:	800015b7          	lui	a1,0x80001
800002b8:	9b440513          	addi	a0,s0,-1612
800002bc:	00500693          	addi	a3,zero,5
800002c0:	00a00613          	addi	a2,zero,10
800002c4:	9e858593          	addi	a1,a1,-1560 # 800009e8 <__global_pointer$+0xffffe738>
800002c8:	16c000ef          	jal	ra,80000434 <printf>
800002cc:	800015b7          	lui	a1,0x80001
800002d0:	9b440513          	addi	a0,s0,-1612
800002d4:	00500693          	addi	a3,zero,5
800002d8:	00500613          	addi	a2,zero,5
800002dc:	9ec58593          	addi	a1,a1,-1556 # 800009ec <__global_pointer$+0xffffe73c>
800002e0:	154000ef          	jal	ra,80000434 <printf>
800002e4:	800015b7          	lui	a1,0x80001
800002e8:	9b440513          	addi	a0,s0,-1612
800002ec:	00400693          	addi	a3,zero,4
800002f0:	00500613          	addi	a2,zero,5
800002f4:	9f058593          	addi	a1,a1,-1552 # 800009f0 <__global_pointer$+0xffffe740>
800002f8:	13c000ef          	jal	ra,80000434 <printf>
800002fc:	800015b7          	lui	a1,0x80001
80000300:	9b440513          	addi	a0,s0,-1612
80000304:	00400693          	addi	a3,zero,4
80000308:	00500613          	addi	a2,zero,5
8000030c:	9f458593          	addi	a1,a1,-1548 # 800009f4 <__global_pointer$+0xffffe744>
80000310:	124000ef          	jal	ra,80000434 <printf>
80000314:	800015b7          	lui	a1,0x80001
80000318:	9b440513          	addi	a0,s0,-1612
8000031c:	00400693          	addi	a3,zero,4
80000320:	00500613          	addi	a2,zero,5
80000324:	9f858593          	addi	a1,a1,-1544 # 800009f8 <__global_pointer$+0xffffe748>
80000328:	10c000ef          	jal	ra,80000434 <printf>
8000032c:	800015b7          	lui	a1,0x80001
80000330:	9b440513          	addi	a0,s0,-1612
80000334:	00100693          	addi	a3,zero,1
80000338:	00500613          	addi	a2,zero,5
8000033c:	9fc58593          	addi	a1,a1,-1540 # 800009fc <__global_pointer$+0xffffe74c>
80000340:	0f4000ef          	jal	ra,80000434 <printf>
80000344:	800015b7          	lui	a1,0x80001
80000348:	9b440513          	addi	a0,s0,-1612
8000034c:	00100693          	addi	a3,zero,1
80000350:	00500613          	addi	a2,zero,5
80000354:	a0058593          	addi	a1,a1,-1536 # 80000a00 <__global_pointer$+0xffffe750>
80000358:	0dc000ef          	jal	ra,80000434 <printf>
8000035c:	800015b7          	lui	a1,0x80001
80000360:	9b440513          	addi	a0,s0,-1612
80000364:	00100693          	addi	a3,zero,1
80000368:	00100613          	addi	a2,zero,1
8000036c:	a0458593          	addi	a1,a1,-1532 # 80000a04 <__global_pointer$+0xffffe754>
80000370:	0c4000ef          	jal	ra,80000434 <printf>
80000374:	800015b7          	lui	a1,0x80001
80000378:	9b440513          	addi	a0,s0,-1612
8000037c:	00000693          	addi	a3,zero,0
80000380:	00100613          	addi	a2,zero,1
80000384:	a0858593          	addi	a1,a1,-1528 # 80000a08 <__global_pointer$+0xffffe758>
80000388:	0ac000ef          	jal	ra,80000434 <printf>
8000038c:	800015b7          	lui	a1,0x80001
80000390:	9b440513          	addi	a0,s0,-1612
80000394:	00100693          	addi	a3,zero,1
80000398:	00100613          	addi	a2,zero,1
8000039c:	a0c58593          	addi	a1,a1,-1524 # 80000a0c <__global_pointer$+0xffffe75c>
800003a0:	094000ef          	jal	ra,80000434 <printf>
800003a4:	800015b7          	lui	a1,0x80001
800003a8:	9b440513          	addi	a0,s0,-1612
800003ac:	00000693          	addi	a3,zero,0
800003b0:	00100613          	addi	a2,zero,1
800003b4:	a1058593          	addi	a1,a1,-1520 # 80000a10 <__global_pointer$+0xffffe760>
800003b8:	07c000ef          	jal	ra,80000434 <printf>
800003bc:	800015b7          	lui	a1,0x80001
800003c0:	9b440513          	addi	a0,s0,-1612
800003c4:	00200693          	addi	a3,zero,2
800003c8:	00100613          	addi	a2,zero,1
800003cc:	a1458593          	addi	a1,a1,-1516 # 80000a14 <__global_pointer$+0xffffe764>
800003d0:	064000ef          	jal	ra,80000434 <printf>
800003d4:	800015b7          	lui	a1,0x80001
800003d8:	9b440513          	addi	a0,s0,-1612
800003dc:	00100693          	addi	a3,zero,1
800003e0:	00200613          	addi	a2,zero,2
800003e4:	a1858593          	addi	a1,a1,-1512 # 80000a18 <__global_pointer$+0xffffe768>
800003e8:	04c000ef          	jal	ra,80000434 <printf>
800003ec:	800015b7          	lui	a1,0x80001
800003f0:	9b440513          	addi	a0,s0,-1612
800003f4:	00200693          	addi	a3,zero,2
800003f8:	00100613          	addi	a2,zero,1
800003fc:	a1c58593          	addi	a1,a1,-1508 # 80000a1c <__global_pointer$+0xffffe76c>
80000400:	034000ef          	jal	ra,80000434 <printf>
80000404:	800015b7          	lui	a1,0x80001
80000408:	9b440513          	addi	a0,s0,-1612
8000040c:	00100693          	addi	a3,zero,1
80000410:	00200613          	addi	a2,zero,2
80000414:	a2058593          	addi	a1,a1,-1504 # 80000a20 <__global_pointer$+0xffffe770>
80000418:	01c000ef          	jal	ra,80000434 <printf>
8000041c:	00812403          	lw	s0,8(sp)
80000420:	00c12083          	lw	ra,12(sp)
80000424:	80001537          	lui	a0,0x80001
80000428:	94050513          	addi	a0,a0,-1728 # 80000940 <__global_pointer$+0xffffe690>
8000042c:	01010113          	addi	sp,sp,16
80000430:	0040006f          	jal	zero,80000434 <printf>

80000434 <printf>:
80000434:	fc010113          	addi	sp,sp,-64
80000438:	00054303          	lbu	t1,0(a0)
8000043c:	02f12a23          	sw	a5,52(sp)
80000440:	02410793          	addi	a5,sp,36
80000444:	00812e23          	sw	s0,28(sp)
80000448:	00912c23          	sw	s1,24(sp)
8000044c:	01212a23          	sw	s2,20(sp)
80000450:	01312823          	sw	s3,16(sp)
80000454:	02b12223          	sw	a1,36(sp)
80000458:	02c12423          	sw	a2,40(sp)
8000045c:	02d12623          	sw	a3,44(sp)
80000460:	02e12823          	sw	a4,48(sp)
80000464:	03012c23          	sw	a6,56(sp)
80000468:	03112e23          	sw	a7,60(sp)
8000046c:	00f12023          	sw	a5,0(sp)
80000470:	04030e63          	beq	t1,zero,800004cc <printf+0x98>
80000474:	800018b7          	lui	a7,0x80001
80000478:	80001fb7          	lui	t6,0x80001
8000047c:	80001f37          	lui	t5,0x80001
80000480:	00000793          	addi	a5,zero,0
80000484:	02500593          	addi	a1,zero,37
80000488:	10000737          	lui	a4,0x10000
8000048c:	01600e93          	addi	t4,zero,22
80000490:	800013b7          	lui	t2,0x80001
80000494:	a5488893          	addi	a7,a7,-1452 # 80000a54 <__global_pointer$+0xffffe7a4>
80000498:	a2cf8f93          	addi	t6,t6,-1492 # 80000a2c <__global_pointer$+0xffffe77c>
8000049c:	a24f0f13          	addi	t5,t5,-1500 # 80000a24 <__global_pointer$+0xffffe774>
800004a0:	800012b7          	lui	t0,0x80001
800004a4:	03000493          	addi	s1,zero,48
800004a8:	00900813          	addi	a6,zero,9
800004ac:	02d00413          	addi	s0,zero,45
800004b0:	00a00e13          	addi	t3,zero,10
800004b4:	02b30863          	beq	t1,a1,800004e4 <printf+0xb0>
800004b8:	00670023          	sb	t1,0(a4) # 10000000 <main-0x70000000>
800004bc:	00178793          	addi	a5,a5,1
800004c0:	00f506b3          	add	a3,a0,a5
800004c4:	0006c303          	lbu	t1,0(a3)
800004c8:	fe0316e3          	bne	t1,zero,800004b4 <printf+0x80>
800004cc:	01c12403          	lw	s0,28(sp)
800004d0:	01812483          	lw	s1,24(sp)
800004d4:	01412903          	lw	s2,20(sp)
800004d8:	01012983          	lw	s3,16(sp)
800004dc:	04010113          	addi	sp,sp,64
800004e0:	00008067          	jalr	zero,0(ra)
800004e4:	00178793          	addi	a5,a5,1
800004e8:	00f50333          	add	t1,a0,a5
800004ec:	00034683          	lbu	a3,0(t1)
800004f0:	f9e68693          	addi	a3,a3,-98
800004f4:	0ff6f693          	andi	a3,a3,255
800004f8:	00deea63          	bltu	t4,a3,8000050c <printf+0xd8>
800004fc:	00269693          	slli	a3,a3,0x2
80000500:	011686b3          	add	a3,a3,a7
80000504:	0006a683          	lw	a3,0(a3)
80000508:	00068067          	jalr	zero,0(a3)
8000050c:	a3938613          	addi	a2,t2,-1479 # 80000a39 <__global_pointer$+0xffffe789>
80000510:	05500693          	addi	a3,zero,85
80000514:	00d70023          	sb	a3,0(a4)
80000518:	00064683          	lbu	a3,0(a2)
8000051c:	00160613          	addi	a2,a2,1
80000520:	fe069ae3          	bne	a3,zero,80000514 <printf+0xe0>
80000524:	00034683          	lbu	a3,0(t1)
80000528:	00d70023          	sb	a3,0(a4)
8000052c:	f91ff06f          	jal	zero,800004bc <printf+0x88>
80000530:	00012903          	lw	s2,0(sp)
80000534:	a3528613          	addi	a2,t0,-1483 # 80000a35 <__global_pointer$+0xffffe785>
80000538:	03000693          	addi	a3,zero,48
8000053c:	00092303          	lw	t1,0(s2)
80000540:	00490913          	addi	s2,s2,4
80000544:	01212023          	sw	s2,0(sp)
80000548:	00d70023          	sb	a3,0(a4)
8000054c:	00064683          	lbu	a3,0(a2)
80000550:	00160613          	addi	a2,a2,1
80000554:	fe069ae3          	bne	a3,zero,80000548 <printf+0x114>
80000558:	01c35613          	srli	a2,t1,0x1c
8000055c:	03060913          	addi	s2,a2,48
80000560:	00c87463          	bgeu	a6,a2,80000568 <printf+0x134>
80000564:	03760913          	addi	s2,a2,55
80000568:	01835693          	srli	a3,t1,0x18
8000056c:	00f6f613          	andi	a2,a3,15
80000570:	01270023          	sb	s2,0(a4)
80000574:	03060913          	addi	s2,a2,48
80000578:	00c85463          	bge	a6,a2,80000580 <printf+0x14c>
8000057c:	03760913          	addi	s2,a2,55
80000580:	01435693          	srli	a3,t1,0x14
80000584:	00f6f613          	andi	a2,a3,15
80000588:	01270023          	sb	s2,0(a4)
8000058c:	03060913          	addi	s2,a2,48
80000590:	00c85463          	bge	a6,a2,80000598 <printf+0x164>
80000594:	03760913          	addi	s2,a2,55
80000598:	01035693          	srli	a3,t1,0x10
8000059c:	00f6f613          	andi	a2,a3,15
800005a0:	01270023          	sb	s2,0(a4)
800005a4:	03060913          	addi	s2,a2,48
800005a8:	00c85463          	bge	a6,a2,800005b0 <printf+0x17c>
800005ac:	03760913          	addi	s2,a2,55
800005b0:	00c35693          	srli	a3,t1,0xc
800005b4:	00f6f613          	andi	a2,a3,15
800005b8:	01270023          	sb	s2,0(a4)
800005bc:	03060913          	addi	s2,a2,48
800005c0:	00c85463          	bge	a6,a2,800005c8 <printf+0x194>
800005c4:	03760913          	addi	s2,a2,55
800005c8:	00835693          	srli	a3,t1,0x8
800005cc:	00f6f613          	andi	a2,a3,15
800005d0:	01270023          	sb	s2,0(a4)
800005d4:	03060913          	addi	s2,a2,48
800005d8:	00c85463          	bge	a6,a2,800005e0 <printf+0x1ac>
800005dc:	03760913          	addi	s2,a2,55
800005e0:	00435693          	srli	a3,t1,0x4
800005e4:	00f6f613          	andi	a2,a3,15
800005e8:	01270023          	sb	s2,0(a4)
800005ec:	03060913          	addi	s2,a2,48
800005f0:	00c85463          	bge	a6,a2,800005f8 <printf+0x1c4>
800005f4:	03760913          	addi	s2,a2,55
800005f8:	00f37613          	andi	a2,t1,15
800005fc:	01270023          	sb	s2,0(a4)
80000600:	03060693          	addi	a3,a2,48
80000604:	f2c852e3          	bge	a6,a2,80000528 <printf+0xf4>
80000608:	03760693          	addi	a3,a2,55
8000060c:	00d70023          	sb	a3,0(a4)
80000610:	eadff06f          	jal	zero,800004bc <printf+0x88>
80000614:	00012603          	lw	a2,0(sp)
80000618:	00062683          	lw	a3,0(a2)
8000061c:	00460613          	addi	a2,a2,4
80000620:	00c12023          	sw	a2,0(sp)
80000624:	0006c603          	lbu	a2,0(a3)
80000628:	e8060ae3          	beq	a2,zero,800004bc <printf+0x88>
8000062c:	00168693          	addi	a3,a3,1
80000630:	00c70023          	sb	a2,0(a4)
80000634:	0006c603          	lbu	a2,0(a3)
80000638:	00168693          	addi	a3,a3,1
8000063c:	fe061ae3          	bne	a2,zero,80000630 <printf+0x1fc>
80000640:	e7dff06f          	jal	zero,800004bc <printf+0x88>
80000644:	00012903          	lw	s2,0(sp)
80000648:	a3528613          	addi	a2,t0,-1483
8000064c:	03000693          	addi	a3,zero,48
80000650:	00092303          	lw	t1,0(s2)
80000654:	00490913          	addi	s2,s2,4
80000658:	01212023          	sw	s2,0(sp)
8000065c:	00d70023          	sb	a3,0(a4)
80000660:	00064683          	lbu	a3,0(a2)
80000664:	00160613          	addi	a2,a2,1
80000668:	fe069ae3          	bne	a3,zero,8000065c <printf+0x228>
8000066c:	eedff06f          	jal	zero,80000558 <printf+0x124>
80000670:	00012603          	lw	a2,0(sp)
80000674:	00062683          	lw	a3,0(a2)
80000678:	00460613          	addi	a2,a2,4
8000067c:	00c12023          	sw	a2,0(sp)
80000680:	0406ce63          	blt	a3,zero,800006dc <printf+0x2a8>
80000684:	06069063          	bne	a3,zero,800006e4 <printf+0x2b0>
80000688:	00970023          	sb	s1,0(a4)
8000068c:	e31ff06f          	jal	zero,800004bc <printf+0x88>
80000690:	00012683          	lw	a3,0(sp)
80000694:	0006c603          	lbu	a2,0(a3)
80000698:	00468693          	addi	a3,a3,4
8000069c:	00d12023          	sw	a3,0(sp)
800006a0:	00c70023          	sb	a2,0(a4)
800006a4:	e19ff06f          	jal	zero,800004bc <printf+0x88>
800006a8:	00012683          	lw	a3,0(sp)
800006ac:	0006a603          	lw	a2,0(a3)
800006b0:	00468693          	addi	a3,a3,4
800006b4:	00d12023          	sw	a3,0(sp)
800006b8:	1a060c63          	beq	a2,zero,80000870 <printf+0x43c>
800006bc:	07400613          	addi	a2,zero,116
800006c0:	000f8693          	addi	a3,t6,0
800006c4:	00168693          	addi	a3,a3,1
800006c8:	00c70023          	sb	a2,0(a4)
800006cc:	0006c603          	lbu	a2,0(a3)
800006d0:	00168693          	addi	a3,a3,1
800006d4:	fe061ae3          	bne	a2,zero,800006c8 <printf+0x294>
800006d8:	de5ff06f          	jal	zero,800004bc <printf+0x88>
800006dc:	00870023          	sb	s0,0(a4)
800006e0:	40d006b3          	sub	a3,zero,a3
800006e4:	03c6e633          	rem	a2,a3,t3
800006e8:	03c6c6b3          	div	a3,a3,t3
800006ec:	03060613          	addi	a2,a2,48
800006f0:	00c10223          	sb	a2,4(sp)
800006f4:	16068863          	beq	a3,zero,80000864 <printf+0x430>
800006f8:	03c6e633          	rem	a2,a3,t3
800006fc:	03c6c6b3          	div	a3,a3,t3
80000700:	03060613          	addi	a2,a2,48
80000704:	00c102a3          	sb	a2,5(sp)
80000708:	16068a63          	beq	a3,zero,8000087c <printf+0x448>
8000070c:	03c6e633          	rem	a2,a3,t3
80000710:	03c6c6b3          	div	a3,a3,t3
80000714:	03060613          	addi	a2,a2,48
80000718:	00c10323          	sb	a2,6(sp)
8000071c:	16068863          	beq	a3,zero,8000088c <printf+0x458>
80000720:	03c6e633          	rem	a2,a3,t3
80000724:	03c6c6b3          	div	a3,a3,t3
80000728:	03060613          	addi	a2,a2,48
8000072c:	00c103a3          	sb	a2,7(sp)
80000730:	16068263          	beq	a3,zero,80000894 <printf+0x460>
80000734:	03c6e633          	rem	a2,a3,t3
80000738:	03c6c6b3          	div	a3,a3,t3
8000073c:	03060613          	addi	a2,a2,48
80000740:	00c10423          	sb	a2,8(sp)
80000744:	14068c63          	beq	a3,zero,8000089c <printf+0x468>
80000748:	03c6e633          	rem	a2,a3,t3
8000074c:	03c6c6b3          	div	a3,a3,t3
80000750:	03060613          	addi	a2,a2,48
80000754:	00c104a3          	sb	a2,9(sp)
80000758:	14068663          	beq	a3,zero,800008a4 <printf+0x470>
8000075c:	03c6e633          	rem	a2,a3,t3
80000760:	03c6c6b3          	div	a3,a3,t3
80000764:	03060613          	addi	a2,a2,48
80000768:	00c10523          	sb	a2,10(sp)
8000076c:	14068063          	beq	a3,zero,800008ac <printf+0x478>
80000770:	03c6e633          	rem	a2,a3,t3
80000774:	03c6c6b3          	div	a3,a3,t3
80000778:	03060613          	addi	a2,a2,48
8000077c:	00c105a3          	sb	a2,11(sp)
80000780:	12068a63          	beq	a3,zero,800008b4 <printf+0x480>
80000784:	03c6e633          	rem	a2,a3,t3
80000788:	03c6c6b3          	div	a3,a3,t3
8000078c:	03060613          	addi	a2,a2,48
80000790:	00c10623          	sb	a2,12(sp)
80000794:	12068463          	beq	a3,zero,800008bc <printf+0x488>
80000798:	03068693          	addi	a3,a3,48
8000079c:	00d106a3          	sb	a3,13(sp)
800007a0:	00a00693          	addi	a3,zero,10
800007a4:	01068613          	addi	a2,a3,16
800007a8:	00260333          	add	t1,a2,sp
800007ac:	ff334983          	lbu	s3,-13(t1)
800007b0:	00d68913          	addi	s2,a3,13
800007b4:	00290933          	add	s2,s2,sp
800007b8:	01370023          	sb	s3,0(a4)
800007bc:	ff234303          	lbu	t1,-14(t1)
800007c0:	ffd68613          	addi	a2,a3,-3
800007c4:	00670023          	sb	t1,0(a4)
800007c8:	ff494303          	lbu	t1,-12(s2)
800007cc:	00670023          	sb	t1,0(a4)
800007d0:	ce0606e3          	beq	a2,zero,800004bc <printf+0x88>
800007d4:	00c68313          	addi	t1,a3,12
800007d8:	00230333          	add	t1,t1,sp
800007dc:	ff434303          	lbu	t1,-12(t1)
800007e0:	ffc68613          	addi	a2,a3,-4
800007e4:	00670023          	sb	t1,0(a4)
800007e8:	cc060ae3          	beq	a2,zero,800004bc <printf+0x88>
800007ec:	00b68313          	addi	t1,a3,11
800007f0:	00230333          	add	t1,t1,sp
800007f4:	ff434303          	lbu	t1,-12(t1)
800007f8:	ffb68613          	addi	a2,a3,-5
800007fc:	00670023          	sb	t1,0(a4)
80000800:	ca060ee3          	beq	a2,zero,800004bc <printf+0x88>
80000804:	00a68313          	addi	t1,a3,10
80000808:	00230333          	add	t1,t1,sp
8000080c:	ff434303          	lbu	t1,-12(t1)
80000810:	ffa68613          	addi	a2,a3,-6
80000814:	00670023          	sb	t1,0(a4)
80000818:	ca0602e3          	beq	a2,zero,800004bc <printf+0x88>
8000081c:	00968313          	addi	t1,a3,9
80000820:	00230333          	add	t1,t1,sp
80000824:	ff434303          	lbu	t1,-12(t1)
80000828:	ff968613          	addi	a2,a3,-7
8000082c:	00670023          	sb	t1,0(a4)
80000830:	c80606e3          	beq	a2,zero,800004bc <printf+0x88>
80000834:	00868313          	addi	t1,a3,8
80000838:	00230333          	add	t1,t1,sp
8000083c:	ff434303          	lbu	t1,-12(t1)
80000840:	ff868613          	addi	a2,a3,-8
80000844:	00670023          	sb	t1,0(a4)
80000848:	c6060ae3          	beq	a2,zero,800004bc <printf+0x88>
8000084c:	00768613          	addi	a2,a3,7
80000850:	00260633          	add	a2,a2,sp
80000854:	ff464603          	lbu	a2,-12(a2)
80000858:	ff768693          	addi	a3,a3,-9
8000085c:	00c70023          	sb	a2,0(a4)
80000860:	c4068ee3          	beq	a3,zero,800004bc <printf+0x88>
80000864:	00414683          	lbu	a3,4(sp)
80000868:	00d70023          	sb	a3,0(a4)
8000086c:	c51ff06f          	jal	zero,800004bc <printf+0x88>
80000870:	06600613          	addi	a2,zero,102
80000874:	000f0693          	addi	a3,t5,0
80000878:	e4dff06f          	jal	zero,800006c4 <printf+0x290>
8000087c:	00514683          	lbu	a3,5(sp)
80000880:	00d70023          	sb	a3,0(a4)
80000884:	00414683          	lbu	a3,4(sp)
80000888:	fe1ff06f          	jal	zero,80000868 <printf+0x434>
8000088c:	00300693          	addi	a3,zero,3
80000890:	f15ff06f          	jal	zero,800007a4 <printf+0x370>
80000894:	00400693          	addi	a3,zero,4
80000898:	f0dff06f          	jal	zero,800007a4 <printf+0x370>
8000089c:	00500693          	addi	a3,zero,5
800008a0:	f05ff06f          	jal	zero,800007a4 <printf+0x370>
800008a4:	00600693          	addi	a3,zero,6
800008a8:	efdff06f          	jal	zero,800007a4 <printf+0x370>
800008ac:	00700693          	addi	a3,zero,7
800008b0:	ef5ff06f          	jal	zero,800007a4 <printf+0x370>
800008b4:	00800693          	addi	a3,zero,8
800008b8:	eedff06f          	jal	zero,800007a4 <printf+0x370>
800008bc:	00900693          	addi	a3,zero,9
800008c0:	ee5ff06f          	jal	zero,800007a4 <printf+0x370>

Disassembly of section .rodata:

800008c4 <__BSS_END__-0x11ec>:
800008c4:	7552                	c.flwsp	fa0,52(sp)
800008c6:	6e6e                	c.flwsp	ft8,216(sp)
800008c8:	6e69                	c.lui	t3,0x1a
800008ca:	73632067          	0x73632067
800008ce:	5f72                	c.lwsp	t5,60(sp)
800008d0:	6574                	c.flw	fa3,76(a0)
800008d2:	0a737473          	csrrci	s0,0xa7,6
800008d6:	0000                	c.unimp
800008d8:	72727363          	bgeu	tp,t2,80000ffe <printf+0xbca>
800008dc:	00000077          	0x77
800008e0:	6554                	c.flw	fa3,12(a0)
800008e2:	6e697473          	csrrci	s0,0x6e6,18
800008e6:	53432067          	0x53432067
800008ea:	2052                	c.fldsp	ft0,272(sp)
800008ec:	7325                	c.lui	t1,0xfffe9
800008ee:	000a                	c.slli	zero,0x2
800008f0:	63656843          	fmadd.d	fa6,fa0,fs6,fa2,unknown
800008f4:	676e696b          	0x676e696b
800008f8:	2520                	c.fld	fs0,72(a0)
800008fa:	2064                	c.fld	fs1,192(s0)
800008fc:	3d3d                	c.jal	8000073a <printf+0x306>
800008fe:	2520                	c.fld	fs0,72(a0)
80000900:	0a64                	c.addi4spn	s1,sp,284
80000902:	0000                	c.unimp
80000904:	0035                	c.addi	zero,13
80000906:	0000                	c.unimp
80000908:	0078                	c.addi4spn	a4,sp,12
8000090a:	0000                	c.unimp
8000090c:	7341                	c.lui	t1,0xffff0
8000090e:	74726573          	csrrsi	a0,0x747,4
80000912:	6f69                	c.lui	t5,0x1a
80000914:	206e                	c.fldsp	ft0,216(sp)
80000916:	6166                	c.flwsp	ft2,88(sp)
80000918:	6c69                	c.lui	s8,0x1a
8000091a:	6465                	c.lui	s0,0x19
8000091c:	203a                	c.fldsp	ft0,392(sp)
8000091e:	7325                	c.lui	t1,0xfffe9
80000920:	2120                	c.fld	fs0,64(a0)
80000922:	203d                	c.jal	80000950 <printf+0x51c>
80000924:	7325                	c.lui	t1,0xfffe9
80000926:	000a                	c.slli	zero,0x2
80000928:	7845                	c.lui	a6,0xffff1
8000092a:	7469                	c.lui	s0,0xffffa
8000092c:	203a                	c.fldsp	ft0,392(sp)
8000092e:	0000                	c.unimp
80000930:	72727363          	bgeu	tp,t2,80001056 <printf+0xc22>
80000934:	00000073          	ecall
80000938:	72727363          	bgeu	tp,t2,8000105e <printf+0xc2a>
8000093c:	00000063          	beq	zero,zero,8000093c <printf+0x508>
80000940:	6c41                	c.lui	s8,0x10
80000942:	206c                	c.fld	fa1,192(s0)
80000944:	6574                	c.flw	fa3,76(a0)
80000946:	20737473          	csrrci	s0,0x207,6
8000094a:	6170                	c.flw	fa2,68(a0)
8000094c:	64657373          	csrrci	t1,0x646,10
80000950:	000a                	c.slli	zero,0x2
80000952:	0000                	c.unimp
80000954:	7552                	c.flwsp	fa0,52(sp)
80000956:	6e6e                	c.flwsp	ft8,216(sp)
80000958:	6e69                	c.lui	t3,0x1a
8000095a:	6e752067          	0x6e752067
8000095e:	6f5f 5f70 6574      	0x65745f706f5f
80000964:	0a737473          	csrrci	s0,0xa7,6
80000968:	0000                	c.unimp
8000096a:	0000                	c.unimp
8000096c:	002d                	c.addi	zero,11
8000096e:	0000                	c.unimp
80000970:	6554                	c.flw	fa3,12(a0)
80000972:	6e697473          	csrrci	s0,0x6e6,18
80000976:	704f2067          	0x704f2067
8000097a:	7265                	c.lui	tp,0xffff9
8000097c:	7461                	c.lui	s0,0xffff8
8000097e:	6f69                	c.lui	t5,0x1a
80000980:	206e                	c.fldsp	ft0,216(sp)
80000982:	7325                	c.lui	t1,0xfffe9
80000984:	7720                	c.flw	fs0,104(a4)
80000986:	7469                	c.lui	s0,0xffffa
80000988:	2068                	c.fld	fa0,192(s0)
8000098a:	2078                	c.fld	fa4,192(s0)
8000098c:	203d                	c.jal	800009ba <printf+0x586>
8000098e:	6425                	c.lui	s0,0x9
80000990:	000a                	c.slli	zero,0x2
80000992:	0000                	c.unimp
80000994:	007e                	c.slli	zero,0x1f
80000996:	0000                	c.unimp
80000998:	0021                	c.addi	zero,8
8000099a:	0000                	c.unimp
8000099c:	7552                	c.flwsp	fa0,52(sp)
8000099e:	6e6e                	c.flwsp	ft8,216(sp)
800009a0:	6e69                	c.lui	t3,0x1a
800009a2:	706f2067          	0x706f2067
800009a6:	745f 7365 7374      	0x73747365745f
800009ac:	000a                	c.slli	zero,0x2
800009ae:	0000                	c.unimp
800009b0:	0000002b          	0x2b
800009b4:	6554                	c.flw	fa3,12(a0)
800009b6:	6e697473          	csrrci	s0,0x6e6,18
800009ba:	704f2067          	0x704f2067
800009be:	7265                	c.lui	tp,0xffff9
800009c0:	7461                	c.lui	s0,0xffff8
800009c2:	6f69                	c.lui	t5,0x1a
800009c4:	206e                	c.fldsp	ft0,216(sp)
800009c6:	7325                	c.lui	t1,0xfffe9
800009c8:	7720                	c.flw	fs0,104(a4)
800009ca:	7469                	c.lui	s0,0xffffa
800009cc:	2068                	c.fld	fa0,192(s0)
800009ce:	2078                	c.fld	fa4,192(s0)
800009d0:	203d                	c.jal	800009fe <printf+0x5ca>
800009d2:	6425                	c.lui	s0,0x9
800009d4:	6120                	c.flw	fs0,64(a0)
800009d6:	646e                	c.flwsp	fs0,216(sp)
800009d8:	7920                	c.flw	fs0,112(a0)
800009da:	3d20                	c.fld	fs0,120(a0)
800009dc:	2520                	c.fld	fs0,72(a0)
800009de:	0a64                	c.addi4spn	s1,sp,284
800009e0:	0000                	c.unimp
800009e2:	0000                	c.unimp
800009e4:	002a                	c.slli	zero,0xa
800009e6:	0000                	c.unimp
800009e8:	0000002f          	0x2f
800009ec:	0025                	c.addi	zero,9
800009ee:	0000                	c.unimp
800009f0:	005e                	c.slli	zero,0x17
800009f2:	0000                	c.unimp
800009f4:	0026                	c.slli	zero,0x9
800009f6:	0000                	c.unimp
800009f8:	007c                	c.addi4spn	a5,sp,12
800009fa:	0000                	c.unimp
800009fc:	3c3c                	c.fld	fa5,120(s0)
800009fe:	0000                	c.unimp
80000a00:	3e3e                	c.fldsp	ft8,488(sp)
80000a02:	0000                	c.unimp
80000a04:	2626                	c.fldsp	fa2,72(sp)
80000a06:	0000                	c.unimp
80000a08:	7c7c                	c.flw	fa5,124(s0)
80000a0a:	0000                	c.unimp
80000a0c:	3d3d                	c.jal	8000084a <printf+0x416>
80000a0e:	0000                	c.unimp
80000a10:	3d21                	c.jal	80000828 <printf+0x3f4>
80000a12:	0000                	c.unimp
80000a14:	003c                	c.addi4spn	a5,sp,8
80000a16:	0000                	c.unimp
80000a18:	003e                	c.slli	zero,0xf
80000a1a:	0000                	c.unimp
80000a1c:	3d3c                	c.fld	fa5,120(a0)
80000a1e:	0000                	c.unimp
80000a20:	3d3e                	c.fldsp	fs10,488(sp)
80000a22:	0000                	c.unimp
80000a24:	6166                	c.flwsp	ft2,88(sp)
80000a26:	736c                	c.flw	fa1,100(a4)
80000a28:	0065                	c.addi	zero,25
80000a2a:	0000                	c.unimp
80000a2c:	7274                	c.flw	fa3,100(a2)
80000a2e:	6575                	c.lui	a0,0x1d
80000a30:	0000                	c.unimp
80000a32:	0000                	c.unimp
80000a34:	7830                	c.flw	fa2,112(s0)
80000a36:	0000                	c.unimp
80000a38:	6e55                	c.lui	t3,0x15
80000a3a:	776f6e6b          	0x776f6e6b
80000a3e:	206e                	c.fldsp	ft0,216(sp)
80000a40:	6f66                	c.flwsp	ft10,88(sp)
80000a42:	6d72                	c.flwsp	fs10,28(sp)
80000a44:	7461                	c.lui	s0,0xffff8
80000a46:	7320                	c.flw	fs0,96(a4)
80000a48:	6570                	c.flw	fa2,76(a0)
80000a4a:	69666963          	bltu	a2,s6,800010dc <printf+0xca8>
80000a4e:	7265                	c.lui	tp,0xffff9
80000a50:	203a                	c.fldsp	ft0,392(sp)
80000a52:	0025                	c.addi	zero,9
80000a54:	06a8                	c.addi4spn	a0,sp,840
80000a56:	8000                	0x8000
80000a58:	0690                	c.addi4spn	a2,sp,832
80000a5a:	8000                	0x8000
80000a5c:	0670                	c.addi4spn	a2,sp,780
80000a5e:	8000                	0x8000
80000a60:	050c                	c.addi4spn	a1,sp,640
80000a62:	8000                	0x8000
80000a64:	050c                	c.addi4spn	a1,sp,640
80000a66:	8000                	0x8000
80000a68:	050c                	c.addi4spn	a1,sp,640
80000a6a:	8000                	0x8000
80000a6c:	050c                	c.addi4spn	a1,sp,640
80000a6e:	8000                	0x8000
80000a70:	050c                	c.addi4spn	a1,sp,640
80000a72:	8000                	0x8000
80000a74:	050c                	c.addi4spn	a1,sp,640
80000a76:	8000                	0x8000
80000a78:	050c                	c.addi4spn	a1,sp,640
80000a7a:	8000                	0x8000
80000a7c:	050c                	c.addi4spn	a1,sp,640
80000a7e:	8000                	0x8000
80000a80:	050c                	c.addi4spn	a1,sp,640
80000a82:	8000                	0x8000
80000a84:	050c                	c.addi4spn	a1,sp,640
80000a86:	8000                	0x8000
80000a88:	050c                	c.addi4spn	a1,sp,640
80000a8a:	8000                	0x8000
80000a8c:	0644                	c.addi4spn	s1,sp,772
80000a8e:	8000                	0x8000
80000a90:	050c                	c.addi4spn	a1,sp,640
80000a92:	8000                	0x8000
80000a94:	050c                	c.addi4spn	a1,sp,640
80000a96:	8000                	0x8000
80000a98:	0614                	c.addi4spn	a3,sp,768
80000a9a:	8000                	0x8000
80000a9c:	050c                	c.addi4spn	a1,sp,640
80000a9e:	8000                	0x8000
80000aa0:	050c                	c.addi4spn	a1,sp,640
80000aa2:	8000                	0x8000
80000aa4:	050c                	c.addi4spn	a1,sp,640
80000aa6:	8000                	0x8000
80000aa8:	050c                	c.addi4spn	a1,sp,640
80000aaa:	8000                	0x8000
80000aac:	0530                	c.addi4spn	a2,sp,648
80000aae:	8000                	0x8000

Disassembly of section .comment:

00000000 <.comment>:
   0:	3a434347          	fmsub.d	ft6,ft6,ft4,ft7,rmm
   4:	2820                	c.fld	fs0,80(s0)
   6:	29554e47          	fmsub.s	ft8,fa0,fs5,ft5,rmm
   a:	3120                	c.fld	fs0,96(a0)
   c:	2e31                	c.jal	328 <main-0x7ffffcd8>
   e:	2e31                	c.jal	32a <main-0x7ffffcd6>
  10:	0030                	c.addi4spn	a2,sp,8

Disassembly of section .riscv.attributes:

00000000 <.riscv.attributes>:
   0:	2941                	c.jal	490 <main-0x7ffffb70>
   2:	0000                	c.unimp
   4:	7200                	c.flw	fs0,32(a2)
   6:	7369                	c.lui	t1,0xffffa
   8:	01007663          	bgeu	zero,a6,14 <main-0x7fffffec>
   c:	001f 0000 1004      	0x10040000001f
  12:	7205                	c.lui	tp,0xfffe1
  14:	3376                	c.fldsp	ft6,376(sp)
  16:	6932                	c.flwsp	fs2,12(sp)
  18:	7032                	c.flwsp	ft0,44(sp)
  1a:	5f30                	c.lw	a2,120(a4)
  1c:	326d                	c.jal	fffff9c6 <__global_pointer$+0x7fffd716>
  1e:	3070                	c.fld	fa2,224(s0)
  20:	615f 7032 0030      	0x307032615f
  26:	0108                	c.addi4spn	a0,sp,128
  28:	0b0a                	c.slli	s6,0x2
