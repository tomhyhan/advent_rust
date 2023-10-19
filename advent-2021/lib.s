	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 1
	.file	"lib.c"
	.def	_init_vector;
	.scl	2;
	.type	32;
	.endef
	.globl	_init_vector                    # -- Begin function init_vector
	.p2align	4, 0x90
_init_vector:                           # @init_vector
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$12, %esp
	movl	8(%ebp), %eax
	movl	$12, (%esp)
	calll	_malloc
	movl	%eax, -8(%ebp)
	movl	8(%ebp), %eax
	shll	$3, %eax
	movl	%eax, (%esp)
	calll	_malloc
	movl	%eax, %ecx
	movl	-8(%ebp), %eax
	movl	%ecx, (%eax)
	movl	-8(%ebp), %eax
	cmpl	$0, (%eax)
	jne	LBB0_2
# %bb.1:
	movl	-8(%ebp), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	$0, -4(%ebp)
	jmp	LBB0_3
LBB0_2:
	movl	-8(%ebp), %eax
	movl	$0, 4(%eax)
	movl	8(%ebp), %ecx
	movl	-8(%ebp), %eax
	movl	%ecx, 8(%eax)
	movl	-8(%ebp), %eax
	movl	%eax, -4(%ebp)
LBB0_3:
	movl	-4(%ebp), %eax
	addl	$12, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_get;
	.scl	2;
	.type	32;
	.endef
	.globl	_get                            # -- Begin function get
	.p2align	4, 0x90
_get:                                   # @get
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$8, %esp
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
	cmpl	$0, 12(%ebp)
	jb	LBB1_2
# %bb.1:
	movl	12(%ebp), %eax
	movl	8(%ebp), %ecx
	cmpl	4(%ecx), %eax
	jb	LBB1_3
LBB1_2:
	leal	"??_C@_0M@JBMNDFH@index?5error?$AA@", %eax
	movl	%eax, (%esp)
	calll	_printf
	movl	$-1, -4(%ebp)
	jmp	LBB1_4
LBB1_3:
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	12(%ebp), %ecx
	movl	(%eax,%ecx,8), %eax
	movl	%eax, -4(%ebp)
LBB1_4:
	movl	-4(%ebp), %eax
	addl	$8, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_printf;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,_printf
	.globl	_printf                         # -- Begin function printf
	.p2align	4, 0x90
_printf:                                # @printf
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	pushl	%esi
	subl	$28, %esp
	movl	8(%ebp), %eax
	leal	12(%ebp), %eax
	movl	%eax, -12(%ebp)
	movl	-12(%ebp), %esi
	movl	8(%ebp), %eax
	movl	%eax, -16(%ebp)                 # 4-byte Spill
	movl	%esp, %eax
	movl	$1, (%eax)
	calll	___acrt_iob_func
	movl	-16(%ebp), %edx                 # 4-byte Reload
	movl	%eax, %ecx
	movl	%esp, %eax
	movl	%esi, 12(%eax)
	movl	%edx, 4(%eax)
	movl	%ecx, (%eax)
	movl	$0, 8(%eax)
	calll	__vfprintf_l
	movl	%eax, -8(%ebp)
	movl	-8(%ebp), %eax
	addl	$28, %esp
	popl	%esi
	popl	%ebp
	retl
                                        # -- End function
	.def	_push;
	.scl	2;
	.type	32;
	.endef
	.text
	.globl	_push                           # -- Begin function push
	.p2align	4, 0x90
_push:                                  # @push
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$12, %esp
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	4(%eax), %eax
	movl	8(%ebp), %ecx
	cmpl	8(%ecx), %eax
	jne	LBB3_4
