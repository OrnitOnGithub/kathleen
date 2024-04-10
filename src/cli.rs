use colored::*;

use std::env;
use std::fs;
use std::process;

use crate::error;
use crate::tokenizer::Token;

pub const FILEPATH_ARG: usize = 1;
const OUTPUTPATH_ARG: usize = 2;
const EXTRA_ARGS: usize = 3;

const LICENSE: &'static str = include_str!("../LICENSE");

/// TODO: document
pub fn handle_args() -> (String, String, bool, bool, bool) {

  let args: Vec<String> = env::args().collect();

  // If there are no arguments
  if args.len() < FILEPATH_ARG+1 {
    println!("{}", "Kathleen Programming Language Compiler\n".green());
    println!("{}", LICENSE);
    print_help()
  }
  // If `help` is an argument
  if args.contains(&"help".to_string()) {
    print_help();
  }

  // Set source file (file_path) and output file (output_path) paths.
  let file_path = &args[FILEPATH_ARG];
  let mut output_path: &String =  &String::from("output"); // set a default
  // If there is a 2nd argument (output file name)
  if args.len() >= OUTPUTPATH_ARG+1 {
    output_path = &args[OUTPUTPATH_ARG];
  }

  // If input file does not exist
  if !fs::metadata(&file_path).is_ok() {
    // Throw the error saying it doesnt
    error::print_error(
      error::ErrorCode::InvalidFile,
      Token { token: "".to_string(), line: 0, token_number: 0 },
      &format!("File not found: {}", file_path.green()),
    );
    error::throw_errors();
  }
  // Check if the file extension is `.kl`
  let last_three_chars: String = file_path.chars().rev().take(3).collect();
  if last_three_chars != "lk.".to_string() {
    error::print_error(
      error::ErrorCode::InvalidFileWarning,
      Token { token: "".to_string(), line: 0, token_number: 0 },
      "File does not end with `.kl` extention. Are you sure this is a kathleen file?")
  }

  let mut noasm: bool = false;
  let mut nolink: bool = false;
  let mut keep: bool = false;

  // If there are 3 or more args
  if args.len() > EXTRA_ARGS {
    for i in 3..args.len() {
      // Check for all extra arguments and set to true
      match args[i].as_str() {
        "noasm" => {
          // Stop before assembly.
          // I sould rename this to noass lol
          noasm = true;
        }
        "nolink" => {
          // Stop before linking.
          nolink = true;
        }
        "keep" => {
          // Keep intermediate files.
          keep = true;
        }
        _ => {
          println!("Unknown arg");
        }
      }
    }
  }

  return (file_path.to_string(), output_path.to_string(), noasm, nolink, keep);

}

/// This function shows a help menu with all possible
/// arguments. Never returns as it causes an exit.
fn print_help() -> ! {
  // Show help menu
  // Maybe later put this in a formatted text file, this really sucks.
  println!("{}", "HELP".green());
  println!("{}", "---------".green());
  println!("Usage: {}", "kathleen <arg1> <arg2> <other...".italic().green());
  println!("Argument 1:");
  println!("    - {}", "path to file to compile");
  println!("    - {} {}", "help".green(), "Shows this help menu.");
  println!("Argument 2:");
  println!("    - {}", "name of output file");
  println!("    - {} {}", "if not provided, sets output file name to", "output".green());
  println!("Other arguments:");
  println!("    - {} {}", "noasm".green(), "  Stop the compilation before it assembles the output file.");
  println!("    - {} {}", "nolink".green(), " Stop the compilation before it links the output file.");
  println!("    - {} {}", "keep".green(), "   Don't delete intermediate files (output.asm, output.o).");
  println!("Example usage:");
  println!("    {}", "kathleen hello.kl hello keep".green());
  println!("    {}", "            |       |     |");
  println!("    {}", "         source   output  keep the assembly output and object file as well");
  println!();
  process::exit(1);
}
