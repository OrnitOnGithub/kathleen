use crate::error::{print_error, ErrorCode, throw_errors}; // For throwing errors.
use crate::ir_generator::{Instruction, Type};

/// This function turns the IR into a near assembly representation. This is 
/// a set of one-to-one instructions that later get turned into blocks of
/// assembly code.
pub fn generate_nar(instructions: Vec<Instruction>) -> NAR {
    let mut data: Vec<NAI> = Vec::new();    // The .data section, where constants and pointers are defined.
    let mut bss: Vec<NAI> = Vec::new();     // The .bss section. This might not get used lol
    let mut main: Vec<NAI> = Vec::new();    // The .main section, called in .text

    for instruction in instructions {
        match instruction.inst_type {

            // Int creation
            Type::Int(value) => {
                let name_struct: Type = instruction.parameters[0].inst_type.clone();
                let varname: String = if let Type::Name(s) = name_struct {
                    s
                } else {
                    panic!("Unexpected enum variant");
                };

                data.push( NAI::CreatePointer(varname.clone()) );
                main.push( NAI::AssignIntToPointer(varname.clone(), value) )
            }
            
            // Print
            Type::Print => {
                let name_struct: Type = instruction.parameters[0].inst_type.clone();
                let varname: String = if let Type::ReferenceTo(s) = name_struct {
                    s
                } else {
                    panic!("Unexpected enum variant");
                };
                main.push( NAI::PrintReferenceTo(varname) );
            }
            Type::PrintLn => {
                main.push(NAI::PrintLn);
            }

            _ => {
                println!("nothing")
            }
        }
    }
    return NAR {
        data,
        bss,
        main,
    }

}

/// This is one near assembly instruction. Each of these gets
/// turned into an assembly block of code.
#[derive(Debug, Clone)]
pub enum NAI {
    CreatePointer(String),
    AssignIntToPointer(String, i32),

    PrintReferenceTo(String),
    Print(String),
    PrintLn,
}
/// This is the near assembly representation struct.
#[derive(Debug, Clone)]
pub struct NAR {
    pub data: Vec<NAI>,
    pub bss: Vec<NAI>,
    pub main: Vec<NAI>,
}