# %bb.1:
	movl	8(%ebp), %eax
	movl	8(%eax), %ecx
	shll	$1, %ecx
	movl	%ecx, 8(%eax)
	movl	8(%ebp), %eax
	movl	8(%eax), %eax
	shll	$3, %eax
	movl	8(%ebp), %ecx
	movl	(%ecx), %ecx
	movl	%ecx, (%esp)
	movl	%eax, 4(%esp)
	calll	_realloc
	movl	%eax, %ecx
	movl	8(%ebp), %eax
	movl	%ecx, (%eax)
	movl	8(%ebp), %eax
	cmpl	$0, (%eax)
	jne	LBB3_3
# %bb.2:
	movl	$-1, -4(%ebp)
	jmp	LBB3_5
LBB3_3:
	jmp	LBB3_4
LBB3_4:
	movl	12(%ebp), %edx
	movl	8(%ebp), %ecx
	movl	(%ecx), %eax
	movl	4(%ecx), %ecx
	movl	%edx, (%eax,%ecx,8)
	movl	$0, 4(%eax,%ecx,8)
	movl	8(%ebp), %eax
	movl	4(%eax), %ecx
	addl	$1, %ecx
	movl	%ecx, 4(%eax)
	movl	$1, -4(%ebp)
LBB3_5:
	movl	-4(%ebp), %eax
	addl	$12, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_pop;
	.scl	2;
	.type	32;
	.endef
	.globl	_pop                            # -- Begin function pop
	.p2align	4, 0x90
_pop:                                   # @pop
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$12, %esp
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	cmpl	$0, 4(%eax)
	jne	LBB4_2
# %bb.1:
	leal	"??_C@_0CA@GOFJJJAI@cannot?5pop?5from?5an?5empty?5vector?$AA@", %eax
	movl	%eax, (%esp)
	calll	_printf
	movl	$-1, -4(%ebp)
	jmp	LBB4_3
LBB4_2:
	movl	8(%ebp), %eax
	movl	4(%eax), %ecx
	decl	%ecx
	movl	%ecx, 4(%eax)
	movl	8(%ebp), %ecx
	movl	(%ecx), %eax
	movl	4(%ecx), %ecx
	movl	(%eax,%ecx,8), %eax
	movl	%eax, -8(%ebp)
	movl	-8(%ebp), %eax
	movl	%eax, -4(%ebp)
LBB4_3:
	movl	-4(%ebp), %eax
	addl	$12, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_size;
	.scl	2;
	.type	32;
	.endef
	.globl	_size                           # -- Begin function size
	.p2align	4, 0x90
_size:                                  # @size
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	4(%eax), %eax
	popl	%ebp
	retl
                                        # -- End function
	.def	_capacity;
	.scl	2;
	.type	32;
	.endef
	.globl	_capacity                       # -- Begin function capacity
	.p2align	4, 0x90
_capacity:                              # @capacity
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	8(%eax), %eax
	popl	%ebp
	retl
                                        # -- End function
	.def	_print_vector;
	.scl	2;
	.type	32;
	.endef
	.globl	_print_vector                   # -- Begin function print_vector
	.p2align	4, 0x90
_print_vector:                          # @print_vector
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$16, %esp
	movl	8(%ebp), %eax
	movl	$0, -4(%ebp)
LBB7_1:                                 # =>This Inner Loop Header: Depth=1
	movl	-4(%ebp), %eax
	movl	8(%ebp), %ecx
	cmpl	4(%ecx), %eax
	jae	LBB7_4
# %bb.2:                                #   in Loop: Header=BB7_1 Depth=1
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	-4(%ebp), %edx
	movl	(%eax,%edx,8), %ecx
	movl	4(%eax,%edx,8), %edx
	movl	%esp, %eax
	movl	%edx, 8(%eax)
	movl	%ecx, 4(%eax)
	movl	$"??_C@_05JMLLFKBP@?$CFllu?6?$AA@", (%eax)
	calll	_printf
# %bb.3:                                #   in Loop: Header=BB7_1 Depth=1
	movl	-4(%ebp), %eax
	addl	$1, %eax
	movl	%eax, -4(%ebp)
	jmp	LBB7_1
LBB7_4:
	addl	$16, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_free_vector;
	.scl	2;
	.type	32;
	.endef
	.globl	_free_vector                    # -- Begin function free_vector
	.p2align	4, 0x90
