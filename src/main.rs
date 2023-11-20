use std::fs::read_to_string;

// Later the compiler should take this as a parameter.
const FILEPATH: &str = "src/mylang.c";

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.

fn main() {

    // Create a Vector for each line of the code.
    //
    // The only reason we separate lines is for the 
    // compiler to be able to return a line number when
    // an error occurs. All we do is return the index + 1.
    let mut lines = Vec::new();
    // Iterate through the lines, add to Vector
    for line in read_to_string(FILEPATH).unwrap().lines() {
        lines.push(line.to_string());
    }


    // Tokenise the code by splitting every whitespace character.
    // Store all tokens in a vector
    // We get a Vector like this:
    // ```
    //   line 1         line 2
    // [["Hello"], ["hi", "there"]]
    // ```
    let mut tokenised_lines = Vec::new();
    for line in lines {
        // Create a new vector with every token
        tokenised_lines.push(line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>());
    }
    println!("tokens: {:?}", tokenised_lines);


    // Remove all comments from the code
    // Leave blank spaces where there were comments to maintain
    // the line count
    //
    // CURRENTLY WE ONLY HAVE "//" IMPLEMENTED
    // As soon as we meet "//" -> Rest of the line becomes a comment
    // As soon as we meet "/*" -> Everything beomes a comment until "*/"
    //                                             |
    // The "comment" bool is true until "*/"  <---'
    // let mut comment: bool = true;
    for line_index in 0..tokenised_lines.len() {
        let line = &mut tokenised_lines[line_index];
    
        // Find the index of the first occurrence of "//"
        if let Some(index) = line.iter().position(|keyword| keyword == "//") {
            line.truncate(index); // Remove elements from the index to the end
        }
    }
    
    println!("commentless tokens: {:?}", tokenised_lines);
    


}

/// Represents an instruction or a set of instructions
/// in the intermediate representation.
/// `inst_type` defines the type of instruction, while `parameters`
/// contains a vector of additional instructions or arguments
/// associated with this instruction.
// #[derive(Debug)]
struct Instruction {
    inst_type: Type,
    parameters: Vec<Instruction>,
}

/// This Enum enumerates different types of "instructions",
/// including definitions for sections like `.data`, loop
/// constructs, conditions, functions, identifiers,
/// constants, and various data types.
//#[derive(Debug)]
enum Type {
    StaticData,         // Section .data
    Main,               // global main \n main:

    Function,           // The start of the function, OR THE LOOP
    FunctionCall,       // To call the function OR CALL THE LOOP AGAIN
    FunctionReturn,     // To exit the function, OR BREAK THE LOOP

    Condition,          // Define the evaluation
    ConditionTrue,      // Where to go if TRUE Basical
    ConditionFalse,     // Where to go if FALSE
    ConditionExitPoint, // Basically ret, same as function return

    Identifier(i32),    // To differentiate between for example jump points.
                        // Must be incremented after each use
    List,
    Const,              // Lifetime = whole execution, immutable
    Static,             // Lifetime = whole execution, it is mutable though
    Int(i32),
    Float(f32),
    String(String),
}
