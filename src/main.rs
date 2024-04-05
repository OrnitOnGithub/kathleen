use std::process::Command;

use std::fs::read_to_string;
use std::fs::File;
use std::fs;

use std::time::Instant;

// Colours in the terminal
use colored::*;

/// This is the code for the tokenizer; first step of compilation.
/// Check `src/tokenizer.rs` for more info.
mod tokenizer;

/// This is the code for the generation of the Intermediate 
/// Representation. Check `src/ir_generator.rs` for more info.
mod ir_generator;

/// This is the code for the generation of the second IR,
/// the Near Assembly Representation. Check `src/ir_generator.rs`
/// for more info.
mod nar_generator;

/// This is the code responsible for generating the assembly
/// output. Check `src/asm_generator.rs` for more info.
mod asm_generator;

/// This is the code for throwing errors.
/// Check `src/error.rs` for more info.
mod error;

/// Module for interacting with the user. Mainly handles
/// command line arguments.
mod cli;

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.
fn main() {
  // get info from the command line arguments
  let (
    file_path, // path to source code
    output_path, // name of output
    noasm, // if true, stop compilation before assembly code gets assembled
    nolink, // if true, stop compilation before object file gets linked
    keep, // if true, don't delete the assembly and object files (intermediate files) after compilation
  ) = cli::handle_args();

  println!("{} {}\n", "Kathleen: compiling".green(), file_path.green().italic());  
  let compilation_start_time = Instant::now();

  // Create a Vector for each line of the code.
  // The tokenizer function will use this to know which line of code
  // each token is in.
  let mut code_lines: Vec<String> = Vec::new();
  // Iterate through the lines, add to Vector
  for line in read_to_string(file_path).unwrap().lines() {
      code_lines.push(line.to_string());
  
  }

  // Compile
  let tokens
    = tokenizer::tokenize(code_lines);

  let intermediate_representation
    = ir_generator::generate_ir(tokens);

  let near_assembly_representation 
    = nar_generator::generate_nar(intermediate_representation);

  let assembly_output
    = asm_generator::generate_asm(near_assembly_representation);
  // Write the assembly output to a file

  let asm_path = &format!("{}.asm", output_path);
  let out_path = &format!("{}.o", output_path);
  
  let _file = File::create(asm_path);
  fs::write(asm_path, assembly_output)
    .expect("Unable to write to file");

  let asm_generation_duration = compilation_start_time.elapsed();
  println!("{}: {:?}\n", "Generated assembly output in".green(), asm_generation_duration);

  // Assemble and link the assembly output file
  let mut issue_during_assembling_or_linking: bool = false;
  if !noasm {
    // run the command `nasm -f elf64 output.asm -o output.o -g`
    let mut assemble = Command::new("nasm");
    assemble.arg("-f").arg("elf64").arg(asm_path).arg("-o").arg(out_path).arg("-g");
    let asm_output = assemble.output()
      .expect("Failed to run command. Is NASM installed?");
    println!("{}: [{}]",
      "Assembling with NASM",
      String::from_utf8(asm_output.stderr.clone()).unwrap().red()
    );
    if !asm_output.stderr.len() == 0 {
      issue_during_assembling_or_linking = true;
    }
  }
  if !nolink && !noasm{
    // run the command `gcc -no-pie output.o -o output -g`
    let mut link = Command::new("gcc");
    link.arg("-no-pie").arg(out_path).arg("-o").arg(output_path.clone()).arg("-g");
    let link_output = link.output()
      .expect("Failed to run command. Is GCC installed?");
    println!("{}: [{}]",

      "Linking with GCC",
      String::from_utf8(link_output.stderr.clone()).unwrap().red()
    );
    if !link_output.stderr.len() == 0 {
      issue_during_assembling_or_linking = true;
    }
  }
 
  if !issue_during_assembling_or_linking {
    let compilation_duration = compilation_start_time.elapsed();
    println!("{} {:?}{}{}",
    "Compilation successful in".green(),
    compilation_duration,
    "! Run the output binary with ./".green(),
    output_path.green().italic()
    );
  }

  if !keep && !noasm && !nolink && !issue_during_assembling_or_linking {
    fs::remove_file(asm_path).expect("DEV: Couldn't remove asm path");
    fs::remove_file(out_path).expect("DEV: Couldn't remove out path");
  }
}