_free_vector:                           # @free_vector
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	pushl	%eax
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	8(%ebp), %eax
	movl	%eax, (%esp)
	calll	_free
	addl	$4, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_init_ptr_vector;
	.scl	2;
	.type	32;
	.endef
	.globl	_init_ptr_vector                # -- Begin function init_ptr_vector
	.p2align	4, 0x90
_init_ptr_vector:                       # @init_ptr_vector
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$12, %esp
	movl	8(%ebp), %eax
	movl	$12, (%esp)
	calll	_malloc
	movl	%eax, -8(%ebp)
	movl	8(%ebp), %eax
	shll	$2, %eax
	movl	%eax, (%esp)
	calll	_malloc
	movl	%eax, %ecx
	movl	-8(%ebp), %eax
	movl	%ecx, (%eax)
	cmpl	$0, -8(%ebp)
	jne	LBB9_2
# %bb.1:
	leal	"??_C@_0BI@BBHHCLPP@ptr?5vector?5fail?5to?5init?$AA@", %eax
	movl	%eax, (%esp)
	calll	_printf
	movl	-8(%ebp), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	$0, -4(%ebp)
	jmp	LBB9_3
LBB9_2:
	movl	-8(%ebp), %eax
	movl	$0, 4(%eax)
	movl	8(%ebp), %ecx
	movl	-8(%ebp), %eax
	movl	%ecx, 8(%eax)
	movl	-8(%ebp), %eax
	movl	%eax, -4(%ebp)
LBB9_3:
	movl	-4(%ebp), %eax
	addl	$12, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_push_pv;
	.scl	2;
	.type	32;
	.endef
	.globl	_push_pv                        # -- Begin function push_pv
	.p2align	4, 0x90
_push_pv:                               # @push_pv
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$12, %esp
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	4(%eax), %eax
	movl	8(%ebp), %ecx
	cmpl	8(%ecx), %eax
	jne	LBB10_4
# %bb.1:
	movl	8(%ebp), %eax
	movl	8(%eax), %ecx
	shll	$1, %ecx
	movl	%ecx, 8(%eax)
	movl	8(%ebp), %eax
	movl	8(%eax), %eax
	shll	$2, %eax
	movl	8(%ebp), %ecx
	movl	(%ecx), %ecx
	movl	%ecx, (%esp)
	movl	%eax, 4(%esp)
	calll	_realloc
	movl	%eax, %ecx
	movl	8(%ebp), %eax
	movl	%ecx, (%eax)
	cmpl	$0, 8(%ebp)
	jne	LBB10_3
# %bb.2:
	movl	$-1, -4(%ebp)
	jmp	LBB10_5
LBB10_3:
	jmp	LBB10_4
LBB10_4:
	movl	12(%ebp), %edx
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	8(%ebp), %ecx
	movl	4(%ecx), %ecx
	movl	%edx, (%eax,%ecx,4)
	movl	8(%ebp), %eax
	movl	4(%eax), %ecx
	addl	$1, %ecx
	movl	%ecx, 4(%eax)
	movl	$1, -4(%ebp)
LBB10_5:
	movl	-4(%ebp), %eax
	addl	$12, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_pop_pv;
	.scl	2;
	.type	32;
	.endef
	.globl	_pop_pv                         # -- Begin function pop_pv
	.p2align	4, 0x90
_pop_pv:                                # @pop_pv
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$12, %esp
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	cmpl	$0, 4(%eax)
	jne	LBB11_2
# %bb.1:
	leal	"??_C@_0BH@EJLMGCBG@pop?5from?5an?5empty?5list?$AA@", %eax
	movl	%eax, (%esp)
	calll	_printf
	movl	$0, -4(%ebp)
	jmp	LBB11_3
