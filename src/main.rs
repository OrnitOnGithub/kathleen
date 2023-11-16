use std::fs::read_to_string;

const FILEPATH: &str = "src/mylang.c"; // Later the compiler should take this as a parameter.

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
    // 
    //   line 1         line 2
    // [["Hello"], ["hi", "there"]]
    let mut tokenised_lines = Vec::new();
    for line in lines {
        // Create a new vector with every token
        tokenised_lines.push(line.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>());
    }
    println!("tokens: {:?}", tokenised_lines);


    


}


//#[derive(Debug)]
struct Instruction {
    instruction_type: Type,
    parameters: Vec<Instruction>,    
}


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
    Int(i32),
    Float(f32),
    String(String),
}
