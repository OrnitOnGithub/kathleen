# Kathleen compiler documentation

This is the documentation for the compiler itself. This document is here to explain how a program is compiled. Further information is found [inside the code](../src/main.rs).

There are 7 steps to compiling a Kathleen program. This may resemble some other compilers.
- **Command Line Interface**: The parameters provided by the user are processed. File name, output file name, and extra options are extracted.
- **Tokenization**: The first step in compiling a program is to split the keywords into tokens. This is done by the tokenizer. It also acts as a pre-processor by removing comments and joining strings together into a single token.
- **Inetermediate representation generation**: Our tokens are then analyzed and converted into an intermediate representation (IR). This is a sort of abstract instruction tree. It is a structured organisation of instructions the program has to execute.
- **Near assembly representation generation**: This is a second intermediate representation that is generated using the IR. It is a linear sequence of very simple "instructions". It is made to be read entirely linearly. It also 1-to-1 translates into blocks of assembly code.
- **Assembly output generation**: The near assembly representation (NAR) is then converted into assembly code. Each small instruction gets converted into a small block of assembly code.
- **Assembling**: The assembly code has to be assembled into machine code. This is however not done by the Kathleen compiler but by NASM.
- **Linking**: The object file then has to be linked so it can use the C library. This is also done by an external program, GCC.
- **Errors**: Problems may occur anytime during compilation. They have to be handled and an error module exists for that.
 
# Tokenization (tokenizer.rs)

Spelled with a "z" by convention.

The tokenizer is a funtion inside `src/tokenizer.rs`, `pub fn tokenize(lines: Vec<String>) -> Vec<Token>` called in `src/main.rs`

The tokenizer works in four steps, to finally provide a Vector of `Token` structs.

## Step 1 -- Separate by whitespace and special characters

Tokens are first separated by whitespace, anything like spaces, tabs, newlines.

`let var int = 12;` => `let` `var` `int` `=` `12;`

Notice how the 12 and the semicolon are stuck to eachother? We also need to separate by things like brackets, commas, etc.

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

During this step strings are also kept together as a single token. If a `"` is met then everything remains unconditionally attached until the next `"`

## Step 2 -- Remove comments

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
  Increment(String),
  Loop(String),
  LoopExit(String),
  ...
}
```

Example of an instruction that defines a constant string
```
Instruction
|---inst_type
|     `-Type::PrintConstStr
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
#[derive(Debug, Clone)]
pub enum NAI {
    CreatePointer(String),
    AllocateInt(String, u64),

    PrintReferenceTo(String),
    Print(String),
    PrintLn,

    StdLib,
    EndProgram,
}
// near assembly representation
#[derive(Debug, Clone)]
pub struct NAR {
  pub data: Vec<NAI>,
  pub bss: Vec<NAI>,
  pub main: Vec<NAI>,
  pub other: Vec<NAI>,
}
```

# Assembly output genertion (asm_generator)

This part of the program turns the near assembly representation into blocks of assembly code, and generates the final assembly output

In the following example, the near assembly instruction for printing a string gets turned into assembly code:

```rust
// match statement encounters the print constant string enum variant
NAI::PrintConstStr(variable_name) => {
  // it appends the modified assembly code block to the `asm` string.
  asm += &replace_values_in_file(
    "print_constant_string.asm",
    vec![&variable_name, variable_name.as_str()]
  )
}
```
replace_values_in_file is a function that takes as parameters values and inserts them into a file as shown below:

asmpath/print_constant.asm: original file
```asm
; Prints a constant string (defined in .data section)
mov         rdx,    [<>length]      ; length of the message
mov         rsi,    <>              ; pointer to the message
mov         rdi,    1
mov         rax,    1
syscall

```
here is what will get appended to `asm`: modified version where the name of the string (let's call it "constantname") gets inserted into the `<>` placeholders
```asm
; Prints a constant string (defined in .data section)
mov         rdx,    [constantnamelength]      ; length of the message
mov         rsi,    constantname              ; pointer to the message
mov         rdi,    1
mov         rax,    1
syscall
```

This process is done for the `NAI`s in each of the sections in the `NAR` struct.

The assembly output is then written to an intermediate file.

# Assembling and linking

The compiler uses the following two programs to assemble and link the assembly output.

## Assembler (NASM)

The assembly output is first assembled by the Netwide Assembler.

## Linker (GCC)

Since malloc and free from the C library are used, the compiler uses GCC as its linker.