LBB11_2:
	movl	8(%ebp), %eax
	movl	4(%eax), %ecx
	addl	$-1, %ecx
	movl	%ecx, 4(%eax)
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	8(%ebp), %ecx
	movl	4(%ecx), %ecx
	movl	(%eax,%ecx,4), %eax
	movl	%eax, -8(%ebp)
	movl	-8(%ebp), %eax
	movl	%eax, -4(%ebp)
LBB11_3:
	movl	-4(%ebp), %eax
	addl	$12, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_size_pv;
	.scl	2;
	.type	32;
	.endef
	.globl	_size_pv                        # -- Begin function size_pv
	.p2align	4, 0x90
_size_pv:                               # @size_pv
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	4(%eax), %eax
	popl	%ebp
	retl
                                        # -- End function
	.def	_capacity_pv;
	.scl	2;
	.type	32;
	.endef
	.globl	_capacity_pv                    # -- Begin function capacity_pv
	.p2align	4, 0x90
_capacity_pv:                           # @capacity_pv
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	movl	8(%ebp), %eax
	movl	8(%ebp), %eax
	movl	8(%eax), %eax
	popl	%ebp
	retl
                                        # -- End function
	.def	_free_ptr_vector;
	.scl	2;
	.type	32;
	.endef
	.globl	_free_ptr_vector                # -- Begin function free_ptr_vector
	.p2align	4, 0x90
_free_ptr_vector:                       # @free_ptr_vector
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$8, %esp
	movl	8(%ebp), %eax
	movl	$0, -4(%ebp)
LBB14_1:                                # =>This Inner Loop Header: Depth=1
	movl	-4(%ebp), %eax
	movl	8(%ebp), %ecx
	cmpl	4(%ecx), %eax
	jae	LBB14_4
# %bb.2:                                #   in Loop: Header=BB14_1 Depth=1
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	-4(%ebp), %ecx
	movl	(%eax,%ecx,4), %eax
	movl	%eax, (%esp)
	calll	_free
# %bb.3:                                #   in Loop: Header=BB14_1 Depth=1
	movl	-4(%ebp), %eax
	addl	$1, %eax
	movl	%eax, -4(%ebp)
	jmp	LBB14_1
LBB14_4:
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	8(%ebp), %eax
	movl	%eax, (%esp)
	calll	_free
	addl	$8, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_init_hashmap;
	.scl	2;
	.type	32;
	.endef
	.globl	_init_hashmap                   # -- Begin function init_hashmap
	.p2align	4, 0x90
_init_hashmap:                          # @init_hashmap
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$12, %esp
	movl	$8, (%esp)
	calll	_malloc
	movl	%eax, -4(%ebp)
	movl	-4(%ebp), %eax
	movl	$1000, 4(%eax)                  # imm = 0x3E8
	movl	$4000, (%esp)                   # imm = 0xFA0
	calll	_malloc
	movl	%eax, %ecx
	movl	-4(%ebp), %eax
	movl	%ecx, (%eax)
	movl	$0, -8(%ebp)
LBB15_1:                                # =>This Inner Loop Header: Depth=1
	cmpl	$1000, -8(%ebp)                 # imm = 0x3E8
	jae	LBB15_4
# %bb.2:                                #   in Loop: Header=BB15_1 Depth=1
	movl	-4(%ebp), %eax
	movl	(%eax), %eax
	movl	-8(%ebp), %ecx
	movl	$0, (%eax,%ecx,4)
# %bb.3:                                #   in Loop: Header=BB15_1 Depth=1
	movl	-8(%ebp), %eax
	addl	$1, %eax
	movl	%eax, -8(%ebp)
	jmp	LBB15_1
LBB15_4:
	movl	-4(%ebp), %eax
	addl	$12, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_hash;
	.scl	2;
	.type	32;
	.endef
	.globl	_hash                           # -- Begin function hash
	.p2align	4, 0x90
_hash:                                  # @hash
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	pushl	%eax
	movl	8(%ebp), %eax
	movl	$0, -4(%ebp)
LBB16_1:                                # =>This Inner Loop Header: Depth=1
	movl	8(%ebp), %eax
	cmpb	$0, (%eax)
	je	LBB16_3
