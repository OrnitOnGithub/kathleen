use core::panic;
use std::vec;
use std::iter::FromIterator;

use crate::error::{self, print_error, throw_errors, ErrorCode}; // For throwing errors.
use crate::tokenizer::Token;

// TODO: document
#[derive(Clone)]
struct Variable {
  name: String,
  var_type: Type,
}
/// Keep track of all variables
static mut VARIABLE_LIST: Vec<Variable> = Vec::new();

/// Check a variable's type. Used by IR generator and NAR generator for type-specific handling.
fn var_type(token: Token) -> Type {

  unsafe {
    for variable in VARIABLE_LIST.clone() {
      if token.token == variable.name {
        return variable.var_type;
      }
    }
    // this implementation is garbage, there should be a preliminary
    // step where all variables are found beforehand.
    print_error(ErrorCode::VariableNotDefined, token, "")
  }
  panic!("DEV: tried to check variable type of variable that does not exist.");
}

/// This is a function that turns a Vector of Token structs into the first Vector of (intermediate) Instructions.
/// It returns the first intermediate representation of two. This is the most abstracted one.
pub fn generate_ir(tokens: Vec<Token>) -> Vec<Instruction> {

  // This is what a Token struct looks like btw:
  /*
  {
    token: String,       // the token itself
    line: usize,         // which line this at
    token_number: usize, // which token in the line this is (1st, 2nd...)
  }
  */

  let ir: Vec<Instruction> = create_instructions(tokens);
  
  throw_errors(); // cause panic if there were any errors

  return ir;


  // Put into a second function in case an implementation requires recursion,
  // or this could also be helpful for implementing functions, not necessarily
  // with recursion.
  fn create_instructions(mut tokens_to_process: Vec<Token>) -> Vec<Instruction> {

    let mut instructions_to_return: Vec<Instruction> = Vec::new();

    loop {
      // a loop where:
      // - Keywords are handled
      // - a for loop finds a ; and deletes everything before that semicolon.
      // - and starts again, until we have reached the last semicolon.
      let mut index_of_semicolon: usize = index_first_occurence_of(tokens_to_process.clone(), String::from(";"));

      if index_of_semicolon == 0 {
        if tokens_to_process.len() > 0 {
          print_error(ErrorCode::ForgotSemicolon, tokens_to_process[0].clone(), "");
        }
        break
      }

      let instruction_token: String = tokens_to_process[0].token.clone();
      
      match instruction_token.as_str() {

        // MARK: Increment
        "inc" => {
          // Increment an integer. A very simple instruction.

          let variable_to_increment: String = tokens_to_process[1].token.clone();
          
          let instruction: Instruction = Instruction {
            inst_type: Type::Increment(variable_to_increment),
            parameters: vec![],
          };
          instructions_to_return.push(instruction);
        }

        // MARK: Loop
        "loop" => {
          // EXAMPLE:
          // loop loop_name {
          //   do something idk;
          // }

          let loop_name = tokens_to_process[1].token.clone();

          // A loop, added only as proof of concept for recursive packing and unpacking
          // of the IR.
          let index_of_next_closed_curly_brace: usize
            = closing_curly_brace_index(tokens_to_process.clone());
          let tokens_inside_loop: Vec<Token>
            = Vec::from_iter(tokens_to_process[3..index_of_next_closed_curly_brace].iter().cloned());
          let instruction: Instruction = Instruction {
            inst_type: Type::Loop(loop_name),
            parameters: create_instructions(tokens_inside_loop),
          };
          instructions_to_return.push(instruction);

          // delete all tokens before curly brace
          for _ in 0..index_of_next_closed_curly_brace {
            tokens_to_process.remove(0);
          }
          index_of_semicolon = 0; // Act as if the closing bracket was the semicolon (end of instruction)
        }

        // MARK: Break
        "break" => {
          // EXAMPLE: break loop_name;

          let loop_name = tokens_to_process[1].token.clone();

          let instruction: Instruction = Instruction {
            inst_type: Type::LoopExit(loop_name),
            parameters: vec![]
          };

          instructions_to_return.push(instruction);
        }

        // MARK: Let | Const
        "let" | "const" => {
          // This is a let binding or a constant creation. A variable is being defined.
          // EXAMPLE: let varname int = 1234;
          // EXAMPLE: const varname str = "hello";

          // check wether it is a constant. `is_constant` is used later throughout
          // the code, for example to know whether to define a `Str` or `ConstStr`.
          let mut is_constant = false;
          if instruction_token == "const" {
            is_constant = true;
          }

          // If the expression is less than 6 tokens long including the semicolon,
          // then it must be incorrect.
          // let varname int = 12 ;
          //  0     1     2  3  4 5
          if index_of_semicolon < 5 {
            print_error(
              ErrorCode::LackingParameters,
              tokens_to_process[0].clone(),
              "Let bindings work like this: let variable_name data_type = value;"
            );
            break;
          }

          // tokens[1] (the second token) is the variable name.
          // We store it for later use.
          let varname: String = tokens_to_process[1].token.clone();

          // find where the (first) `=` is located, because we know everyhing after that is (a) value(s)
          let index_of_equal: usize = index_first_occurence_of(tokens_to_process.clone(), String::from("="));

          // match for the third token, which is the data type.  
          match tokens_to_process[2].token.as_str() {

            // We are creating an unsigned 64 bit integer.
            // TODO: support for operations after the equal.
            // we create something like this btw:
            /*
            Instruction
            |---inst_type
            |   `-Type::Int(123)
            `---parameters
                `-Instruction
                |---inst_type
                |   `-Type::Name(XYZ)
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
              // I don't remember how this works because I didn't document it whenever I made it,
              // like 3 months ago as of this comment.
              let instruction: Instruction = Instruction {
                inst_type: match tokens_to_process[index_of_equal + 1].token.clone().as_str().parse::<u64>() {
                  Ok(value) => int_type(value, is_constant), // int_type returns Type::ConstInt if constant, Type::Int otherwise.
                  Err(_) => {
                    error::print_error(
                      ErrorCode::IncorrectTypeValuePassed,
                      tokens_to_process[2].clone(),
                      "Value passed was not an unsigned 64 bit integer. (0-18446744073709551615)"
                    );
                    // Return something random because we caused an error anyways
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

              instructions_to_return.push(instruction);
              unsafe { VARIABLE_LIST.push(Variable { name: varname.clone(), var_type: int_type(0, is_constant) }) }
            }

            "str" => {
              /// Same as `int_type` but with str.
              fn str_type(value: String, is_constant: bool) -> Type {
                let mut int_type: Type = Type::Str(value.clone());
                if is_constant {
                  int_type = Type::ConstStr(value);
                }
                return int_type;
              }

              let instruction: Instruction = Instruction {
                inst_type: str_type(tokens_to_process[index_of_equal+1].token.clone(), is_constant),
                parameters: vec![
                  Instruction {
                    inst_type: Type::Name(varname.clone()),
                    parameters: Vec::new(),
                  },
                ]
              };

              instructions_to_return.push(instruction);
              unsafe { VARIABLE_LIST.push(Variable { name: varname, var_type: str_type(String::new(), is_constant) })}

            }
              
            _ => {
              print_error(ErrorCode::UnknownKeyword, tokens_to_process[2].clone(), "Unknown \
              data type. The third token of a let binding is a data type.");
            }
          };
        }
        
        // MARK: Print | PrintLn
        "print" | "println" => {
          // EXAMPLE: println(var var2)
          // EXAMPLE: print(var var2);
          // EXAMPLE: println(var);
          // EXAMPLE: print(var);

          //let varname: String = tokens[1].token.clone(); // WHAT???

          let mut print_line: bool = false;
          if instruction_token == "println" {
            print_line = true;
          }

          let index_of_open_bracket: usize = index_first_occurence_of(tokens_to_process.clone(), String::from("("));
          let index_of_closed_bracket: usize = index_first_occurence_of(tokens_to_process.clone(), String::from(")"));

          for varname_index in index_of_open_bracket+1..index_of_closed_bracket {

            let varname = tokens_to_process[varname_index].token.clone();
            let var_type = var_type(tokens_to_process[varname_index].clone());

            match var_type.clone() {

              // PROBLEM
              // This is reduntant for no reason wtf??
              // Type::Print(String) contains the name,
              // no need to put it in parameters. FIX THIS later.

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
                      inst_type: Type::Name(tokens_to_process[varname_index].token.clone()),
                      parameters: Vec::new(),
                    }
                  ]
                };
                instructions_to_return.push(instruction);
              }

              Type::Str(str) | Type::ConstStr(str) => {
                // If the variable is constant, do PrintConstStr, otherwise do PrintStr
                let mut print_str_type: Type = Type::PrintStr(varname.clone());
                if var_type == Type::ConstStr(str) {
                  print_str_type = Type::PrintConstStr(varname.clone());
                }
                let instruction: Instruction = Instruction {
                  inst_type: print_str_type, // Either PrintConstStr or PrintStr
                  parameters: vec![]
                };
                instructions_to_return.push(instruction);
              }

              _ => {
                todo!("DEV: Tried to print non-implemented data type");
              }
            }
          }

          // If the instruction is "println", then add the PrintLn
          // Instruction to the list of instructions. This instruction
          // only prints a newline.
          if print_line {
            let instruction: Instruction = Instruction {
              inst_type: Type::PrintLn,
              parameters: Vec::new(),
            };
            instructions_to_return.push(instruction);
          }
        }

        // MARK: _ =>
        _ => {
          print_error(ErrorCode::UnknownKeyword, tokens_to_process[0].clone(), "This token is not a supported instruction.")
        }
      }
      
      // end of loop here
      // delete all tokens before semicolon
      for _ in 0..index_of_semicolon+1 {
        tokens_to_process.remove(0);
      }
    }
    return instructions_to_return
  }
}

/// Get the index of the curly brace that closes the block the start of `tokens` is in.
/// For example, if we had the following in `Vec<Token>` form:
/// ```
/// {     // 0
///   {   // 1
/// 
///   }   // 2
/// }     // 3
/// }     // 4
/// ```
/// It would return `3`, because that corresponds to the curly brace that closes the first
/// open one.
fn closing_curly_brace_index(tokens: Vec<Token>) -> usize {
  let mut open_brace_counter: usize = 0;

  for (index_of_token, token) in tokens.iter().enumerate() {
    if token.token.as_str() == "{" {
      open_brace_counter += 1;
    }
    if token.token.as_str() == "}" {
      open_brace_counter -= 1;
      if open_brace_counter == 0 {
        return index_of_token
      }
    }
  }
  // If we reach here, curly brace was not found.

  let index_of_first_curly_brace = index_first_occurence_of(tokens.clone(), String::from("{"));

  print_error(
    ErrorCode::CannotFindCounterpart,
    tokens[index_of_first_curly_brace].clone(),
    "This curly brace has no closing curly brace to go with it. You are missing a closed curly brace `}` somewhere.");
  throw_errors();
  return 0;
}

fn index_first_occurence_of(tokens: Vec<Token>, query: String) -> usize {
  let mut index_of_occurence: usize = 0;
  for (index, token) in tokens.clone().iter().enumerate() {
    if token.token == query {
      index_of_occurence = index;
      break
    }
  }
  return index_of_occurence
}

// TODO: RETHINK & REORGANISE this mess of an IR

/// Represents an instruction or a set of instructions
/// in the intermediate representation.
/// `inst_type` defines the type of instruction, `parameters`
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
  /// Increments an integer.
  /// - String: name of the integer to increment
  /// - Needs no additional parameters (in `Instruction` struct)
  Increment(String),
  /// Creates a loop
  /// - String: name of the loop
  /// - Parameters: Instructions to loop over
  Loop(String),
  /// Exits from the named loop - "break"
  /// - String: name of loop
  LoopExit(String),
  /// A kind of useless enum variant that needs to be removed
  Name(String),
  /// Define a 64 bit integer
  /// - u64: value
  /// - Uses Name enum variant to define its name in an Instruction in parameters.
  ///   Should probably be changed.
  Int(u64),
  /// Define a constant integer
  /// - not implemented
  ConstInt(u64),
  /// Define a dynamic string
  /// - not implemented
  Str(String),
  /// Define a constant string
  /// - String: Value
  /// - Name in parameters: name
  ConstStr(String),
  // TODO: maybe print should be a single enum variant for all types and nar_generator deals with the bull.sh
  /// Print an integer
  /// - String: name
  PrintInt(String),
  /// Print a constant integer - not implemented
  PrintConstInt(String),
  /// Print a dynamic string - not implemented
  PrintStr(String),
  /// Print a constant string
  /// - String: name of var to print
  PrintConstStr(String),
  /// Print a newline
  PrintLn,
}