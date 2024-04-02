## 02/04/24

- Removed sex from the compiler

## 01/04/24

- Added sex to the compiler

## 30-13/03/24

- Added "stdlib.asm", contains function for int -> str and the info in the .data section previously in "external.asm"
  - Printing 64 bit unsigned integers now works
- Fixed issue with too many newlines or weird characters after every print. print behaves as expected now.

## 23/03/24

- Embedded all files into the compiled binary. The program can now access `asm/nasm-x64-linux/*` even when run outside of the project directory.
- Added two errors in `error.rs` and used them in `main.rs` for handling:
  - Missing command line argument: file path
  - Provided file does not exist

## 04-16/03/24

- Constant strings, printing of constant strings and printline finally work.

## 02-03/02/24

- Started working on strings in the tokenizer, however current implementation sucks.

- Currently working on re-doing the variable types.
  - Constant integer
  - Dynamic integer
  - Constant string
  - Dynamic string
- The plan is:
  - Have constant string printable
  - Have dynamic string printable
  - Have ints printable

## 01/02/24

- Made the errors nicer with the colored crate.

## 29/02/24

- First compilation! All the first program did was put an integer on the heap.
- Made the error messages cleaner by adding an extra space where needed to align everything
  - Before:
    ```
    9 | code
    10 | code
    11 | code
    ```
  - Now:
    ```
     9 | code
    10 | code
    11 | code
    ```

## 27/02/24

- Created the assembly code blocks and a function in asm_generator.asm that replaces the "<>" in the asm blocks with the proper values.

## 26/02/24

- Finally got the bss idea working.
  - Here's how variables are made:
    - allocate area in .bss for pointer
    - call malloc, put the recieved pointer into the memory area pointed to by the .bss pointer.
    - To access the data, double dereference the BSS pointer.

## 24-25/02/24

- Turns out the way I wanted to do things is not possible (store pointer to var in section .data, which is immutable, silly me!)
- Instead I'm working on a different approach, where we have a list of pointers in the bss section.
- Today, the 25th, I was finally able to write assembly code that puts a value on the heap.

## 21/02/24

- Started work on NAR.
- Finished Let and Print in both IR and NAR

## 20/02/24

- Finished Let binding in IR
    - only supports int.
- Put the IR generating function in another function to make functions, loops and all that pizazz easier later

## Before

idk