### Program structure

```
compileTime

fn main {
    runTime
}
```

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

