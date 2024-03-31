; ----------------
; STANDARD LIBRARY

uint64_to_str:
  ; Parameters:
  ;   rsi and rax : integer
  ;   rdi         : pointer to buffer + 38
  ;
  ; Example usage:
  ;   mov     rax, 1234 ; some integer
  ;   mov     rsi, rax
  ;   mov     rdi, uint_64_as_str_buffer + 38 ; a 39 byte buffer, 
  ;                            ; but give the pointer to the before-last byte
  ;                            ; (last is null terminator)
  ;   call    uint64_to_str
  ;
  ; Returns
  ;   nothing, modifies buffer given in rdi.
  mov     rdx, 0
  mov     rbx, 10
  div     rbx
  ; convert the remainder of the integer 
  ; to ASCII by adding '0' to make it a character
  add     dl, '0'
  mov     [rdi], dl
  dec     rdi
  cmp     rax, 0
  jg      uint64_to_str
  mov     byte [rdi], 0
  mov rdi, 0
  mov rbx, 0
  mov rax, 0
  mov rsi, 0
  ret

section .bss
  ; a buffer for temporarily storing string versions of 64 bit unsigned integers.
  ;  38 characters for the number plus the null terminator (39 bytes)
  uint64_as_str_buffer resb 39

section .data
  extern      malloc
  newline db 10
