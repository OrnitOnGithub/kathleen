; Prints a constant string (defined in .data section)
mov         rdx,    [<>length]      ; length of the message
mov         rsi,    <>              ; pointer to the message
mov         rdi,    1
mov         rax,    1
syscall
