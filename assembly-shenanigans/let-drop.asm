section .data
	text db "Hello", 0

section .bss
    variable resq 1  ; Reserve space for a 64-bit variable

section .text
    global _start

_start:
	; (in the language) let is used
    ; Initialize the variable with a value
    mov qword [variable], text  ; qword = 8 bytes, word = 2 bytes for example.
    							; To store 2 words for example, we would then
    							; need to use [variable + x]

    ; Print the value
    mov rax, 1      	; syscall number for sys_write (1)
    mov rdi, 1      	; File descriptor for stdout (1)
    mov rsi, [variable] ; Load the address of the format string
    mov rdx, 6     		; Length of the string
    syscall

	; (in the language) drop is used
    ; manually deallocate the memory
    mov rax, 11     ; syscall number for sys_munmap (11)
    mov rdi, [variable]  ; Pointer to the variable
    mov rsi, 6 
    syscall

    ; print the value - this time we get a segfault!
    mov rax, 1
    mov rdi, 1
    mov rsi, [variable]
    mov rdx, 6
    syscall

    ; exit the program
    mov rax, 60 ; syscall number for sys_exit (60)
    mov rdi, 0  ; return code (0)
    syscall