# %bb.2:                                #   in Loop: Header=BB16_1 Depth=1
	movl	-4(%ebp), %eax
	movl	8(%ebp), %ecx
	movsbl	(%ecx), %ecx
	addl	%ecx, %eax
	movl	$1000, %ecx                     # imm = 0x3E8
	xorl	%edx, %edx
	divl	%ecx
	movl	%edx, -4(%ebp)
	movl	8(%ebp), %eax
	addl	$1, %eax
	movl	%eax, 8(%ebp)
	jmp	LBB16_1
LBB16_3:
	movl	-4(%ebp), %eax
	addl	$4, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_insert;
	.scl	2;
	.type	32;
	.endef
	.globl	_insert                         # -- Begin function insert
	.p2align	4, 0x90
_insert:                                # @insert
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$20, %esp
	movl	16(%ebp), %eax
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
	movl	12(%ebp), %eax
	movl	%eax, (%esp)
	calll	_hash
	movl	%eax, -4(%ebp)
	movl	$12, (%esp)
	calll	_malloc
	movl	%eax, -8(%ebp)
	movl	12(%ebp), %eax
	movl	%eax, (%esp)
	calll	__strdup
	movl	%eax, %ecx
	movl	-8(%ebp), %eax
	movl	%ecx, (%eax)
	movl	16(%ebp), %ecx
	movl	-8(%ebp), %eax
	movl	%ecx, 4(%eax)
	movl	-8(%ebp), %eax
	movl	$0, 8(%eax)
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	-4(%ebp), %ecx
	shll	$2, %ecx
	addl	%ecx, %eax
	movl	%eax, -12(%ebp)
	movl	-12(%ebp), %eax
	cmpl	$0, (%eax)
	jne	LBB17_2
# %bb.1:
	movl	-8(%ebp), %ecx
	movl	-12(%ebp), %eax
	movl	%ecx, (%eax)
	jmp	LBB17_8
LBB17_2:
	jmp	LBB17_3
LBB17_3:                                # =>This Inner Loop Header: Depth=1
	movl	-12(%ebp), %eax
	cmpl	$0, (%eax)
	je	LBB17_7
# %bb.4:                                #   in Loop: Header=BB17_3 Depth=1
	movl	12(%ebp), %edx
	movl	-12(%ebp), %eax
	movl	(%eax), %eax
	movl	(%eax), %ecx
	movl	%esp, %eax
	movl	%edx, 4(%eax)
	movl	%ecx, (%eax)
	calll	_strcmp
	cmpl	$0, %eax
	jne	LBB17_6
# %bb.5:
	movl	16(%ebp), %ecx
	movl	-12(%ebp), %eax
	movl	(%eax), %eax
	movl	%ecx, 4(%eax)
	movl	-8(%ebp), %eax
	movl	(%eax), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	-8(%ebp), %eax
	movl	4(%eax), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	-8(%ebp), %eax
	movl	%eax, (%esp)
	calll	_free
	jmp	LBB17_8
LBB17_6:                                #   in Loop: Header=BB17_3 Depth=1
	movl	-12(%ebp), %eax
	movl	(%eax), %eax
	addl	$8, %eax
	movl	%eax, -12(%ebp)
	jmp	LBB17_3
LBB17_7:
	movl	-8(%ebp), %ecx
	movl	-12(%ebp), %eax
	movl	%ecx, (%eax)
LBB17_8:
	addl	$20, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_find_val;
	.scl	2;
	.type	32;
	.endef
	.globl	_find_val                       # -- Begin function find_val
	.p2align	4, 0x90
_find_val:                              # @find_val
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$20, %esp
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
	movl	12(%ebp), %eax
	movl	%eax, (%esp)
	calll	_hash
	movl	%eax, -8(%ebp)
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	-8(%ebp), %ecx
	movl	(%eax,%ecx,4), %eax
	movl	%eax, -12(%ebp)
