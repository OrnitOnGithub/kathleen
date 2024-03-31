section .bss
  buffer resb 39 ; 38 characters for the number plus the null terminator

section .text
  global main

main:
  mov     rax, -1
  mov     rsi, rax
  mov     rdi, buffer + 38
  call    int_to_str

  ; print the string
  mov     rsi, buffer
  mov     rdx, 39
  mov     rax, 1
  mov     rdi, 1
  ; exit
  syscall
  mov     rax, 60
  mov     rdi, 0
  syscall

; Parameters:
;   rsi and rax : integer
;   rdi         : pointer to buffer + 38
;
; Example usage:
;   mov     rax, 1234 ; some integer
;   mov     rsi, rax
;   mov     rdi, buffer + 38 ; a 39 byte buffer, 
;                            ; but give the pointer to the before-last byte
;                            ; (last is null terminator)
;   call    int_to_str
;
; Returns
;   nothing, modifies buffer given in rdi.
uint64_to_str:
  mov     rdx, 0
  mov     rbx, 10
  div     rbx
  ; convert the remainder of the integer to ASCII by adding '0' to make it a character
  add     dl, '0'
  mov     [rdi], dl
  dec     rdi
  cmp     rax, 0
  jg      int_to_str
  mov     byte [rdi], 0
  ret

