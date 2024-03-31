; Print a newline
mov         rdx,    1            ; length of the message
mov         rsi,    newline      ; pointer to the message (newline constant)
mov         rdi,    1
mov         rax,    1
syscall
