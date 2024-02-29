use std::fs::read_to_string;
use std::fs::File;
use std::fs;

mod tokenizer;      // This is the code for the tokenizer; first step of compilation.
                    // Check `src/tokenizer.rs` for more info

mod ir_generator;   // This is the code for the generation of the Intermediate 
                    // Representation. Check `src/ir_generator.rs` for more info

mod nar_generator;  // This is the code for the generation of the second IR,
                    // the Near Assembly Representation. Check `src/ir_generator.rs`
                    // for more info.

mod asm_generator;  // This is the code responsible for generating the assembly
                    // output. Check `src/asm_generator.rs` for more info.

mod error;          // This is the code for throwing errors.
                    // Check `src/error.rs` for more info

// File path - Later the compiler should take this as a parameter.
pub const FILEPATH: &str = "mylang";
pub const OUTPUTPATH: &str = "output.asm";

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.

fn main() {

    // Create a Vector for each line of the code.
    // The tokenizer function will use this to know which line of code
    // each token is in.
    let mut code_lines = Vec::new();
    // Iterate through the lines, add to Vector
    for line in read_to_string(FILEPATH).unwrap().lines() {
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
    fs::write(file_path, assembly_output).expect("Unable to write file");
    println!("Program compiled!")
}