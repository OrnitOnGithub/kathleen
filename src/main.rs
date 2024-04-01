use std::process::Command;
use std::env;

use std::fs::read_to_string;
use std::fs::File;
use std::fs;

use std::time::Instant;

// Colours in the terminal
use colored::*;

use crate::tokenizer::Token;

mod tokenizer;      // This is the code for the tokenizer; first step of compilation.
                    // Check `src/tokenizer.rs` for more info.

mod ir_generator;   // This is the code for the generation of the Intermediate 
                    // Representation. Check `src/ir_generator.rs` for more info.

mod nar_generator;  // This is the code for the generation of the second IR,
                    // the Near Assembly Representation. Check `src/ir_generator.rs`
                    // for more info.

mod asm_generator;  // This is the code responsible for generating the assembly
                    // output. Check `src/asm_generator.rs` for more info.

mod error;          // This is the code for throwing errors.
                    // Check `src/error.rs` for more info.

pub const FILEPATH_ARG: usize = 1;
pub const OUTPUTPATH_ARG: usize = 2;

const LICENSE: &'static str = include_str!("../LICENSE");

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.

/// The main function handles user input and then starts the compilation.
fn main() {
    let compilation_start_time = Instant::now();

    let args: Vec<String> = env::args().collect();

    // If there are no arguments
    if args.len() < FILEPATH_ARG+1 {
        println!("{}", "Kathleen Programming Language Compiler\n".green());
        println!("{}", LICENSE);
        error::print_help()
    }

    // If `help` is an argument
    if args.contains(&"help".to_string()) {
        error::print_help();
    }

    let file_path = &args[FILEPATH_ARG];
    let mut output_path: &String =  &String::from("output");

    // If there is a second argument (output file name)
    if args.len() >= OUTPUTPATH_ARG+1 {
        output_path = &args[OUTPUTPATH_ARG];
    }

    // If input file does not exist
    if !fs::metadata(&file_path).is_ok() {
        // Throw the error saying it doesnt
        error::print_error(
            error::ErrorCode::InvalidFile,
            Token { token: "".to_string(), line: 0, token_number: 0 },
            &format!("File not found: {}", file_path.green()),
        );
        error::throw_errors();
    }

    println!("{} {}\n", "Kathleen: compiling".green(), file_path.green().italic());   

    // Create a Vector for each line of the code.
    // The tokenizer function will use this to know which line of code
    // each token is in.
    let mut code_lines: Vec<String> = Vec::new();
    // Iterate through the lines, add to Vector
    for line in read_to_string(file_path).unwrap().lines() {
        code_lines.push(line.to_string());
    
    }

    // Compile
    let tokens
        = tokenizer::tokenize(code_lines);

    let intermediate_representation
        = ir_generator::generate_ir(tokens);

    let near_assembly_representation 
        = nar_generator::generate_nar(intermediate_representation);

    let assembly_output
        = asm_generator::generate_asm(near_assembly_representation);

    // Write the assembly output to a file
    let file_path: &str = "output.asm";
    let _file = File::create(file_path);
    fs::write(file_path, assembly_output)
        .expect("Unable to write to file");

    // Assemble and link the assembly output file

    // run the command `nasm -f elf64 output.asm -o output.o -g`
    let mut assemble = Command::new("nasm");
    assemble.arg("-f").arg("elf64").arg(file_path).arg("-o").arg("output.o").arg("-g");
    let asm_output = assemble.output()
        .expect("Failed to run command. Is NASM installed?");
    println!("{}: [{}]",
        "Errors while assembling",
        String::from_utf8(asm_output.stderr.clone()).unwrap().red()
    );
    
    // run the command `gcc -no-pie output.o -o output -g`
    let mut link = Command::new("gcc");
    link.arg("-no-pie").arg("output.o").arg("-o").arg(output_path).arg("-g");
    let link_output = link.output()
        .expect("Failed to run command. Is GCC installed?");
    println!("{}: [{}]",
        "Errors while linking",
        String::from_utf8(link_output.stderr.clone()).unwrap().red()
    );

    if asm_output.stderr.len() == 0 && link_output.stderr.len() == 0 {
        let compilation_duration = compilation_start_time.elapsed();
        println!("{} {:?}{}{}",
        "\nCompilation successful in".green(),
        compilation_duration,
        "! Run the output binary with ./".green(),
        output_path.green().italic()
        );
    }
}