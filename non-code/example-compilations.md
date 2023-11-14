language:
```c
const a: int = 7

main() {
    print(a); print(a);
    print(a);
}
```
assembly:
```asm
section .data
    whatever

section .text
global main

main:
    print
    print
    print

    exit syscall
```