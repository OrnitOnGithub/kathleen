use std::vec;

use crate::error::{self, print_error, throw_errors, ErrorCode, ERROR_COUNT}; // For throwing errors.
use crate::tokenizer::Token;

/// This is a function that turns a Vector of Token structs into the first Vector of (intermediate) Instructions.
/// It returns the first intermediate representation of two. This is the most abstracted one.
pub fn generate_ir(mut tokens: Vec<Token>) -> Vec<Instruction> {

    // This is what a Token struct looks like btw:
    /*
    {
        token: String,          // the token itself
        line: usize,            // which line this at
        token_number: usize,    // which token in the line this is (1st, 2nd...)
    }
    */

    // Put into a second function in case an implementation requires recursion,
    // or this could also be helpful for implementing functions, not necessarily
    // with recursion.
    fn create_instructions(mut tokens: Vec<Token>) -> Vec<Instruction> {

        let mut instructions: Vec<Instruction> = Vec::new();

        loop {
            // a loop where:
            // - Keywords are handled
            // - a for loop finds a ; and deletes everything before that semicolon.
            // - and starts again, until we have reached the last semicolon.
            let mut index_of_semicolon: usize = 0;
            for (index, token) in tokens.clone().iter().enumerate() {
                if token.token == String::from(";") {
                index_of_semicolon = index;
                break
                }
            }

            if index_of_semicolon == 0 {
                break
            }
            let token: String = tokens[0].token.clone();
            
            match token.as_str() {
                
                "let" => {
                    // This is a let binding. A variable is being defined.
                    // EXAMPLE: let varname bool = true;
                    // EXAMPLE: let varname int const = 1234;
                    // note: not handleing const yet
                    
                    // tokens[1] (the second token) is the variable name.
                    let varname: String = tokens[1].token.clone();
                    // We will store it for later use.
                    
                    // find where the (first) `=` is located, because we know everyhing after that is (a) value(s)
                    let mut index_of_equal: usize = 0;
                    for (index, token) in tokens.clone().iter().enumerate() {
                        if token.token == String::from("=") {
                            index_of_equal = index;
                            break
                        }
                    }
                    
                    match tokens[2].token.as_str() {
                        // Now let's check what variable type we've got.
                        // types: int, bool
                        
                        "int" => {
                            // We found an int! Let's create the `Instruction` for it.
                            // TODO: support for operations after the equal.
                        
                            let instruction: Instruction = Instruction {
                                inst_type: match tokens[index_of_equal + 1].token.clone().as_str().parse::<u64>() {
                                    Ok(value) => Type::Int(value),
                                    Err(e) => {
                                        error::print_error(ErrorCode::IncorrectTypeValuePassed, tokens[2].clone(), "Value passed was not an unsigned 64 bit integer.");
                                        // You might want to return a default value or handle it differently based on your needs
                                        return vec![Instruction {
                                            inst_type: Type::Int(0), // Replace with your default value
                                            parameters: Vec::new(),
                                        }];
                                    }
                                },
                                parameters: vec![Instruction {
                                    inst_type: Type::Name(varname),
                                    parameters: Vec::new(),
                                }],
                            };
                        
                            // We have now created this unholy abomination...
                            instructions.push(instruction);
                        }
                            
                            _ => {
                            print_error(ErrorCode::UnknownKeyword, tokens[2].clone(), "Unknown \
                            data type. The third token of a let binding is a data type.");
                        }
                    };
                }
                
                "print" | "println" => {

                    let mut print_line: bool = false;
                    if token == "println" {
                        print_line = true;
                    }
                    
                    // I'll clean this up later
                    let mut index_of_open_bracket: usize = 0;
                    for (index, token) in tokens.clone().iter().enumerate() {
                        if token.token == String::from("(") {
                            index_of_open_bracket = index;
                            break
                        }
                    }
                    let mut index_of_closed_bracket: usize = 0;
                    for (index, token) in tokens.clone().iter().enumerate() {
                        if token.token == String::from(")") {
                            index_of_closed_bracket = index;
                            break
                        }
                    }
                    for index in index_of_open_bracket+1..index_of_closed_bracket {
                        // Later, keep track of variable types so we know how to reference them accordingly.
                        let is_referenced: bool = true; // temporary.
                        
                        if is_referenced {
                            let instruction: Instruction = Instruction {
                                inst_type: Type::Print,
                                parameters: vec![
                                    Instruction {
                                        inst_type: Type::ReferenceTo(tokens[index].token.clone()),
                                        parameters: Vec::new(),
                                    }
                                ]
                            };
                            instructions.push(instruction);
                        }
                        else {
                            todo!();
                        }
                        // Here we have created this:
                        /*
                        Instruction
                        |---inst_type
                        |     `-Type::Print
                        `---parameters
                            `-Instruction
                            |---inst_type
                                |     `-Type::ReferenceTo(XYZ)
                                `---parameters
                                `-[]
                        */
                    }
                    if print_line {
                        let instruction: Instruction = Instruction {
                            inst_type: Type::PrintLn,
                            parameters: Vec::new(),
                        };
                        instructions.push(instruction);
                    }
                }

                _ => {
                    print_error(ErrorCode::UnknownKeyword, tokens[0].clone(), "")
                }   
            }
            // end of loop here
            // delete all tokens before semicolon
            for _ in 0..index_of_semicolon+1 {
                tokens.remove(0);
            }
        }
        return instructions
    }
    
    let ir: Vec<Instruction> = create_instructions(tokens);
    
    throw_errors(); // cause panic if there were any errors

    return ir;

}
/// Represents an instruction or a set of instructions
/// in the intermediate representation.
/// `inst_type` defines the type of instruction, while `parameters`
/// contains a vector of additional instructions or arguments
/// associated with this instruction.
#[derive(Debug, Clone)]
pub struct Instruction {
    pub inst_type: Type,
    pub parameters: Vec<Instruction>,
}

/// This Enum enumerates different types of "instructions",
/// including definitions for sections like `.data`, loop
/// constructs, conditions, functions,
/// constants, and various data types.
#[derive(Debug, Clone)]
pub enum Type {

    Function,           // The start of the function
    FunctionCall,       // To call the function
    FunctionReturn,     // To exit the function

    Loop,
    LoopBreak,
    
    Condition,          // Define the evaluation
    ConditionTrue,      // Where to go if TRUE Basically
    ConditionFalse,     // Where to go if FALSE
    ConditionExitPoint, // Basically ret, same as function return
    
    Name(String),
    List,
    Static,             // Lifetime = whole execution, it is mutable though
    Int(u64),
    Bool(bool),
    Float(f32),
    String(String),

    Print,
    PrintLn,
    ReferenceTo(String),// Like name but we put [] around it in assembly
    
    Undefined,          // Testing purposes
    Error,
}