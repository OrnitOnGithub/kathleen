use std::process::Command;
use std::env;

use std::fs::read_to_string;
use std::fs::File;
use std::fs;

use crate::tokenizer::Token;

extern crate colored;
use colored::*;

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

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.

fn main() {

    // get the file path from command line argument number 1
    let args: Vec<String> = env::args().collect();

    if args.contains(&"help".to_string()) {
        error::print_help();
    }

    // If there are no CLI arguments
    if args.len() < FILEPATH_ARG+1 {
        error::print_error(
            error::ErrorCode::MissingCommandLineArgument,
            Token { token: "".to_string(), line: 0, token_number: 0 },
            "Missing source code file path. Run with parameter `help` for usage info");
        error::print_help();
    }
    let file_path = &args[FILEPATH_ARG];

    // See if file exists
    if !fs::metadata(&file_path).is_ok() {
        error::print_error(
            error::ErrorCode::InvalidFile,
            Token { token: "".to_string(), line: 0, token_number: 0 },
            &format!("File not found: {}", file_path),
        )
    }

    // Create a Vector for each line of the code.
    // The tokenizer function will use this to know which line of code
    // each token is in.
    let mut code_lines: Vec<String> = Vec::new();
    // Iterate through the lines, add to Vector
    for line in read_to_string(file_path).unwrap().lines() {
        code_lines.push(line.to_string());
    }
    //println!("Code: {:?} \n", code_lines);

    // Tokenize (and preprocess) the code. See `tokenize` function
    // (in tokenizer.rs) for more info
    let tokens = tokenizer::tokenize(code_lines);
    //println!("Tokens: {:?} \n", tokens);

    let intermediate_representation = ir_generator::generate_ir(tokens);
    //println!("IR: {:?} \n", intermediate_representation);

    let near_assembly_representation = nar_generator::generate_nar(intermediate_representation);
    //println!("NAR: {:?} \n", near_assembly_representation);

    let assembly_output = asm_generator::generate_asm(near_assembly_representation);
    //println!("{:?} \n", assembly_output);

    // create the output file. Later this should also be passed as parameter.
    let file_path: &str = "output.asm";
    let _file = File::create(file_path);
    fs::write(file_path, assembly_output).expect("Unable to write to file");

    // Assemble and link the assembly output

    // run the command `nasm -f elf64 output.asm -o output.o -g`
    let mut assemble = Command::new("nasm");
    assemble.arg("-f").arg("elf64").arg(file_path).arg("-o").arg("output.o").arg("-g");
    let _ = assemble.output().expect("failed to execute process");
    //println!("{:?}", assemble_output);
    
    // run the command `gcc -no-pie output.o -o output -g`
    let mut link = Command::new("gcc");
    link.arg("-no-pie").arg("output.o").arg("-o").arg("output").arg("-g");
    let _ = link.output().expect("failed to execute process");
    //println!("{:?}", link_output);

    let compilation_successful: &str = "\nCompilation successful! Run the output binary with `./output`";
    println!("{}", compilation_successful.green());

}