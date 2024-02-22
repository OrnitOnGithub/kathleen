# Memory allocation

Static variables will be preallocated.

Non-static variables can have their pointers preallocated.

Dynamically allocated variables (lists) are yet to be figured out.

<br>

Pointers can also be preallocated

The stack can be used.

Rust for example stores the pointers to the heap in the stack.

Holy mother of god the stack pointer seems to be manipulatable
```asm
; Some code that pushes values onto the stack

    ; Move the stack pointer to access the 11th value
    mov rax, 10          ; Index of the 11th value (zero-based)
    mov rbx, 8           ; Size of each value (assuming 64-bit architecture)
    imul rax, rbx        ; Calculate the offset
    sub rsp, rax         ; Adjust the stack pointer

    ; Now you can access the value at [rsp]

    ; Example: print the value at [rsp]
    mov rdi, 1            ; file descriptor (stdout)
    mov rsi, rsp          ; pointer to the value
    mov rdx, 8            ; size of the value (assuming 64-bit)
    mov rax, 1            ; sys_write system call number
    syscall

    ; Restore the stack pointer to its original position
    add rsp, rax
```

## Functional let-drop example

```asm
section .data
    hello db "Hello world", 0, 10

section .text
    extern malloc, free
    global main

main:
    ; Allocate memory for 10 bytes
    mov rdi, 10                   ; Set the first argument for malloc to 10 bytes
    mov rax, 8                    ; Set the second argument for malloc to 8 (size of each element)
    imul rdi, rax                 ; Calculate total bytes to allocate (10 * 8 = 80)
    call malloc                   ; Invoke malloc to allocate memory
    mov rbx, rax                  ; Store the allocated memory address in rbx

    ; Store the value 123 at the memory address
    mov rax, rbx                  ; Load rbx into rax for memory addressing
    mov qword [rax], 123          ; Store the value 123 at the memory address pointed by rbx

    ; Move to the next memory location and store 456
    mov rax, rbx                  ; Load rbx into rax for memory addressing
    add rax, 8                    ; Move to the next qword location (8 bytes ahead)
    mov qword [rax], 456          ; Store the value 456 at the memory address pointed by rbx + 8

    ; Push the value of rbx onto the stack
    push rbx                      ; Push the value stored in rbx onto the stack

    ; Pop the top value from the stack into rbx
    pop rbx                       ; Pop the top value from the stack into rbx

    ; Free the allocated memory
    mov rdi, rbx                  ; Set the argument for free to the address stored in rbx
    call free                     ; Invoke free to deallocate the memory

    ; Exit the program
    mov rax, 60                   ; Set up the syscall number for exit
    mov rdi, 0                    ; Set up the exit status
    syscall                       ; Make syscall to exit the program
```


## Let

Language

```
let variable = text
```

Assembly

```asm

```

## Drop

Language

```
drop variable
```

Assembly

```asm

```
