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

## Main function

```c
main(parameter: int) {
    // this program prints whatever parameter 
    // it is given when ran in the coneolse
    print(parameter)
}
```
