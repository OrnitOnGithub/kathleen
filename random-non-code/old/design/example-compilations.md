### Something simple
code:
```c
static a: int = 7

main() {
    print(a); print(a);
    print(a);

    exit(1)
}
```
IR:
```
data
    stat int a 7
main
    print a
    print a
    print a
    exit 1
```
assembly:
```asm
section .data
    whatever

section .text
    global main

main:
    print syscall
    print syscall
    print syscall

    exit syscall
```

### A loop and a condition

code:
```c
static var: int = 7

main()
{
    loop
    {
        if (var == 7)
        {
            print(var);
        }
        else
        {
            print(var); // Since var is static this should never happen
            print(var);
        }
    }
}

```
IR:
```
data
    stat int a 7
main
    loopstart 1
        eval a 7
        true
            print a
        false
            print a
            print a
    loopend 1
```
Notes:
- `loopend <number>` jumps to `loopstart <number>`
- false should not even exist because a and 7 are both constants.
    the compiler should evaluate this equation.
- Hello