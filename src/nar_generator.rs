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
  static mut OTHER: Vec<NAI> = Vec::new();  // Logic inside logic, usually

  create_nais(instructions, false);

  fn create_nais(instructions: Vec<Instruction>, is_embedded: bool) -> () {

    let mut data: Vec<NAI> = Vec::new();
    let mut bss: Vec<NAI> = Vec::new();
    let mut main: Vec<NAI> = Vec::new();
    let mut other: Vec<NAI> = Vec::new();

    for instruction in instructions {
      match instruction.inst_type {

        Type::Int(value) => {
          // Int creation
          // It should really not be done like this!
          // pointer (8 bytes) -> pointer (8 bytes) -> int (8 bytes)
          // ignoring the first pointer, that's twice as much data per int!
          // This is a proof of concept to apply to dynamically sized
          // data types later.
          let name_struct: Type = instruction.parameters[0].inst_type.clone();
          let variable_name: String = if let Type::Name(s) = name_struct {
            s
          } else {
            panic!("DEV: Unexpected enum variant");
          };
          
          bss.push( NAI::CreatePointer(variable_name.clone()) );
          main.push( NAI::AllocateInt(variable_name.clone(), value) );
        }
        
        Type::ConstStr(value) => {
          // Definition of a constant String
          let name_struct: Type = instruction.parameters[0].inst_type.clone();
          let variable_name: String = if let Type::Name(s) = name_struct {
            s
          } else {
            panic!("DEV: Unexpected enum variant");
          };
          
          data.push(NAI::DefineConstStr(variable_name, value.clone(), value.len()));
        }
        
        Type::PrintInt(variable_name) => {
          // Print an integer
          main.push( NAI::PrintInt(variable_name) );
        }
        
        Type::PrintConstStr(variable_name) => {
          main.push( NAI::PrintConstStr(variable_name) );
        }
        
        Type::PrintLn => {
          // Only print a newline.
          main.push(NAI::PrintLn);
        }

        Type::Loop(loop_name) => {
          unsafe {
            let loopname = format!("{}_{}", "loop", loop_name);
            main.push(NAI::LoopCall(loopname.clone()));
            OTHER.push(NAI::LoopDefine(loopname.clone()));
            create_nais(instruction.parameters, true);
            OTHER.push(NAI::LoopRepeat(loopname.clone()));
          }
        }

        Type::LoopExit(loop_name) => {
          let loopname = format!("{}_{}", "loop", loop_name);
          other.push(NAI::LoopExit(loopname))
        }

        Type::Increment(variable_name) => {
          main.push(NAI::Increment(variable_name));
        }
        
        _ => {
          panic!("DEV: NAR: Not an implemented instruction yet: {:?}", instruction);
        }
      }

      unsafe {
        BSS.append(&mut bss);
        DATA.append(&mut data);
        if is_embedded {
          OTHER.append(&mut main);
        }
        else {
          MAIN.append(&mut main);
        }
        OTHER.append(&mut other);
      }
    }
  }

  unsafe {
    let data = DATA.clone();
    let bss = BSS.clone();
    let main = MAIN.clone();
    let other = OTHER.clone();
    return NAR {
      data,
      bss,
      main,
      other,
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

  Increment(String),

  PrintInt(String),       // Print an integer
  PrintConstStr(String),  // Print a constant string. String = name of variable.
  PrintLn,                // Print a newline.

  LoopCall(String),       
  LoopDefine(String),
  LoopExit(String),
  LoopRepeat(String),

  EndProgram,
  StdLib,
}
/// This is the near assembly representation struct.
#[derive(Debug, Clone)]
pub struct NAR {
  pub data: Vec<NAI>,
  pub bss: Vec<NAI>,
  pub main: Vec<NAI>,
  pub other: Vec<NAI>,
}