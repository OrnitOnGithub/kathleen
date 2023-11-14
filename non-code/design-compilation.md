# How the compilables get compiled

The compiler tokenises the code, then the code gets turned into an IR.

The IR must smoothly translate to assembly.
```
data:
    constant
    static
    statically-sized

main:
    expression
    expression
    [...]

    condition:
        if:
            expression
        else:
            expression
 
    exit
```