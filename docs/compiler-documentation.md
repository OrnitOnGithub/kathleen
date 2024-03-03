# Ornithopter747's compiler: documentation

This is the documentation for the compiler itself. This document is here to explain how a program is compiled on a higher level. Lower level documentation is found [inside the code](../src/main.rs).

This document is not very complete. Some of the examples provided may be out of date.

## Table of contents:

[1. Tokenization (tokenizer)](#tokenization-tokenizer)
[2. Intermediate representation generation (ir\_generator)](#intermediate-representation-generation-ir_generator)
[3. Near assembly representaation (nar\_generator)](#near-assembly-representaation-nar_generator)
[4. Assembly output genertion (asm\_generator)](#assembly-output-genertion-asm_generator)
[5. Assembling and linking](#assembling-and-linking)

# Tokenization (tokenizer)

Spelled with a "z" by convention.

The tokenizer is a funtion inside `src/tokenizer.rs`, `pub fn tokenize(lines: Vec<String>) -> Vec<Token>` called in `src/main.rs`

The tokenizer works in four steps, to finally provide a Vector of `Token` structs.

## Step 1 -- Separate by whitespace

Tokens are first separated by whitespace, anything like spaces, tabs, newlines.

`let var int = 12;` => `let` `var` `int` `=` `12;`

Notice how the 12 and the semicolon are stuck to eachother?

## Step 2 -- Separate special characters

Prior to this step, one token could totally have been `function(arg1,arg2)`, or `12;`

It is now necessary to separate futher by things like brackets, commas, etc.

The list of characters to separate is:

```
( ) [ ] { }
- + * % / & = < > ! | ^
, . ; : ' "
```
```rust
let special_chars: HashSet<char> = [
    '(', ')',                       
    '{', '}',                       
    '[', ']',                       
    '<', '>',                       
    '\'', '"',                      
    '!', '|', '&',                  
    ',', '.', ':', ';',
    '+', '*', '/', '-', '=', '^',
].iter().cloned().collect();
```

## Step 3 -- Remove comments

If two consecutive "/" tokens are met, delete them and the rest of the line.

This makes the below comments valid.
```
//comment
// comment
/    / comment
```


## Step 4 -- Turn into `Token` struct

```rust
{
    token: String,          // the token itself
    line: usize,            // which line it is at
    token_number: usize,    // index of token in line
}
```

Turning each token into a struct that describes it is great for error handling, as soon as a problematic token is encountered it can easily be passed to `error::print_error`, who will easily know where the token is located.

# Intermediate representation generation (ir_generator)

The intermediate representation is a structure represnting the code in an abstracted way. It will turn a vector of `Token` structs into a vector of `Instruction` structs. These form a sort of abstract logic tree.

## Instructions in the IR

This is what the instruction struct looks like:

```rust
pub struct Instruction {
    pub inst_type: Type,
    pub parameters: Vec<Instruction>,
}

pub enum Type {

    Function,
    FunctionCall,       
    FunctionReturn,     

    Int(u64),
    Bool(bool),
    Slice(String),

    Print,
    PrintLn,
    ReferenceTo(String),
}
```

Example of an instruction that prints something
```
Instruction
|---inst_type
|     `-Type::PrintStr
`---parameters
      `-Instruction
        |---inst_type
        |     `-Type::Name(XYZ)
        `---parameters
              `-[]
```

# Near assembly representaation (nar_generator)

The near assembly representation is a second intermediate representtion. It consists of low-level instructions that each have an assembly counterpart.

```rust
// near assembly instruction
pub enum NAI {
    CreatePointer(String),
    AllocateInt(String, u64),

    PrintReferenceTo(String),
    Print(String),
    PrintLn,

    DeclareExterns,
    EndProgram,
}
// near assembly representation
pub struct NAR {
    pub data: Vec<NAI>,
    pub bss: Vec<NAI>,
    pub main: Vec<NAI>,
}
```

# Assembly output genertion (asm_generator)

This part of the program turns the near assembly representation into blocks of assembly code, and generates the final assembly output

# Assembling and linking

My program is not responsible for these two parts.

## Assembler (NASM)

The assembly output is first assembled by the Netwide Assembler.

## Linker (GCC)

Since malloc and free from the C library are used, the compiler uses GCC as its linker.

## Current solution

`runoutput.sh`:
```sh
nasm -f elf64 output.asm -o output.o -g
gcc -no-pie output.o -o output -g
./output
```