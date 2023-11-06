section .data
    ; Define a variable to hold the memory address
    variable_ptr dq 0

section .text
global _start

_start:
    ; Allocate memory for the variable
    mov rdi, 13         ; Length of the string "Hello, world"
    mov rax, 0x9       ; syscall number for brk (heap allocation)
    syscall
    mov [variable_ptr], rax  ; Store the allocated memory address in variable_ptr

    ; Copy the string "Hello, world" to the allocated memory
    mov rdi, rax
    mov rsi, hello
    mov rcx, 13
    rep movsb



    ; Free the memory (drop the variable)

    mov rax, [variable_ptr]
    mov rdi, rax
    mov rax, 0xA       ; syscall number for brk (heap deallocation)
    syscall

    ; Exit the program
    mov rax, 60         ; syscall number for exit
    xor rdi, rdi        ; exit status (0)
    syscall

section .data
hello db "Hello, world", 0