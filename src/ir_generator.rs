use crate::error::{print_error, ErrorCode, throw_errors}; // For throwing errors.
use crate::tokenizer::Token;

/// This is the function that turns a Vector of Token structs into the final Vector of Instructions.
pub fn generate_ir(mut tokens: Vec<Token>) {

    // This is what a Token struct looks like btw:
    /*
    {
        token: String,          // the token itself
        line: usize,            // which line this at
        token_number: usize,    // which token in the line this is (1st, 2nd...)
    }
    */

    let mut instructions: Vec<Instruction> = Vec::new();

    // later, put this in a loop, where:
    // - this runs once
    // - a for loop finds a ; and deletes everything before that semicolon.
    // - and starts again, until we have reached the last semicolon.
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
                        inst_type: Type::Int(tokens[index_of_equal+1].token.clone().as_str().parse::<i32>().unwrap()),
                        parameters: vec![Instruction {
                            inst_type: Type::Name(varname),
                            parameters: Vec::new(),
                            }]
                        };
                        // We have now created this unholy abomination:
                        /*
                        Instruction
                        |---inst_type
                        |     `-Type::Int(xyz)
                        `---parameters
                              `-Instruction
                                |---inst_type
                                |     `-Type::Name(xyz)
                                `---parameters
                                      `-[]
                        */
                        // Should this be done like this...?
                    println!("{:?}", instruction)
                }

                _ => {
                    print_error(ErrorCode::UnknownKeyword, tokens[2].clone(), "Unknown \
                    data type. The third token of a let binding is a data type.");
                }
            };
        }

        _ => {
            print_error(ErrorCode::UnknownKeyword, tokens[0].clone(), "")
        }
    }
    
    throw_errors();

}
/// Represents an instruction or a set of instructions
/// in the intermediate representation.
/// `inst_type` defines the type of instruction, while `parameters`
/// contains a vector of additional instructions or arguments
/// associated with this instruction.
#[derive(Debug, Clone)]
pub struct Instruction {
    inst_type: Type,
    parameters: Vec<Instruction>,
}

/// This Enum enumerates different types of "instructions",
/// including definitions for sections like `.data`, loop
/// constructs, conditions, functions, identifiers,
/// constants, and various data types.
#[derive(Debug, Clone)]
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

    Name(String),
    List,
    Const,              // Lifetime = whole execution, immutable
    Static,             // Lifetime = whole execution, it is mutable though
    Int(i32),
    Bool(bool),
    Float(f32),
    String(String),

    Undefined,          // Testing purposes
}