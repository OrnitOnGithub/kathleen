; Increment an integer
mov         rax,    <>      ; integer to increment
mov         rbx,    [rax]
mov         rcx,    [rbx]
inc         rcx
mov         [rbx],  rcx
