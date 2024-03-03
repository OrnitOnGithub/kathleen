//use crate::error::{print_error, ErrorCode, throw_errors}; // For throwing errors.
use crate::ir_generator::{Instruction, Type};

/// This function turns the IR into a near assembly representation. This is 
/// a set of one-to-one instructions that later get turned into blocks of
/// assembly code.
pub fn generate_nar(instructions: Vec<Instruction>) -> NAR {
    let mut data: Vec<NAI> = Vec::new();    // The .data section, where constants and pointers are defined.
    let mut bss: Vec<NAI> = Vec::new();     // The .bss section, mainly used to store heap pointers
    let mut main: Vec<NAI> = Vec::new();    // The .main section, called in .text

    for instruction in instructions {
        match instruction.inst_type {

            // Int creation
            // It should really not be done like this!
            // pointer (8 bytes) -> pointer (8 bytes) -> int (8 bytes)
            // ignoring the first pointer, that's twice as much data per int!
            Type::Int(value) => {
                let name_struct: Type = instruction.parameters[0].inst_type.clone();
                let varname: String = if let Type::Name(s) = name_struct {
                    s
                } else {
                    panic!("Unexpected enum variant");
                };

                bss.push( NAI::CreatePointer(varname.clone()) );
                main.push( NAI::AllocateInt(varname.clone(), value) )
            }

            Type::ConstStr(value) => {
                
            }
            
            // Print an integer
            /*
            Type::PrintInt(varname) => {
                main.push( NAI::PrintInt(varname) );
            }
            */
            
            /*
            Type::PrintConstStr(varname) => {
                
                //let length: usize = varname.len();
                main.push( NAI::PrintConstStr(varname, length) );
            }
            */

            // Only print a newline.
            Type::PrintLn => {
                main.push(NAI::PrintLn);
            }

            _ => {
                println!("DEV: Not an implemented instruction yet")
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
    CreatePointer(String),      // Create a pointer in the .bss section to memory.
    AllocateInt(String, u64),   // Allocate a dword, put the int in it and put the pointer in the BSS pointer's pointed memory region.

    PrintInt(String),           // Print an integer
    PrintConstStr(String, usize),    // Print a constant string. parameters are name of slice and length of slice.
    Print(String),              // Print a constant str, defined in .data
    PrintLn,                    // Print a newline.

    DeclareExterns,
    EndProgram,
}
/// This is the near assembly representation struct.
#[derive(Debug, Clone)]
pub struct NAR {
    pub data: Vec<NAI>,
    pub bss: Vec<NAI>,
    pub main: Vec<NAI>,
}