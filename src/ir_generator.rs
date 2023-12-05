pub fn generate_ir(mut tokenized_lines: Vec<Vec<String>>) -> Vec<Instruction> {
    // Vec<Vec<String>>
    // outer Vector (Vec<Vec<String>>): line
    // inner Vector (Vec<String>): word (String)

    let mut instructions: Vec<Instruction> = Vec::new();

    for mut line in tokenized_lines.into_iter() {
        instructions.push(create_instruction(&mut line));
    }

    fn create_instruction(line: &mut Vec<String>) -> Instruction {
        
        while (line.len() > 0) {

            let word = line[0].clone();
            
            line.remove(0);
            
            match &word as &str {
                "{" => {
                    return Instruction {
                        inst_type: Type::Scope,
                        parameters: vec![create_instruction(line)],
                    };
                }
                
                _ => {
                    // replace this with unrecognised keyword error
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
