mov     rax, <> ; The integer to print
mov rbx, [rax]
mov rax, [rbx]
mov     rsi, rax
mov     rdi, uint64_as_str_buffer + 38
call uint64_to_str

mov         rdx,    39
mov         rsi,    uint64_as_str_buffer
mov         rdi,    1
mov         rax,    1
syscall
