```
 _  __     _   _     _
| |/ /__ _| |_| |__ | | ___  ___ _ __
| ' // _` | __| '_ \| |/ _ \/ _ \ '_ \
| . \ (_| | |_| | | | |  __/  __/ | | |
|_|\_\__,_|\__|_| |_|_|\___|\___|_| |_|
 COMPILER            by Ornithopter747
```
<div aria-label="Kathleen compiler, by Ornithopter747"><p aria-hidden="true"></p></div> <!-- This is for screen readers -->

This is a small compiler that is in the works. It compiles a made up low-level programming language into an executable output for x64 Linux.

## Try it

### Build it

There are no releases yet so to try this program you have to build it yourself.
- Linux x64 required
- Install NASM and GCC
- Clone this repository
- Run `cargo build`
- You'll find the executable in `target/debug/katheen`.

### Run it

(in these examples I will assume you added `<path/to/project>/target/debug` to your path.)

To display the help menu run
```sh
kathleen
```
or
```sh
kathleen help
```
To compile a program run
```sh
kathleen <src> <output-name> [options]
```
For example:
```sh
kathleen hello.kl hello
```
This will create an executable you can run with `./hello` out of your program in `hello.kl`.

## Kathleen Language

### Learn it

To learn about the language's syntax or anything related to writing in it, check out [language.md](docs/language.md) or [examples](examples).

### Visual Studio Code extension

You can download the syntax highlighting extension [here](https://marketplace.visualstudio.com/items?itemName=Ornithopter747.kathleen-syntax).

## Documentation

All documentation and information can either be found in the [code itself](src/main.rs), or in the [`docs` directory](/docs/) for higher-level information. NOTE: this is not information meant for the user, it is information about the project.

## Issues

If you are having trouble, or if something is unclear, you can create an issue and ask for help using the quesition / documentation issue template. <br>
If you encounter any issues using this program, create a github issue and use the bug report template. <br>
If you want to propose any suggestions you are free to do so, you can use the feature request template.

If you want to try and solve an issue yourself, check out [Contribute](#contribute)

## Contribute

Once the language will have taken its basic shape, and any help will be greatly appreciated. As of now I want to work on this by myself.

## License & Legal

This project is licenced under the MIT license. In short, this means you are free to copy, use and distribute this program as long as you include the `LICENSE` in your own project.