LBB18_1:                                # =>This Inner Loop Header: Depth=1
	cmpl	$0, -12(%ebp)
	je	LBB18_5
# %bb.2:                                #   in Loop: Header=BB18_1 Depth=1
	movl	12(%ebp), %edx
	movl	-12(%ebp), %eax
	movl	(%eax), %ecx
	movl	%esp, %eax
	movl	%edx, 4(%eax)
	movl	%ecx, (%eax)
	calll	_strcmp
	cmpl	$0, %eax
	jne	LBB18_4
# %bb.3:
	movl	-12(%ebp), %eax
	movl	4(%eax), %eax
	movl	%eax, -4(%ebp)
	jmp	LBB18_6
LBB18_4:                                #   in Loop: Header=BB18_1 Depth=1
	movl	-12(%ebp), %eax
	movl	8(%eax), %eax
	movl	%eax, -12(%ebp)
	jmp	LBB18_1
LBB18_5:
	movl	$0, -4(%ebp)
LBB18_6:
	movl	-4(%ebp), %eax
	addl	$20, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_freeHashMap;
	.scl	2;
	.type	32;
	.endef
	.globl	_freeHashMap                    # -- Begin function freeHashMap
	.p2align	4, 0x90
_freeHashMap:                           # @freeHashMap
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$16, %esp
	movl	8(%ebp), %eax
	movl	$0, -4(%ebp)
LBB19_1:                                # =>This Loop Header: Depth=1
                                        #     Child Loop BB19_3 Depth 2
	movl	-4(%ebp), %eax
	movl	8(%ebp), %ecx
	cmpl	4(%ecx), %eax
	jae	LBB19_7
# %bb.2:                                #   in Loop: Header=BB19_1 Depth=1
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	-4(%ebp), %ecx
	movl	(%eax,%ecx,4), %eax
	movl	%eax, -8(%ebp)
LBB19_3:                                #   Parent Loop BB19_1 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	cmpl	$0, -8(%ebp)
	je	LBB19_5
# %bb.4:                                #   in Loop: Header=BB19_3 Depth=2
	movl	-8(%ebp), %eax
	movl	%eax, -12(%ebp)
	movl	-8(%ebp), %eax
	movl	8(%eax), %eax
	movl	%eax, -8(%ebp)
	movl	-12(%ebp), %eax
	movl	(%eax), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	-12(%ebp), %eax
	movl	4(%eax), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	-12(%ebp), %eax
	movl	%eax, (%esp)
	calll	_free
	jmp	LBB19_3
LBB19_5:                                #   in Loop: Header=BB19_1 Depth=1
	jmp	LBB19_6
LBB19_6:                                #   in Loop: Header=BB19_1 Depth=1
	movl	-4(%ebp), %eax
	addl	$1, %eax
	movl	%eax, -4(%ebp)
	jmp	LBB19_1
LBB19_7:
	movl	8(%ebp), %eax
	movl	(%eax), %eax
	movl	%eax, (%esp)
	calll	_free
	movl	8(%ebp), %eax
	movl	%eax, (%esp)
	calll	_free
	addl	$16, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_read_file_data;
	.scl	2;
	.type	32;
	.endef
	.globl	_read_file_data                 # -- Begin function read_file_data
	.p2align	4, 0x90
_read_file_data:                        # @read_file_data
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$20, %esp
	movl	8(%ebp), %eax
	movl	8(%ebp), %ecx
	leal	-4(%ebp), %edx
	leal	"??_C@_01KDCPPGHE@r?$AA@", %eax
	movl	%edx, (%esp)
	movl	%ecx, 4(%esp)
	movl	%eax, 8(%esp)
	calll	_fopen_s
	movl	%eax, -8(%ebp)
	cmpl	$0, -8(%ebp)
	je	LBB20_2
# %bb.1:
	movl	-8(%ebp), %eax
	leal	"??_C@_0CJ@JJOKGIII@Failed?5to?5open?5the?5file?4?5Error?5c@", %ecx
	movl	%ecx, (%esp)
	movl	%eax, 4(%esp)
	calll	_printf
	movl	$1, (%esp)
	calll	_exit
