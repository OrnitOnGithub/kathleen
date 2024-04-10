# Kathleen compiler documentation

This document provides high-level information about the project overall. Further information can be found in [compiler.md](compiler.md) or [inside the code](../src/main.rs).

## docs

- [brainstorm.md](brainstorm.md) -> A document full of random brainstorming stuff.
- [compiler.md](compiler.md) -> information about the compilation process.
- [keywords.md](keywords.md) -> the list of all keywords to be implemented.
- [language.md](language.md) -> information about the Kathleen language.
- [todo.md](todo.md) -> the temporary to do list.

## Project structure

- [src](#src)/
- [asm](#asm)/
- [examples](#examples)/
- [tools](#tools)/

### src

The source code directory.

- `main.rs`: the main program, handles user input and starts 
the compilation process.
- `tokenizer.rs`: the tokeniser and preprocessor.
- `ir_generator.rs`: the generator of the intermediate representation.
- `nar_generator.rs`: the generator of the "near assembly representation", a second intermediate representation
- `asm_generator.rs`: the generator of the assembly output using the near assembly representation.
- `error.rs`: handles errors, warnings and shows the help menu.

### asm

The directory for all assembly code to be used by the compiler. Currently the only directory inside it is `nasm-x64-linux` because that is the only supported platform.

`nasm-x64-linux` contains a list of template assembly code blocks that get put together by the compiler. Here's the code to print an integer (`asm/nasm.x64.linux/print_uint64`) as example:
```asm
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
```
<> will get replaced by the compiler.

### examples

Contains a list of example programs in the Kathleen language.

### Tools

Contains all tools related to the language. Currently that's only the syntax highlighting extention for vscode.