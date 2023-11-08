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
