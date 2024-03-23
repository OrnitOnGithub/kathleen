use crate::nar_generator::{NAI,NAR};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "asm/nasm-x64-linux/"]
struct AssemblyDirEmbed;

pub fn generate_asm(nar: NAR) -> String {

    let mut asm: String = String::new();

    /// This function does its thing cant be fucked to document rn
    fn generate_asm_block(nai: NAI) -> String {
        let mut asm: String = String::new();

        match nai {
            NAI::AllocateInt(varname, int) => {
                asm += &replace_values_in_file(
                    "allocate_int.asm",
                    vec![varname.as_str(), &int.to_string()])
            }
            NAI::CreatePointer(varname) => {
                asm += &replace_values_in_file("create_bss_pointer.asm", vec![&varname]);
            }
            NAI::DeclareExterns => {
                asm += &replace_values_in_file("external.asm", vec![]);
            }
            NAI::EndProgram => {
                asm += &replace_values_in_file("endprogram.asm", vec![]);
            }
            NAI::DefineConstStr(name, value, length) => {
                asm += &replace_values_in_file(
                    "define_const_str.asm",
                    vec![&name.to_string(), &value.to_string(), &name.to_string(), &length.to_string()]
                )
            }
            NAI::PrintConstStr(varname) => {
                asm += &replace_values_in_file(
                    "print_constant_string.asm",
                    vec![&varname, varname.as_str()])
            }
            NAI::PrintLn => {
                asm += &replace_values_in_file(
                    "println.asm",
                    vec![])
            }
            _ => { todo!() }
        }

        return asm;
    }

    // DATA
    asm += "section .data\n" ;
    for x in nar.data {
        asm += &generate_asm_block(x);
        asm += "\n";
    }
    asm += &generate_asm_block(NAI::DeclareExterns);
    asm += "\n";
    asm += "\n";

    
    // BSS
    asm += "section .bss\n" ;
    for x in nar.bss {
        asm += &generate_asm_block(x);
        asm += "\n";
    }
    asm += "\n";

    // MAIN
    asm += "section .text\nglobal main\n\nmain:\n" ;
    for x in nar.main {
        asm += &generate_asm_block(x);
        asm += "\n\n";
    }
    asm += &generate_asm_block(NAI::EndProgram);

    return asm;
}

/// This function takes two parameters:
/// - `filepath: &str` the path to the assembly file to be used as template. Note
///     that this creates the path like this: `asmpath+filepath`, asmpath being
///     an already defined constant in the file.
/// - `values_to_replace` a list of values to be placed in the assembly file
///
/// This function iterates through the contents of the file until it finds "<>"
/// and replaces those two characters with a value from `values_to_replace`.
/// Next time in encounters "<>" it will replace it with the next item in
/// `values_to_replace`. It then returns the new edited file contents. This
/// function is also quite unsafe.
///
/// Here's an example usage of the function:
/// ```rust
/// asm += &replace_values_in_file("print_constant.asm", vec!["1", "constantname"]);
/// ```
/// asmpath/print_constant.asm:
/// ```asm
/// mov         rdx,    <>      ; length of the message
/// mov         rsi,    <>      ; pointer to the message
/// mov         rdi,    1
/// mov         rax,    1
/// syscall
/// ```
/// here is what will get appended to `asm`:
/// ```asm
/// mov         rdx,    1      ; length of the message
/// mov         rsi,    constantname      ; pointer to the message
/// mov         rdi,    1
/// mov         rax,    1
/// syscall
/// ```
/// 
fn replace_values_in_file(filepath: &str, values_to_replace: Vec<&str>) -> String {

    // Read the embedded directory (gets compiled into the project) + filepath as a String
    let file = AssemblyDirEmbed::get(filepath).expect("File not found");
    let file_content = std::str::from_utf8(file.data.as_ref()).expect("Failed to convert file data to string");
    let mut contents = file_content.to_string();
    

    // I believe there is a much simpler way to do this in Rust, but I'll keep my own code for funsies
    let mut value_to_replace_index: usize = 0;
    for _ in 0..values_to_replace.len() {   // This for loop is a duct-tape solution. For some reason this didn't actually
                                            // replace all the "<>" in the file, only like the first two
        for index in 0..contents.len()-1 {
            if contents.as_bytes()[index] == "<".as_bytes()[0] && contents.as_bytes()[index+1] == ">".as_bytes()[0] {
                contents = contents[0..index].to_string()
                + values_to_replace[value_to_replace_index]
                + &contents[index+2..contents.len()].to_string();
                value_to_replace_index += 1;
                break // part of the aforementioned duct-tape solution
            }
        }
    }
    return contents;
}