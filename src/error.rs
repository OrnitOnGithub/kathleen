extern crate colored;
use colored::*;

use std::fs::read_to_string;
use crate::tokenizer::Token;
use crate::cli::FILEPATH_ARG_INDEX;
use std::env;
use std::process;

static mut ERROR_COUNT: usize = 0;
static mut WARNING_COUNT: usize = 0;
const WARNING_COLOUR: CustomColor = CustomColor { r: 255, g: 200, b: 50};

/// used in the `throw_error()`
pub enum ErrorCode {
  /// Error code for any keyword that is not recognised
  UnknownKeyword,
  /// Error code for when an incorrect data type is passed, for example
  /// `let banana int = "12";`
  IncorrectTypeValuePassed,
  LackingParameters,
  ForgotSemicolon,
  VariableNotDefined,
  InvalidFile,
  InvalidFileWarning,
  CannotFindCounterpart,
}

/// This function only prints the errors and does not cause exiting the program.
/// 
/// `throw_errors` will actually cause the exit but only if
/// `print_errors` was called at least once
///
pub fn print_error(error_code: ErrorCode, token: Token, extra_info: &str) {
  
  let mut is_warning: bool = false;   // If an error is only a warning, this will be set to
                                      // true and the error count will not be incremented.

  let mut colored_extra_info = extra_info.red();
  if extra_info == "" {
    colored_extra_info = "none".red();
  }

  println!();
  match error_code {
    ErrorCode::UnknownKeyword => {
      println!("Unkown token {} at line {}", token.token.italic(), (token.line+1).to_string().blue());
      show_lines(token);
      println!("Additional information: {}", colored_extra_info);
    }
    ErrorCode::IncorrectTypeValuePassed => {
      println!("Incorrect type of value assigned to \"{}\" at line {}", token.token.italic(), (token.line+1).to_string().blue());
      show_lines(token);
      println!("Additional information: {}", colored_extra_info);
    }
    ErrorCode::LackingParameters => {
      println!("Incorrect amount of parameters for {} at line {}", token.token.italic(), (token.line+1).to_string().blue());
      show_lines(token);
      println!("Additional information: {} {}", "Fatal error, IR generation cannot proceed, further errors will not be reported.".red(), colored_extra_info);
    }
    ErrorCode::ForgotSemicolon => {
      println!("You might have forgotten a semicolon at line {}", (token.line+1).to_string().blue());
      show_lines(token);
      println!("Additional information: {}", colored_extra_info.custom_color(WARNING_COLOUR));
      is_warning = true;
    }
    ErrorCode::VariableNotDefined => {
      println!("Variable {} referenced before assignment at line {}", token.token.italic(), (token.line+1).to_string().blue());
      show_lines(token);
      println!("Additional information: {}", colored_extra_info);
    }
    ErrorCode::InvalidFile => {
      println!("Invalid file.");
      println!("Additional information: {}", colored_extra_info);
    }
    ErrorCode::InvalidFileWarning => {
      println!("Potentially invalid file.");
      println!("Additional information: {}", colored_extra_info.custom_color(WARNING_COLOUR));
      is_warning = true;
    }
    ErrorCode::CannotFindCounterpart => {
      println!("Token {} on line {} has no matching counterpart.", token.token.italic(), (token.line+1).to_string().blue());
      show_lines(token);
      println!("Additional information: {}", colored_extra_info);
    }
  }
  println!();

  unsafe {
    if is_warning {
      WARNING_COUNT += 1;
    }
    else {
      ERROR_COUNT += 1;
    }
  }
}

/// This function causes an exit if errors occured.
/// (if `print_errors()` was called)
/// 
/// Before this exit, if errors occurred, they will already
/// have been printed by `print_error()`
///
pub fn throw_errors() {
  unsafe { println!("{} {}", "Warnings:".custom_color(WARNING_COLOUR), WARNING_COUNT.to_string().custom_color(WARNING_COLOUR)); }
  unsafe { println!("{} {}", "Errors occurred:".red(), ERROR_COUNT.to_string().red()); }
  println!();
  unsafe {
    if ERROR_COUNT > 0 {
      process::exit(1);
    }
    else {
      println!("No issues found with program. Starting compilation...")
    }
  }
}

/// Show the lines around the problematic one
/// ```
///  9 |
/// 10 | Something problematic here
/// 11 |
/// ```
/// With nice colours too
fn show_lines(token: Token) -> () {

  // get the file path
  let args: Vec<String> = env::args().collect();
  let file_path = &args[FILEPATH_ARG_INDEX];
  
  let line: usize = token.line;

  // Create a Vector for each line of the code.
  let mut lines = Vec::new();
  // Iterate through the lines, add to Vector
  lines.push(String::from(" "));
  for line in read_to_string(file_path).unwrap().lines() {
    lines.push(line.to_string());
  }
  lines.push(String::from(" "));
  let line = line+1;

  // Look at the digit count of each line and add a space in front accordingly
  // example:
  // "9", "10", "11" => " 9", "10", "11"
  //  9 |   <=== the extra space is to align everything
  // 10 |
  // 11 |
  let digit_count_first: u32 = (line-1).checked_ilog10().unwrap_or(0) + 1;
  let digit_count_middle: u32 = (line).checked_ilog10().unwrap_or(0) + 1;
  let digit_count_last: u32 = (line+1).checked_ilog10().unwrap_or(0) + 1;

  let mut line_1: String = (line-1).to_string();
  let mut line_2: String = (line).to_string();
  let line_3: String = (line+1).to_string();
  
  if digit_count_first < digit_count_middle {
    line_1 = " ".to_owned() + &(line-1).to_string();
  }
  if digit_count_middle < digit_count_last {
    line_1 = " ".to_owned() + &(line-1).to_string();
    line_2 = " ".to_owned() + &(line).to_string();
  }

  println!("{} {} {}", line_1.blue(), "|".blue(), lines[line-1]);
  println!("{} {} {}", line_2.blue(), "|".blue(), lines[line]);
  println!("{} {} {}", line_3.blue(), "|".blue(), lines[line+1]);
}