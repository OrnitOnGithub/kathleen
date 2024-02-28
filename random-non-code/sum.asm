section .data
    x db '5' ; define byte 5 and 3
    y db '3'
    msg db  "sum of x and y is "
    len equ $ - msg

segment .bss
    sum resb 1 ; reserve one byte for an u8 (0-255)

section .text

global _start

_start:

    mov     eax, [x]
    sub     eax, '0'
    mov     ebx, [y]
    sub     ebx, '0'
    add     eax, ebx
    add     eax, '0'

    mov     [sum], eax

    mov     ecx, msg
    mov     edx, len
    mov     ebx, 1
    mov     eax, 4
    int     0x80

    mov     ecx, sum
    mov     edx, 1
    mov     ebx, 1
    mov     eax, 4
    int     0x80

    mov     eax, 1
    int     0x80