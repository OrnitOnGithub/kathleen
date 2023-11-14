# Simple stuff

## Print

Language

```
print("Hello")
```

Assembly

```asm
mov rax, 1            ; syscall number for sys_write (1)
mov rdi, 1            ; file descriptor (1 for some reason idk)
mov rsi, text         ; variable to print
mov rdx, <length>     ; length of thing in bytes
syscall
```


## Exit

Language

```
quit
```

Assembly

```asm
mov rax 60
mov rdi 0
syscall
```

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

## Let

Language

```
let variable = text
```

Assembly

```asm
section .data
	text db "Hello", 0          

section .bss
    variable resq 1     ; Reserve space for a 64-bit variable

_start:
    ; (in the language) let is used
    ; Initialize the variable with a value

    mov qword [variable], text  ; qword = 8 bytes, word = 2 bytes for example.
                                ; To store 2 words for example, we would
                                ; need to use [variable + x]
```

## Drop

Language

```
drop variable
```

Assembly

```asm
; (in the language) drop is used
; manually deallocate the memory

mov rax, 11             ; syscall number for sys_munmap (11)
mov rdi, [variable]     ; Pointer to the variable
mov rsi, 6 
syscall
```
