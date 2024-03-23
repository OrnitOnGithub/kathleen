nasm -f elf64 output.asm -o output.o -g

gcc -no-pie output.o -o output -g