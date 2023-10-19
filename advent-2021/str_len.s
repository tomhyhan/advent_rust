	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 1
	.file	"str_len.c"
	.def	_str_len;
	.scl	2;
	.type	32;
	.endef
	.globl	_str_len                        # -- Begin function str_len
	.p2align	4, 0x90
_str_len:                               # @str_len
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	pushl	%eax
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	%eax, -4(%ebp)
LBB0_1:                                 # =>This Inner Loop Header: Depth=1
	movl	-4(%ebp), %eax
	movsbl	(%eax), %eax
	cmpl	$0, %eax
	je	LBB0_3
# %bb.2:                                #   in Loop: Header=BB0_1 Depth=1
	movl	-4(%ebp), %eax
	addl	$1, %eax
	movl	%eax, -4(%ebp)
	jmp	LBB0_1
LBB0_3:
	movl	-4(%ebp), %eax
	movl	8(%ebp), %ecx
	subl	%ecx, %eax
	addl	$4, %esp
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
	subl	$24, %esp
	movl	$0, -4(%ebp)
	movl	L___const.main.str, %eax
	movl	%eax, -19(%ebp)
	movl	L___const.main.str+4, %eax
	movl	%eax, -15(%ebp)
	movl	L___const.main.str+8, %eax
	movl	%eax, -11(%ebp)
	movw	L___const.main.str+12, %ax
	movw	%ax, -7(%ebp)
	movb	L___const.main.str+14, %al
	movb	%al, -5(%ebp)
	leal	-19(%ebp), %eax
	movl	%eax, (%esp)
	calll	_str_len
	xorl	%eax, %eax
	addl	$24, %esp
	popl	%ebp
	retl
                                        # -- End function
	.section	.rdata,"dr"
L___const.main.str:                     # @__const.main.str
	.asciz	"as234523452345"

	.addrsig
	.addrsig_sym _str_len
