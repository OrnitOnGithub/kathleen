use core::panic;
use std::vec;

use crate::error::{self, print_error, throw_errors, ErrorCode}; // For throwing errors.
use crate::tokenizer::Token;

/// Keep track of all variables
#[derive(Clone)]
struct Variable {
    name: String,
    var_type: Type,
}
static mut VARIABLE_LIST: Vec<Variable> = Vec::new();

/// Check a variable's type. Used by IR generator and NAR generator for type-specific handling.
fn get_var_type(token: Token) -> Type {

    let mut var_found: bool = false;
    unsafe {
        for variable in VARIABLE_LIST.clone() {
            if token.token == variable.name {
                var_found = true;
                return variable.var_type;
            }
        }
        if !var_found {
            print_error(ErrorCode::VariableNotDefined, token, "")
        }
    }
    panic!("DEV: tried to check variable type of variable that does not exist.");
}

/// This is a function that turns a Vector of Token structs into the first Vector of (intermediate) Instructions.
/// It returns the first intermediate representation of two. This is the most abstracted one.
pub fn generate_ir(tokens: Vec<Token>) -> Vec<Instruction> {

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
                if tokens.len() > 0 {
                    print_error(ErrorCode::ForgotSemicolon, tokens[0].clone(), "");
                }
                break
            }
            let token: String = tokens[0].token.clone();
            
            match token.as_str() {

                "let" | "const" => {
                    // This is a let binding or a constant creation. A variable is being defined.
                    // EXAMPLE: let varname bool = true;
                    // EXAMPLE: let varname int const = 1234;
                    // EXAMPLE: const varname str = "hello";

                    // check wether it is a constant. `is_constant` is used later throughout
                    // the code, for example to know whether to define a `Str` or `ConstStr`.
                    let mut is_constant = false;
                    if token == "const" {
                        is_constant = true;
                    }

                    // If the expression is less than 6 tokens long including the semicolon,
                    // then it must be incorrect.
                    // let varname int = 12 ;
                    //  1     2     3  4  5 6
                    //  0     1     2  3  4 5  <- if we start at 0, so for indices.
                    if index_of_semicolon < 5 {
                        print_error(ErrorCode::LackingParameters, tokens[0].clone(), "Let bindings work like this: let variable_name data_type = value;");
                        break;
                    }


                    // tokens[1] (the second token) is the variable name.
                    // We store it for later use.
                    let varname: String = tokens[1].token.clone();

                    // find where the (first) `=` is located, because we know everyhing after that is (a) value(s)
                    let mut index_of_equal: usize = 0;
                    for (index, token) in tokens.clone().iter().enumerate() {
                        if token.token == String::from("=") {
                            index_of_equal = index;
                            break
                        }
                    }

                    // match for the third token, which is the data type.  
                    match tokens[2].token.as_str() {

                        // We are creating an unsigned 64 bit integer.
                        // TODO: support for operations after the equal.
                        // we create something like this btw:
                        /*
                        Instruction
                        |---inst_type
                        |     `-Type::Int
                        `---parameters
                              `-Instruction
                                |---inst_type
                                |     `-Type::Name(XYZ)
                                `---parameters
                                      `-[]
                         */
                        "int" => {

                            /// This function returns the correct integer `Type` depending on whether
                            /// is_constant is true or not. It is used later in the code to make creating
                            /// The `Instruction` struct for integers or constant integers easier.
                            /// 
                            /// Return `ConstInt(value)` if int is constant, `Int(value)` otherwise.
                            fn int_type(value: u64, is_constant: bool) -> Type {
                                let mut int_type: Type = Type::Int(value);
                                if is_constant {
                                    int_type = Type::ConstInt(value);
                                }
                                return int_type;
                            }
                            
                            // Return an instruction struct that defines either a ConstInt or an Int.
                            //
                            let instruction: Instruction = Instruction {
                                inst_type: match tokens[index_of_equal + 1].token.clone().as_str().parse::<u64>() {
                                    Ok(value) => int_type(value, is_constant), // int_type returns Type::ConstInt if constant, Type::Int otherwise.
                                    Err(_) => {
                                        error::print_error(
                                            ErrorCode::IncorrectTypeValuePassed,
                                            tokens[2].clone(),
                                            "Value passed was not an unsigned 64 bit integer."
                                        );

                                        return vec![Instruction {
                                            inst_type: Type::Int(0),
                                            parameters: Vec::new(),
                                        }];
                                    }
                                },
                                parameters: vec![Instruction {
                                    inst_type: Type::Name(varname.clone()),
                                    parameters: Vec::new(),
                                }],
                            };

                            instructions.push(instruction);
                            unsafe { VARIABLE_LIST.push(Variable { name: varname.clone(), var_type: int_type(0, is_constant) }) }
                        }

                        "str" => {
                            panic!("DEV: String not implemented yet!");
                        }
                            
                        _ => {
                            print_error(ErrorCode::UnknownKeyword, tokens[2].clone(), "Unknown \
                            data type. The third token of a let binding is a data type.");
                        }
                    };
                }
                
                "print" | "println" => {

                    let varname: String = tokens[1].token.clone();

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
                        
                        let var_type = get_var_type(tokens[index].clone());

                        match var_type.clone() {

                            Type::Int(int) | Type::ConstInt(int) => {

                                // If the variable is constant, do PrintConstInt, otherwise do PrintInt
                                let mut print_int_type: Type = Type::PrintInt(varname.clone());
                                if var_type == Type::ConstInt(int) {
                                    print_int_type = Type::PrintConstInt(varname.clone());
                                }

                                let instruction: Instruction = Instruction {
                                    inst_type: print_int_type, // Either PrintConstInt or PrintInt
                                    parameters: vec![
                                        Instruction {
                                            inst_type: Type::Name(tokens[index].token.clone()),
                                            parameters: Vec::new(),
                                        }
                                    ]
                                };
                                instructions.push(instruction);
                            }

                            Type::Str(str) | Type::ConstStr(str) => {

                                // If the variable is constant, do PrintConstStr, otherwise do PrintStr
                                let mut print_str_type: Type = Type::PrintStr(varname.clone());
                                if var_type == Type::ConstStr(str) {
                                    print_str_type = Type::PrintConstStr(varname.clone());
                                }

                                let instruction: Instruction = Instruction {
                                    inst_type: print_str_type, // Either PrintConstStr or PrintStr
                                    parameters: vec![
                                        Instruction {
                                            inst_type: Type::Name(tokens[index].token.clone()),
                                            parameters: Vec::new(),
                                        }
                                    ]
                                };
                                instructions.push(instruction);
                            }

                            _ => {
                                todo!("DEV: Tried to print non-implemented data type");
                            }
                        }
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
                    print_error(ErrorCode::UnknownKeyword, tokens[0].clone(), "This token is not a supported instruction.")
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
#[derive(Debug, Clone, PartialEq)]
pub enum Type {

    Function,

    Loop,
    LoopBreak,
    
    Condition,          // Define the evaluation
    ConditionTrue,      // Where to go if TRUE Basically
    ConditionFalse,     // Where to go if FALSE
    
    Name(String),
    Int(u64),
    ConstInt(u64),
    Str(String),
    ConstStr(String),

    PrintInt(String),
    PrintConstInt(String),
    PrintStr(String),
    PrintConstStr(String),
    PrintLn,
    ReferenceTo(String),// Like name but we put [] around it in assembly
}