```
  ___             _ _   _     
 / _ \ _ __ _ __ (_) |_( )___ 
| | | | '__| '_ \| | __|// __|
| |_| | |  | | | | | |_  \__ \
 \___/|_|  |_| |_|_|\__| |___/
  COMPILER
```
<div aria-label="Ornit's compiler"><p aria-hidden="true"></p></div> <!-- This is for screen readers -->

This is a small compiler that is in the works. The goal of this project is to make a relatively simple piece of code that generates an assembly output for x64 linux netwide assembler out of a made up language.

## Try it

There are no releases yet so to try this program you must build it yourself.
- Linux x64 required
- Install NASM and GCC
- Clone this repository
- Write your code in a file
- Run with `cargo run -- <path/to/code>` or `cargo build && ./<path/to/biary> <path/to/code>`
- Run the program you created with `./output`

## Language

To learn about the language's syntax or anything related to writing in it, check out [language.md](docs/language.md)

## Issues

If you are having trouble, or if something is unclear, you can create an issue and ask for help using the quesition / documentation issue template.

If you encounter any issues using this program, create a github issue and use the bug report template.

If you want to propose any suggestions you are free to do so, you can use the feature request template.

If you want to try and solve an issue yourself, check out [Contribute](##contribute)

## Contribute

As of now I want to work on this by myself. Once the language will have taken its basic shape I will accept contributions, and any help will be greatly appreciated.

## Documentation

All documentation and information can either be found in the [code itself](src/main.rs), or in the [`docs` directory](/docs/) for higher-level information.

## License & Legal

This project is licenced under the MIT license. In short, this means you are free to copy, use and distribute this program as long as you include the `LICENSE` in your own project.
