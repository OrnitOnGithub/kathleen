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