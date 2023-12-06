use crate::error::{print_error, ErrorCode, throw_errors}; // For throwing errors.

/// Generates the intermediate representation.
/// Check `src/ir_generator.rs` for more info.
pub fn generate_ir(tokenized_lines: Vec<Vec<String>>) -> Vec<Instruction> {
    // tokenized_lines: Vec<Vec<String>>
    //     outer Vector (Vec<Vec<String>>): line (Vec<String>)
    //     inner Vector (Vec<String>): word (String)

    // This is the list of instructions to be returned.
    let mut instructions: Vec<Instruction> = Vec::new();

    // This is all a little messy.
    // This for loop iterates through each line.
    // Then each line it calls create_instruction(), giving it two parameters:
    //     - the line
    //     - the line number (for throwing errors)
    // create_instructions(line, linecount) then:
    //     - copies the first word in the line
    //     - removes that first word from the line
    //     - generates an instruction
    //         - the function may call itself again, for example when creating a Scope
    //     - It then returns an Instruction.
    //     - NOTE: This function is designed for it to easily call itself within itself.
    //         - Recursion is cery useful.
    for (line_number, mut line) in tokenized_lines.into_iter().enumerate() {
        instructions.push(create_instruction(&mut line, line_number));
    }

    /// This is an occasionally recursive function that is inside `generate_ir()`.
    /// 
    /// This function is meant to be able to handle recursion, hence its slightly weird
    /// design.
    /// 
    /// It takes two paramers :
    /// - `line: &mut Vec<String>` which is the preprocessed line
    /// - `line_number: usize` which is the line count, used to throw errors.
    /// 
    /// And returns:
    /// - `Instruction`: The instruction struct to be added to the instructions vector
    ///     inside `generate_ir()`. This Instruction struct may contain Instruction
    ///     structs within itself, if the function called itself (for example for scopes)
    /// 
    /// Here's what it does:
    /// - copies the first word in the line
    /// - removes that first word from the line
    /// - generates an instruction
    ///    - the function may call itself again, for example when creating a Scope
    /// - It then returns an Instruction.
    fn create_instruction(line: &mut Vec<String>, line_number: usize) -> Instruction {
        
        // I hate that I can't have brackets here!!
        while line.len() > 0 {

            // Copy the word to process it later ---.
            let word = line[0].clone(); //     |
            //                                      |
            // Remove the first word                |
            line.remove(0); //                      |
            //                                      |
            // Handle the keyword  <----------------´
            match &word as &str {
                "{" => {
                    // This is the start of a Scope
                    // when this occurs, a Scope Instruction struct is created and
                    // The function is called a gain inside this Scope Instruction
                    // struct.

                    // line.remove(0); // this is allowed!! 
                                       // reminder to use this for variables or smth
                    return Instruction {
                        inst_type: Type::Scope,
                        // problem here !!
                        // maybe revisit the stack of scopes idea
                        // basically append instructions to last element in stack of
                        // scopes, and when "}" is reached, return that last scope
                        parameters: vec![create_instruction(line, line_number)],
                    };
                }
                
                _ => {
                    // replace this with unrecognised keyword error

                    print_error(ErrorCode::UnknownKeyword, line_number);

                    return Instruction {
                        inst_type: Type::Loop,
                        parameters: vec![],
                    };
                }
            }
        }
        return Instruction{ // If this gets returned it means there was an empty line
            inst_type : Type::Undefined,
            parameters : vec![],
        };
        }

        // This will only throw an error if any errors occurred earlier.
        // Check `src/error.rs` for more info
        //throw_errors();
        return instructions;
    }



/// Represents an instruction or a set of instructions
/// in the intermediate representation.
/// `inst_type` defines the type of instruction, while `parameters`
/// contains a vector of additional instructions or arguments
/// associated with this instruction.
#[derive(Debug)]
pub struct Instruction {
    inst_type: Type,
    parameters: Vec<Instruction>,
}

/// This Enum enumerates different types of "instructions",
/// including definitions for sections like `.data`, loop
/// constructs, conditions, functions, identifiers,
/// constants, and various data types.
#[derive(Debug)]
pub enum Type {
    StaticData,         // Section .data
    Main,               // global main \n main:

    Scope,              // Everything in the scope will be inside this
    ScopeExit,          // Always at the end of a scope

    Function,           // The start of the function
    FunctionCall,       // To call the function
    FunctionReturn,     // To exit the function

    Loop,
    LoopBreak,

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

    Undefined,          // Testing purposes
}
