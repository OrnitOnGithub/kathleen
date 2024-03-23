extern crate colored;

use colored::*;
use std::fs::read_to_string;
use crate::tokenizer::Token;
use crate::FILEPATH_ARG;
use std::env;

pub static mut ERROR_COUNT: usize = 0;
pub static mut WARNING_COUNT: usize = 0;
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
    MissingCommandLineArgument,
    InvalidFile,
}

/// This function only prints the errors and does not cause a panic.
/// 
/// `throw_errors` will actually cause the panic but only if
/// `print_errors` was called at least once
///
pub fn print_error(error_code: ErrorCode, token: Token, mut extra_info: &str) {
    
    let mut is_warning: bool = false;   // If an error is only a warning, this will be set to
                                    // true and the error count will not be incremented.

    if extra_info == "" {
        extra_info = "none";
    }

    println!();
    match error_code {
        ErrorCode::UnknownKeyword => {
            println!("Unkown token {} at line {}", token.token.italic(), (token.line+1).to_string().blue());
            show_lines(token);
            println!("Additional information: {}", extra_info.red());
        }

        ErrorCode::IncorrectTypeValuePassed => {
            println!("Incorrect type of value assigned to \"{}\" at line {}", token.token.italic(), (token.line+1).to_string().blue());
            show_lines(token);
            println!("Additional information: {}", extra_info.red());
        }
        ErrorCode::LackingParameters => {
            println!("Incorrect amount of parameters for {} at line {}", token.token.italic(), (token.line+1).to_string().blue());
            show_lines(token);
            println!("Additional information: {} {}", "Fatal error, IR generation cannot proceed, further errors will not be reported.".red(), extra_info.red());
        }
        ErrorCode::ForgotSemicolon => {
            println!("You might have forgotten a semicolon at line {}", (token.line+1).to_string().blue());
            show_lines(token);
            println!("Additional information: {}", extra_info.custom_color(WARNING_COLOUR));
            is_warning = true;
        }
        ErrorCode::VariableNotDefined => {
            println!("Variable {} referenced before assignment at line {}", token.token.italic(), (token.line+1).to_string().blue());
            show_lines(token);
            println!("Additional information: {}", extra_info.red());
        }
        ErrorCode::MissingCommandLineArgument => {
            println!("Missing command line arguments");
            println!("Additional information: {}", extra_info.red());
        }
        ErrorCode::InvalidFile => {
            println!("Invalid file.");
            println!("Additional information: {}", extra_info.red());
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

/// This function causes a panic if errors occured.
/// (if `print_errors()` was called)
/// 
/// Before this panic, if errors occurred, they will already
/// have been printed by `print_error()`
///
pub fn throw_errors() -> () {
    unsafe { println!("{} {}", "Warnings:".custom_color(WARNING_COLOUR), WARNING_COUNT.to_string().custom_color(WARNING_COLOUR)); }
    unsafe { println!("{} {}", "Errors occurred:".red(), ERROR_COUNT.to_string().red()); }
    println!();
    unsafe {
        if ERROR_COUNT > 0 {
            panic!("The intermediate representation could not be generated due to the above errors")
        }
        else {
            println!("No issues found with program. Starting compilation...")
        }
    }
}


/// Show the lines around the problematic one-
/// ```
///  9 |
/// 10 | Something problematic here
/// 11 |
/// ```
/// With nice colours too
fn show_lines(token: Token) -> () {

    // get the file path
    let args: Vec<String> = env::args().collect();
    let file_path = &args[FILEPATH_ARG];
    
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

    let mut line_number_1: String = (line-1).to_string();
    let mut line_number_2: String = (line).to_string();
    let line_number_3: String = (line+1).to_string();
    
    if digit_count_first < digit_count_middle {
        line_number_1 = " ".to_owned() + &(line-1).to_string();
    }
    if digit_count_middle < digit_count_last {
        line_number_1 = " ".to_owned() + &(line-1).to_string();
        line_number_2 = " ".to_owned() + &(line).to_string();
    }

    println!("{} {} {}", line_number_1.blue(), "|".blue(), lines[line-1]);
    println!("{} {} {}", line_number_2.blue(), "|".blue(), lines[line]);
    println!("{} {} {}", line_number_3.blue(), "|".blue(), lines[line+1]);
}