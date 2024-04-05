//use crate::error::{print_error, ErrorCode, throw_errors}; // For throwing errors.
use crate::ir_generator::{Instruction, Type};

/// This function turns the IR into a near assembly representation. This is 
/// a set of one-to-one instructions that later get turned into blocks of
/// assembly code.
pub fn generate_nar(instructions: Vec<Instruction>) -> NAR {
  // This implementation kinda hurts my balls
  static mut DATA: Vec<NAI> = Vec::new();  // The .data section, where constants are defined.
  static mut BSS: Vec<NAI> = Vec::new();   // The .bss section, mainly used to store heap pointers
  static mut MAIN: Vec<NAI> = Vec::new();  // The main label, called in .text section

  create_nais(instructions);

  fn create_nais(instructions: Vec<Instruction>) -> () {
    for instruction in instructions {
      match instruction.inst_type {

        // Int creation
        // It should really not be done like this!
        // pointer (8 bytes) -> pointer (8 bytes) -> int (8 bytes)
        // ignoring the first pointer, that's twice as much data per int!
        // This is a proof of concept to apply to dynamically sized
        // data types later.
        Type::Int(value) => {
          let name_struct: Type = instruction.parameters[0].inst_type.clone();
          let varname: String = if let Type::Name(s) = name_struct {
            s
          } else {
            panic!("DEV: Unexpected enum variant");
          };
          
          unsafe { BSS.push( NAI::CreatePointer(varname.clone()) ); }
          unsafe { MAIN.push( NAI::AllocateInt(varname.clone(), value) ); }
        }
        
        Type::ConstStr(value) => {
          let name_struct: Type = instruction.parameters[0].inst_type.clone();
          let varname: String = if let Type::Name(s) = name_struct {
            s
          } else {
            panic!("DEV: Unexpected enum variant");
          };
          
          unsafe { DATA.push(NAI::DefineConstStr(varname, value.clone(), value.len())) }
        }
        
        // Print an integer
        Type::PrintInt(varname) => {
          unsafe { MAIN.push( NAI::PrintInt(varname) ); }
        }
        
        Type::PrintConstStr(varname) => {
          unsafe { MAIN.push( NAI::PrintConstStr(varname) ); }
        }
        
        // Only print a newline.
        Type::PrintLn => {
          unsafe { MAIN.push(NAI::PrintLn); }
        }
        
        _ => {
          panic!("DEV: NAR: Not an implemented instruction yet: {:?}", instruction);
        }
      }
    }
  }

  unsafe {
    let data = DATA.clone();
    let bss = BSS.clone();
    let main = MAIN.clone();
    return NAR {
      data,
      bss,
      main,
    }
  }
}
  
  /// This is one near assembly instruction. Each of these gets
  /// turned into an assembly block of code.
  #[derive(Debug, Clone)]
  pub enum NAI {
    CreatePointer(String),                 // Create a pointer in the .bss section to memory. (For a qword of data)
    AllocateInt(String, u64),              // Allocate a qword, put the int in it and put the pointer in the BSS pointer's pointed memory region.
    DefineConstStr(String, String, usize), // Define a constant string's name and value, also define its size (for easier printing)
    //               name  value   size

  PrintInt(String),       // Print an integer
  PrintConstStr(String),  // Print a constant string. String = name of variable.
  PrintLn,                // Print a newline.

  EndProgram,
  StdLib
}
/// This is the near assembly representation struct.
#[derive(Debug, Clone)]
pub struct NAR {
  pub data: Vec<NAI>,
  pub bss: Vec<NAI>,
  pub main: Vec<NAI>,
}