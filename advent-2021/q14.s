	.text
	.def	@feat.00;
	.scl	3;
	.type	0;
	.endef
	.globl	@feat.00
.set @feat.00, 0
	.file	"q14.c"
	.def	parse_polymer;
	.scl	2;
	.type	32;
	.endef
	.globl	parse_polymer                   # -- Begin function parse_polymer
	.p2align	4, 0x90
parse_polymer:                          # @parse_polymer
.seh_proc parse_polymer
# %bb.0:
	subq	$40, %rsp
	.seh_stackalloc 40
	.seh_endprologue
	leaq	32(%rsp), %rcx
	xorl	%edx, %edx
	movl	$4, %r8d
	callq	memset
	movl	$10, 32(%rsp)
	movl	32(%rsp), %eax
	addq	$40, %rsp
	retq
	.seh_endproc
                                        # -- End function
	.def	solution;
	.scl	2;
	.type	32;
	.endef
	.globl	solution                        # -- Begin function solution
	.p2align	4, 0x90
solution:                               # @solution
.seh_proc solution
# %bb.0:
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movq	%rcx, 48(%rsp)
	callq	parse_polymer
	movl	%eax, 40(%rsp)
	movl	40(%rsp), %edx
	leaq	"??_C@_03PMGGPEJJ@?$CFd?6?$AA@"(%rip), %rcx
	callq	printf
	nop
	addq	$56, %rsp
	retq
	.seh_endproc
                                        # -- End function
	.def	printf;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,printf
	.globl	printf                          # -- Begin function printf
	.p2align	4, 0x90
printf:                                 # @printf
.seh_proc printf
# %bb.0:
	subq	$72, %rsp
	.seh_stackalloc 72
	.seh_endprologue
	movq	%r9, 104(%rsp)
	movq	%r8, 96(%rsp)
	movq	%rdx, 88(%rsp)
	movq	%rcx, 64(%rsp)
	leaq	88(%rsp), %rax
	movq	%rax, 48(%rsp)
	movq	48(%rsp), %rax
	movq	%rax, 40(%rsp)                  # 8-byte Spill
	movq	64(%rsp), %rax
	movq	%rax, 32(%rsp)                  # 8-byte Spill
	movl	$1, %ecx
	callq	__acrt_iob_func
	movq	32(%rsp), %rdx                  # 8-byte Reload
	movq	40(%rsp), %r9                   # 8-byte Reload
	movq	%rax, %rcx
	xorl	%eax, %eax
	movl	%eax, %r8d
	callq	_vfprintf_l
	movl	%eax, 60(%rsp)
	movl	60(%rsp), %eax
	addq	$72, %rsp
	retq
	.seh_endproc
                                        # -- End function
	.def	main;
	.scl	2;
	.type	32;
	.endef
	.text
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
main:                                   # @main
.seh_proc main
# %bb.0:
	subq	$56, %rsp
	.seh_stackalloc 56
	.seh_endprologue
	movl	$0, 52(%rsp)
	leaq	"??_C@_0BB@MBIANBFD@?4?1inputs?1q14?4txt?$AA@"(%rip), %rcx
	callq	read_file_data
	movq	%rax, 40(%rsp)
	movq	40(%rsp), %rcx
	callq	solution
	movq	40(%rsp), %rcx
	callq	fclose
	xorl	%eax, %eax
	addq	$56, %rsp
	retq
	.seh_endproc
                                        # -- End function
	.def	_vfprintf_l;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,_vfprintf_l
	.globl	_vfprintf_l                     # -- Begin function _vfprintf_l
	.p2align	4, 0x90
_vfprintf_l:                            # @_vfprintf_l
.seh_proc _vfprintf_l
# %bb.0:
	subq	$104, %rsp
	.seh_stackalloc 104
	.seh_endprologue
	movq	%r9, 96(%rsp)
	movq	%r8, 88(%rsp)
	movq	%rdx, 80(%rsp)
	movq	%rcx, 72(%rsp)
	movq	96(%rsp), %rax
	movq	%rax, 64(%rsp)                  # 8-byte Spill
	movq	88(%rsp), %rax
	movq	%rax, 56(%rsp)                  # 8-byte Spill
	movq	80(%rsp), %rax
	movq	%rax, 48(%rsp)                  # 8-byte Spill
	movq	72(%rsp), %rax
	movq	%rax, 40(%rsp)                  # 8-byte Spill
	callq	__local_stdio_printf_options
	movq	40(%rsp), %rdx                  # 8-byte Reload
	movq	48(%rsp), %r8                   # 8-byte Reload
	movq	56(%rsp), %r9                   # 8-byte Reload
	movq	%rax, %rcx
	movq	64(%rsp), %rax                  # 8-byte Reload
	movq	(%rcx), %rcx
	movq	%rax, 32(%rsp)
	callq	__stdio_common_vfprintf
	nop
	addq	$104, %rsp
	retq
	.seh_endproc
                                        # -- End function
	.def	__local_stdio_printf_options;
	.scl	2;
	.type	32;
	.endef
	.section	.text,"xr",discard,__local_stdio_printf_options
	.globl	__local_stdio_printf_options    # -- Begin function __local_stdio_printf_options
	.p2align	4, 0x90
__local_stdio_printf_options:           # @__local_stdio_printf_options
# %bb.0:
	leaq	__local_stdio_printf_options._OptionsStorage(%rip), %rax
	retq
                                        # -- End function
	.section	.rdata,"dr",discard,"??_C@_03PMGGPEJJ@?$CFd?6?$AA@"
	.globl	"??_C@_03PMGGPEJJ@?$CFd?6?$AA@" # @"??_C@_03PMGGPEJJ@?$CFd?6?$AA@"
"??_C@_03PMGGPEJJ@?$CFd?6?$AA@":
	.asciz	"%d\n"

	.section	.rdata,"dr",discard,"??_C@_0BB@MBIANBFD@?4?1inputs?1q14?4txt?$AA@"
	.globl	"??_C@_0BB@MBIANBFD@?4?1inputs?1q14?4txt?$AA@" # @"??_C@_0BB@MBIANBFD@?4?1inputs?1q14?4txt?$AA@"
"??_C@_0BB@MBIANBFD@?4?1inputs?1q14?4txt?$AA@":
	.asciz	"./inputs/q14.txt"

	.lcomm	__local_stdio_printf_options._OptionsStorage,8,8 # @__local_stdio_printf_options._OptionsStorage
	.addrsig
	.addrsig_sym parse_polymer
	.addrsig_sym solution
	.addrsig_sym printf
	.addrsig_sym read_file_data
	.addrsig_sym fclose
	.addrsig_sym _vfprintf_l
	.addrsig_sym __acrt_iob_func
	.addrsig_sym __stdio_common_vfprintf
	.addrsig_sym __local_stdio_printf_options
	.addrsig_sym __local_stdio_printf_options._OptionsStorage
