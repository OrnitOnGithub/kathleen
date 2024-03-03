```
   ___             _ _   _     
  / _ \ _ __ _ __ (_) |_( )___ 
 | | | | '__| '_ \| | __|// __|
 | |_| | |  | | | | | |_  \__ \
  \___/|_|  |_| |_|_|\__| |___/
   COMPILER
```

This is a small compiler that is in the works. The goal of this project is to make a relatively simple piece of code that generates an assembly output for x64 linux netwide assembler.

## Try it

Currently the project is still in development, so there are no releases or packages to use, nor does the program behave as a CLI utility yet. So to get anything to run you must do it yourself.
- Linux x64 required
- Install NASM and GCC
- Clone this repository
- Write code in `mylang` file.
- Run with `cargo run` for example to create assembly output
- Run `./runoutput.sh` to assemble, link and run the assembly file provided by the compiler.

## Language

To learn about the language's syntax or anything related to writing in it, check out [language.md](docs/language.md)

## Issues

Please do not create any issues this right not, it is too early on in development.

If you encounter any issues using this program, create a github issue and please provide:
- The compiler's full output
- The code you wrote
- `output.asm` if it was a runtime issue
- Information about your system
- Any other information that may be need to recreate this issue. If the issue cannot be recreated, reporting it is useless.

If you want to propose any suggestions you are free to do so, especially if you provide something detailed.

If you want to try and solve an issue yourself, check out [Contribute](##contribute)

## Contribute

As of now I want to work on this by myself. Once the language will have taken its basic shape I will accept contributions.

## Documentation

All documentation and information can either be found in the [code itself](src/main.rs), or in the [docs directory](/docs/) for higher-level information.

## License & Legal

This project is licenced under the MIT license. TLDR: This means you are free to copy, use and distribute this program as long as you include the `LICENSE` file in your own project.