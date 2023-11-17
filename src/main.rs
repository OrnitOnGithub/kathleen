use std::fs::read_to_string;

// Later the compiler should take this as a parameter.
const FILEPATH: &str = "src/mylang.c";

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.

fn main() {

    /// Create a Vector for each line of the code.
    ///
    /// The only reason we separate lines is for the 
    /// compiler to be able to return a line number when
    /// an error occurs. All we do is return the index + 1.
    let mut lines = Vec::new();
    // Iterate through the lines, add to Vector
    for line in read_to_string(FILEPATH).unwrap().lines() {
        lines.push(line.to_string());
    }


    /// Tokenise the code by splitting every whitespace character.
    /// Store all tokens in a vector
    /// We get a Vector like this:
    /// ```
    ///   line 1         line 2
    /// [["Hello"], ["hi", "there"]]
    /// ```
    let mut tokenised_lines = Vec::new();
    for line in lines {
        // Create a new vector with every token
        tokenised_lines.push(line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>());
    }
    println!("tokens: {:?}", tokenised_lines);


    //


}

/// Represents an instruction or a set of instructions
/// in the intermediate representation.
///
/// `inst_type` defines the type of instruction, while `parameters`
/// contains a vector of additional instructions or arguments
/// associated with this instruction.
//#[derive(Debug)]
struct Instruction {
    inst_type: Type,
    parameters: Vec<Instruction>,  
}

/// This Enum enumerates different types of "instructions",
/// including definitions for sections like `.data`, loop
/// constructs, conditions, identifiers,
/// constants, and various data types.
//#[derive(Debug)]
enum Type {
    StaticData,         // Section .data
    Main,               // global main \n main:

    Loop,               // Start of the loop
    LoopJump,           // Jump back to the start of the loop
    LoopBreak,          // Exit the loop
    LoopExitPoint,      // Where to go after exiting the loop

    Condition,          // Define the evaluation
    ConditionTrue,      // Where to go
    ConditionFalse,
    ConditionExitPoint, // Where to go after a condition's code.

    Identifier(i32),    // To differentiate between for example jump points.
                        // Must be incremented after each use
    List,
    Const,              // Lifetime = whole execution, immutable
    Static,             // Lifetime = whole execution, it is mutable though
    Int(i32),
    Float(f32),
    String(String),
}
