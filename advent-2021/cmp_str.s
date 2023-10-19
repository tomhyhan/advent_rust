	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 1
	.file	"cmp_str.c"
	.def	_cmp_str;
	.scl	2;
	.type	32;
	.endef
	.globl	_cmp_str                        # -- Begin function cmp_str
	.p2align	4, 0x90
_cmp_str:                               # @cmp_str
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$8, %esp
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
LBB0_1:                                 # =>This Inner Loop Header: Depth=1
	movl	8(%ebp), %eax
	movsbl	(%eax), %ecx
	xorl	%eax, %eax
                                        # kill: def $al killed $al killed $eax
	cmpl	$0, %ecx
	movb	%al, -5(%ebp)                   # 1-byte Spill
	je	LBB0_3
# %bb.2:                                #   in Loop: Header=BB0_1 Depth=1
	movl	8(%ebp), %eax
	movsbl	(%eax), %eax
	movl	12(%ebp), %ecx
	movsbl	(%ecx), %ecx
	cmpl	%ecx, %eax
	sete	%al
	movb	%al, -5(%ebp)                   # 1-byte Spill
LBB0_3:                                 #   in Loop: Header=BB0_1 Depth=1
	movb	-5(%ebp), %al                   # 1-byte Reload
	testb	$1, %al
	jne	LBB0_4
	jmp	LBB0_5
LBB0_4:                                 #   in Loop: Header=BB0_1 Depth=1
	movl	8(%ebp), %eax
	addl	$1, %eax
	movl	%eax, 8(%ebp)
	movl	12(%ebp), %eax
	addl	$1, %eax
	movl	%eax, 12(%ebp)
	jmp	LBB0_1
LBB0_5:
	movl	8(%ebp), %eax
	movsbl	(%eax), %eax
	movl	12(%ebp), %ecx
	movsbl	(%ecx), %ecx
	subl	%ecx, %eax
	cmpl	$0, %eax
	jne	LBB0_7
# %bb.6:
	movl	$0, -4(%ebp)
	jmp	LBB0_8
LBB0_7:
	movl	8(%ebp), %eax
	movsbl	(%eax), %edx
	movl	12(%ebp), %eax
	movsbl	(%eax), %eax
	subl	%eax, %edx
	movl	$4294967295, %eax               # imm = 0xFFFFFFFF
	movl	$1, %ecx
	cmpl	$0, %edx
	cmovgl	%ecx, %eax
	movl	%eax, -4(%ebp)
LBB0_8:
	movl	-4(%ebp), %eax
	addl	$8, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_main;
	.scl	2;
	.type	32;
	.endef
	.globl	_main                           # -- Begin function main
	.p2align	4, 0x90
_main:                                  # @main
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$20, %esp
	movl	$0, -4(%ebp)
	movw	L___const.main.a, %ax
	movw	%ax, -7(%ebp)
	movb	L___const.main.a+2, %al
	movb	%al, -5(%ebp)
	movw	L___const.main.b, %ax
	movw	%ax, -10(%ebp)
	movb	L___const.main.b+2, %al
	movb	%al, -8(%ebp)
	leal	-10(%ebp), %eax
	leal	-7(%ebp), %ecx
	movl	%ecx, (%esp)
	movl	%eax, 4(%esp)
	calll	_cmp_str
	xorl	%eax, %eax
	addl	$20, %esp
	popl	%ebp
	retl
                                        # -- End function
	.section	.rdata,"dr"
L___const.main.a:                       # @__const.main.a
	.asciz	"AC"

L___const.main.b:                       # @__const.main.b
	.asciz	"AB"

	.addrsig
	.addrsig_sym _cmp_str
