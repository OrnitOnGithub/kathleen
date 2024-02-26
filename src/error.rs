use crate::FILEPATH;

pub static mut ERROR_COUNT: usize = 0;

use std::fs::read_to_string;
use crate::tokenizer::Token;
/// used in the `throw_error()`
pub enum ErrorCode {
    /// Error code for any keyword that is not recognised
    UnknownKeyword,
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

    match error_code {
        ErrorCode::UnknownKeyword => {
            println!();
            println!("Unkown token \"{}\" at line #{}", token.token, (token.line+1));
            show_lines(token.line);
            println!("Additional information: {}", extra_info);
            println!();
        }
        _ => {
            println!();
            println!("Unkown error occurred at line #{}", (token.line+1));
            println!("Additional information: {}", extra_info);
            show_lines(token.line);
            println!();
        }
    }
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
pub fn throw_errors() {
    unsafe { println!("Errors occurred: {}", ERROR_COUNT); }
    println!();
    unsafe {
        if ERROR_COUNT > 0 {
            panic!()
        }
    }
}


/// Show the lines around the problematic one
fn show_lines(line: usize) {

    // Create a Vector for each line of the code.
    let mut lines = Vec::new();
    // Iterate through the lines, add to Vector
    lines.push(String::from(" "));
    for line in read_to_string(FILEPATH).unwrap().lines() {
        lines.push(line.to_string());
    }
    lines.push(String::from(" "));
    let line = line+1;

    println!("{} | {}", line-1, lines[line-1]);
    println!("{} | {}", line, lines[line]);
    println!("{} | {}", line+1, lines[line+1]);
}