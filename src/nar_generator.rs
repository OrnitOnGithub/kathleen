//use crate::error::{print_error, ErrorCode, throw_errors}; // For throwing errors.
use crate::ir_generator::{Instruction, Type};

/// This function turns the IR into a near assembly representation. This is 
/// a set of one-to-one instructions that later get turned into blocks of
/// assembly code.
/// 
/// Takes as parameter a vector of `Instruction`s and turns it into a `NAR` struct,
/// which contains all necessary information for easy 1-to-1 conversion into assembly.
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
  /// Create a pointer in the .bss section to memory.
  /// (qword size pointer (normal pointer))
  /// - String = name
  CreatePointer(String),
  /// Allocate a qword, put the int in it and put the pointer
  /// in the BSS pointer's pointed memory region.
  /// This is a proof of concept for dynamic memory and should not be done like this.
  /// - String: name
  /// - u64: value
  AllocateInt(String, u64),
  /// Define a constant string's name and value, also define its size (for easier printing)
  /// - String: name
  /// - String: value
  /// - usize: size
  DefineConstStr(String, String, usize),

  /// Increment an integer
  /// - String: name of integer to increment
  Increment(String),

  /// Print an integer
  /// - String: name of integer
  PrintInt(String),
  /// Print a constant string.
  /// - String: name of variable.
  PrintConstStr(String),  
  /// Just print a newline
  PrintLn,

  /// Call the named loop label, and also define the exit label right under the call
  /// - String: loop name
  LoopCall(String),
  /// Define the label for the loop
  /// - String: loop name
  LoopDefine(String),        // MARK: Break

  /// Exit the named loop by jumping to its exit label.
  /// - String: loop name
  LoopExit(String),
  /// Call the loop label
  /// - String: loop name
  LoopRepeat(String),

  /// Do the program exit syscall
  EndProgram,
  /// Define all the utility functions
  StdLib,
}
/// This is the near assembly representation struct.
/// It contains the 3 main sections
/// - section .data
/// - section .bss
/// - section .text <br>  global main <br> main:
/// As well as an extra buffer to add other functions and such.
#[derive(Debug, Clone)]
pub struct NAR {
  pub data: Vec<NAI>,
  pub bss: Vec<NAI>,
  pub main: Vec<NAI>,
  pub other: Vec<NAI>,
}