LBB20_2:
	movl	-4(%ebp), %eax
	addl	$20, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	_split_string;
	.scl	2;
	.type	32;
	.endef
	.globl	_split_string                   # -- Begin function split_string
	.p2align	4, 0x90
_split_string:                          # @split_string
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$24, %esp
	movl	16(%ebp), %eax
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
	movl	$0, -4(%ebp)
	movl	$0, -8(%ebp)
	movl	12(%ebp), %ecx
	movl	16(%ebp), %edx
	leal	-8(%ebp), %eax
	movl	%edx, (%esp)
	movl	%ecx, 4(%esp)
	movl	%eax, 8(%esp)
	calll	_strtok_s
	movl	%eax, -4(%ebp)
LBB21_1:                                # =>This Inner Loop Header: Depth=1
	cmpl	$0, -4(%ebp)
	je	LBB21_3
# %bb.2:                                #   in Loop: Header=BB21_1 Depth=1
	movl	-4(%ebp), %eax
	movl	%eax, (%esp)
	calll	_atoi
	movl	%eax, -12(%ebp)
	movl	-12(%ebp), %eax
	movl	8(%ebp), %ecx
	movl	%ecx, (%esp)
	movl	%eax, 4(%esp)
	calll	_push
	movl	12(%ebp), %ecx
	xorl	%eax, %eax
	leal	-8(%ebp), %eax
	movl	$0, (%esp)
	movl	%ecx, 4(%esp)
	movl	%eax, 8(%esp)
	calll	_strtok_s
	movl	%eax, -4(%ebp)
	jmp	LBB21_1
LBB21_3:
	movl	$1, %eax
	addl	$24, %esp
	popl	%ebp
	retl
                                        # -- End function
	.def	__vfprintf_l;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,__vfprintf_l
	.globl	__vfprintf_l                    # -- Begin function _vfprintf_l
	.p2align	4, 0x90
__vfprintf_l:                           # @_vfprintf_l
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	pushl	%ebx
	pushl	%edi
	pushl	%esi
	subl	$32, %esp
	movl	20(%ebp), %eax
	movl	16(%ebp), %eax
	movl	12(%ebp), %eax
	movl	8(%ebp), %eax
	movl	20(%ebp), %eax
	movl	%eax, -20(%ebp)                 # 4-byte Spill
	movl	16(%ebp), %ebx
	movl	12(%ebp), %edi
	movl	8(%ebp), %esi
	calll	___local_stdio_printf_options
	movl	-20(%ebp), %ecx                 # 4-byte Reload
	movl	(%eax), %edx
	movl	%edx, -16(%ebp)                 # 4-byte Spill
	movl	4(%eax), %edx
	movl	%esp, %eax
	movl	%ecx, 20(%eax)
	movl	-16(%ebp), %ecx                 # 4-byte Reload
	movl	%ebx, 16(%eax)
	movl	%edi, 12(%eax)
	movl	%esi, 8(%eax)
	movl	%edx, 4(%eax)
	movl	%ecx, (%eax)
	calll	___stdio_common_vfprintf
	addl	$32, %esp
	popl	%esi
	popl	%edi
	popl	%ebx
	popl	%ebp
	retl
                                        # -- End function
	.def	___local_stdio_printf_options;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,___local_stdio_printf_options
	.globl	___local_stdio_printf_options   # -- Begin function __local_stdio_printf_options
	.p2align	4, 0x90
___local_stdio_printf_options:          # @__local_stdio_printf_options
# %bb.0:
	pushl	%ebp
	movl	%esp, %ebp
	leal	___local_stdio_printf_options._OptionsStorage, %eax
	popl	%ebp
	retl
                                        # -- End function
	.section	.rdata,"dr",discard,"??_C@_0M@JBMNDFH@index?5error?$AA@"
	.globl	"??_C@_0M@JBMNDFH@index?5error?$AA@" # @"??_C@_0M@JBMNDFH@index?5error?$AA@"
