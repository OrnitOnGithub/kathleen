use std::process;

use std::fs;

use std::time::Instant;

// Colours in the terminal
use colored::*;

/// Module for interacting with the user. Mainly handles
/// command line arguments.
mod cli;

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

// From now on in comments, "the code" refers to the
// programming language this compiler compiles for.
fn main() {
  // get info from the command line arguments
  let parameters: cli::Parameters = cli::handle_args();

  println!("{} {}\n", "Kathleen: compiling".green(), parameters.file_path.green().italic());

  // For funsies, let's count how long compilation takes.
  let compilation_start_time = Instant::now();

  // Create a Vector for each line of the code.
  // The tokenizer function will use this to know which line of code
  // each token is in.
  let mut code_lines: Vec<String> = Vec::new();
  // Iterate through the lines, add to Vector
  for line in fs::read_to_string(parameters.file_path).unwrap().lines() {
    code_lines.push(line.to_string());
  }

  // Compile

  // Tokenize the lines of code
  let tokens
    = tokenizer::tokenize(code_lines);

  // Generate the intermediate representation out of the tokens
  let intermediate_representation
    = ir_generator::generate_ir(tokens);

  // Generate the near assembly representation using the intermediate representation.
  let near_assembly_representation
    = nar_generator::generate_nar(intermediate_representation);

  // Generate the assembly output using the NAR
  let assembly_output
    = asm_generator::generate_asm(near_assembly_representation);

  // The following could totally be done in a different module. Not sure if useful.

  // Define file names for the intermediate files.
  let asm_path = &format!("{}.asm", parameters.output_path);
  let obj_path = &format!("{}.o", parameters.output_path);
  // Then write the assembly output to a file
  let _file = fs::File::create(asm_path);
  fs::write(asm_path, assembly_output)
    .expect("DEV: Unable to write to file");

  let asm_generation_duration = compilation_start_time.elapsed();
  println!("{}: {:?}\n", "Generated assembly output in".green(), asm_generation_duration);

  // Assemble and link the assembly output file
  let mut issue_during_assembling_or_linking: bool = false;
  if !parameters.dont_assemble {
    // run the command `nasm -f elf64 output.asm -o output.o -g`
    let mut assemble = process::Command::new("nasm");
    assemble.arg("-f").arg("elf64").arg(asm_path).arg("-o").arg(obj_path).arg("-g");
    let asm_output = assemble.output()
      .expect("Failed to run command. Is NASM installed?");
    println!("{}: [{}]",
      "Assembling with NASM",
      String::from_utf8(asm_output.stderr.clone()).unwrap().red(),
    );
    // If we encounter an error (stderr is not empty)
    if !asm_output.stderr.len() == 0 {
      issue_during_assembling_or_linking = true;
    }
  }
  if !parameters.dont_link && !parameters.dont_assemble{
    // run the command `gcc -no-pie output.o -o output -g`
    let mut link = process::Command::new("gcc");
    link.arg("-no-pie").arg(obj_path).arg("-o").arg(parameters.output_path.clone()).arg("-g");
    let link_output = link.output()
      .expect("Failed to run command. Is GCC installed?");
    println!("{}: [{}]",
      "Linking with GCC",
      String::from_utf8(link_output.stderr.clone()).unwrap().red(),
    );
    // If we encounter an error (stderr is not empty)
    if !link_output.stderr.len() == 0 {
      issue_during_assembling_or_linking = true;
    }
  }

  if !issue_during_assembling_or_linking &&
     !parameters.dont_assemble           &&
     !parameters.dont_link
  {
    let compilation_duration = compilation_start_time.elapsed();
    println!("{}{:?}{}{}",
      "Compilation successful in ".green(),
      compilation_duration,
      "! Run the output binary with ./".green(),
      parameters.output_path.green().italic()
    );
  }

  if !parameters.keep_intermediate_files &&
     !parameters.dont_assemble           &&
     !parameters.dont_link               &&
     !issue_during_assembling_or_linking
  {
    fs::remove_file(asm_path).expect("DEV: Couldn't remove asm path");
    fs::remove_file(obj_path).expect("DEV: Couldn't remove out path");
  }
}
