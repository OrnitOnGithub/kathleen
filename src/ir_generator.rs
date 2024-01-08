use crate::error::{print_error, ErrorCode, throw_errors}; // For throwing errors.
use crate::tokenizer::Token;


pub fn generate_ir(tokens: Vec<Token>) {

    let mut instructions: Vec<Instruction> = Vec::new();

    

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