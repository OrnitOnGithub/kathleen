	.file	"let-drop.c"
	.text
	.globl	main
	.type	main, @function
main:
.LFB6:
	.cfi_startproc
	pushq	%rbp
	.cfi_def_cfa_offset 16
	.cfi_offset 6, -16
	movq	%rsp, %rbp
	.cfi_def_cfa_register 6
	subq	$16, %rsp
	movl	$4, %edi
	call	malloc

	movq	%rax, -8(%rbp)
	movq	-8(%rbp), %rax
	movl	$42, (%rax)
	movq	-8(%rbp), %rax
	movq	%rax, %rdi
	call	free

	movl	$0, %eax
	leave
	.cfi_def_cfa 7, 8
	ret
	.cfi_endproc

.LFE6:
	.size	main, .-main
	.ident	"GCC: (GNU) 13.2.1 20231011 (Red Hat 13.2.1-4)"
	.section	.note.GNU-stack,"",@progbits
