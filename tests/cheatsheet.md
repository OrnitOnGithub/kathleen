gcc -S -o tests/let-drop.s tests/let-drop.c

nasm -f elf64 -o let-drop-2.o let-drop-2.asm
ld let-drop-2.o -o let-drop-2