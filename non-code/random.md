### Print

Language input
```
print("Hello")
```

Assembly output
```asm
# Allocate memory for the string
mov rax, 14  # Size of the string
mov rdi, 0   # Request address of end of data segment (brk)
syscall

# Store the string in the allocated memory
mov rsi, rax      # rsi now points to the allocated memory
mov rdi, my_data  # Source address of your string
mov rcx, 12       # Length of the string
rep movsb         # Copy the string


mov rax, 1
mov rdi, 1
mov rsi, text
mov rdx, 14
syscall
```


### Exit

Language input
```
quit
```

Assembly output
```asm
mov rax 60
mov rdi 0
syscall
```

### Variables

Save this for later
```asm
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

    ; Use the variable
    ; ...

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
```
