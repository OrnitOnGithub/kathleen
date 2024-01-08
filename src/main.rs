use std::fs::read_to_string;

mod tokenizer;      // This is the code for the tokenizer; first step of compilation.
                    // Check `src/tokenizer.rs` for more info

mod ir_generator;   // This is the code for the generation of the Intermediate 
                    // Representation. Check `src/ir_generator.rs` for more info

mod error;          // This is the code for throwing errors.
                    // For more info check src/error.rs

// File path - Later the compiler should take this as a parameter.
pub static FILEPATH: &str = "mylang.c";

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.

fn main() {

    // Create a Vector for each line of the code.
    //
    // The only reason we separate lines is for the 
    // compiler to be able to return a line number when
    // an error occurs. All we do is return the index + 1.
    let mut code_lines = Vec::new();
    // Iterate through the lines, add to Vector
    for line in read_to_string(FILEPATH).unwrap().lines() {
        code_lines.push(line.to_string());
    }

    // Tokenize (and preprocess) the code. See `tokenize` function
    // (in tokenizer.rs) for more info
    let tokens = tokenizer::tokenize(code_lines);

    let intermediate_representation = ir_generator::generate_ir(tokens);

    println!("IR: {:?}", intermediate_representation);

}