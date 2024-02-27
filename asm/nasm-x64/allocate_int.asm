mov rdi, 8 ; ask for a qword (8 bytes)
call malloc ; call malloc and recieve pointer in RAX
mov [<>], rax ; move that pointer in the memory at the BSS pointer
mov qword [rax], <> ; put the value in the memory area