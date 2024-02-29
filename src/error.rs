use crate::FILEPATH;

pub static mut ERROR_COUNT: usize = 0;

use std::fs::read_to_string;
use crate::tokenizer::Token;
/// used in the `throw_error()`
pub enum ErrorCode {
    /// Error code for any keyword that is not recognised
    UnknownKeyword,
    IncorrectTypeValuePassed,
}

/// This function only prints the errors and does not cause a panic.
/// 
/// `throw_errors` will actually cause the panic but only if
/// `print_errors()` was called at least once
///
pub fn print_error(error_code: ErrorCode, token: Token, mut extra_info: &str) {

    if extra_info == "" {
        extra_info = "none";
    }

    println!();
    match error_code {
        ErrorCode::UnknownKeyword => {
            println!("Unkown token \"{}\" at line #{}", token.token, (token.line+1));
            show_lines(token.line);
            println!("Additional information: {}", extra_info);
        }

        ErrorCode::IncorrectTypeValuePassed => {
            println!("Incorrect type of value \"{}\" passed at #{}", token.token, (token.line+1));
            show_lines(token.line);
            println!("Additional information: {}", extra_info);
        }
    }
    println!();

    unsafe {
        ERROR_COUNT += 1;
    }
}

/// This function causes a panic if errors occured.
/// (if `print_errors()` was called)
/// 
/// Before this panic, if errors occurred, they will already
/// have been printed by `print_error()`
///
pub fn throw_errors() -> () {
    unsafe { println!("Errors occurred: {}", ERROR_COUNT); }
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


/// Show the lines around the problematic one
fn show_lines(line: usize) -> () {
    // 9 10 11
    // 8 9 10

    // Create a Vector for each line of the code.
    let mut lines = Vec::new();
    // Iterate through the lines, add to Vector
    lines.push(String::from(" "));
    for line in read_to_string(FILEPATH).unwrap().lines() {
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

    println!("{} | {}", line_number_1, lines[line-1]);
    println!("{} | {}", line_number_2, lines[line]);
    println!("{} | {}", line_number_3, lines[line+1]);
}