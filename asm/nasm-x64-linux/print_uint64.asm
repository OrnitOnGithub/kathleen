; Prints an unsigned 64 bit integer
mov     rax, <> ; The integer to print
mov rbx, [rax]
mov rax, [rbx]  ; Get the value by double-dereference
mov     rdi, uint64_as_str_buffer + 38
call uint64_to_str ; convert int to string, put in uint64_as_str_buffer
; print uint64_as_str_buffer, which now contains our int as strint.
mov         rdx,    39
mov         rsi,    uint64_as_str_buffer
mov         rdi,    1
mov         rax,    1
syscall