"??_C@_0M@JBMNDFH@index?5error?$AA@":
	.asciz	"index error"

	.section	.rdata,"dr",discard,"??_C@_0CA@GOFJJJAI@cannot?5pop?5from?5an?5empty?5vector?$AA@"
	.globl	"??_C@_0CA@GOFJJJAI@cannot?5pop?5from?5an?5empty?5vector?$AA@" # @"??_C@_0CA@GOFJJJAI@cannot?5pop?5from?5an?5empty?5vector?$AA@"
"??_C@_0CA@GOFJJJAI@cannot?5pop?5from?5an?5empty?5vector?$AA@":
	.asciz	"cannot pop from an empty vector"

	.section	.rdata,"dr",discard,"??_C@_05JMLLFKBP@?$CFllu?6?$AA@"
	.globl	"??_C@_05JMLLFKBP@?$CFllu?6?$AA@" # @"??_C@_05JMLLFKBP@?$CFllu?6?$AA@"
"??_C@_05JMLLFKBP@?$CFllu?6?$AA@":
	.asciz	"%llu\n"

	.section	.rdata,"dr",discard,"??_C@_0BI@BBHHCLPP@ptr?5vector?5fail?5to?5init?$AA@"
	.globl	"??_C@_0BI@BBHHCLPP@ptr?5vector?5fail?5to?5init?$AA@" # @"??_C@_0BI@BBHHCLPP@ptr?5vector?5fail?5to?5init?$AA@"
"??_C@_0BI@BBHHCLPP@ptr?5vector?5fail?5to?5init?$AA@":
	.asciz	"ptr vector fail to init"

	.section	.rdata,"dr",discard,"??_C@_0BH@EJLMGCBG@pop?5from?5an?5empty?5list?$AA@"
	.globl	"??_C@_0BH@EJLMGCBG@pop?5from?5an?5empty?5list?$AA@" # @"??_C@_0BH@EJLMGCBG@pop?5from?5an?5empty?5list?$AA@"
"??_C@_0BH@EJLMGCBG@pop?5from?5an?5empty?5list?$AA@":
	.asciz	"pop from an empty list"

	.section	.rdata,"dr",discard,"??_C@_01KDCPPGHE@r?$AA@"
	.globl	"??_C@_01KDCPPGHE@r?$AA@"       # @"??_C@_01KDCPPGHE@r?$AA@"
"??_C@_01KDCPPGHE@r?$AA@":
	.asciz	"r"

	.section	.rdata,"dr",discard,"??_C@_0CJ@JJOKGIII@Failed?5to?5open?5the?5file?4?5Error?5c@"
	.globl	"??_C@_0CJ@JJOKGIII@Failed?5to?5open?5the?5file?4?5Error?5c@" # @"??_C@_0CJ@JJOKGIII@Failed?5to?5open?5the?5file?4?5Error?5c@"
"??_C@_0CJ@JJOKGIII@Failed?5to?5open?5the?5file?4?5Error?5c@":
	.asciz	"Failed to open the file. Error code: %d\n"

	.lcomm	___local_stdio_printf_options._OptionsStorage,8,8 # @__local_stdio_printf_options._OptionsStorage
	.addrsig
	.addrsig_sym _malloc
	.addrsig_sym _free
	.addrsig_sym _printf
	.addrsig_sym _push
	.addrsig_sym _realloc
	.addrsig_sym _hash
	.addrsig_sym __strdup
	.addrsig_sym _strcmp
	.addrsig_sym _fopen_s
	.addrsig_sym _exit
	.addrsig_sym _strtok_s
	.addrsig_sym _atoi
	.addrsig_sym __vfprintf_l
	.addrsig_sym ___acrt_iob_func
	.addrsig_sym ___stdio_common_vfprintf
	.addrsig_sym ___local_stdio_printf_options
	.addrsig_sym ___local_stdio_printf_options._OptionsStorage
