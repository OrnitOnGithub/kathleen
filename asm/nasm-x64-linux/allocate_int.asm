; Allocate a 64 bit unsigned integer.
mov         rdi,    8       ; qword (8 bytes) as parameter for malloc
call malloc                 ; call malloc and recieve pointer in RAX
mov         [<>],   rax     ; move that pointer in the memory at the BSS pointer
mov qword   [rax],  <>      ; put the value in the memory area
