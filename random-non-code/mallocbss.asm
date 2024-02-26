; this program returns nothing, debug it and look at
; registers to see the interesting stuff

section .bss
	pointer resq 1

section .data
    extern malloc
    fail_msg db "Malloc failed.", 10, 0
	sentence db "Hello world, this is a long sentence", 10, 0
	const10: dq 10
	
section .text
    global main

main:
    mov rdi, 16 ; bytes to allocate
    call malloc

    ; check if malloc returned a valid pointer
    test rax, rax  		 ; test if rax is zero
    jz malloc_failed  	 ; If zero, jump to malloc_failed label


 	; now we have a pointer inside rax

 	mov qword [rax], 42
 	mov qword [rax+8], 43

	;mov rdi, [rax] ; into rdi we have contents of pointer
	;mov rsi, [rax+8]

	mov [pointer], rax ;move the pointer in rax to the mem pointed by pointer1
	mov rdi, [pointer] ; now the pointer should be in rdi
	mov rsi, [rdi]
	
	

	;mov rdx, 8
	;mov rsi, [rax]
	;mov rdi, 1
	;mov rax, 1
	;syscall

    ; Exit the program
    mov rdi, 0
    mov rax, 60
    syscall


malloc_failed:
	mov rdx, 17
	mov rsi, fail_msg
	mov rdi, 1
	mov rax, 1
	ret
