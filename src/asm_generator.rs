use crate::nar_generator::{NAI,NAR};
use std::fs;

const ASM_PATH: &str = "asm/nasm-x64/";

pub fn generate_asm(nar: NAR) -> String {

    let mut asm: String = String::new();

    /// This function does its thing cant be fucked to document rn
    fn generate_asm_block(nai: NAI) -> String {
        let mut asm: String = String::new();

        match nai {

            NAI::AllocateInt(varname, int) => {
                asm += &replace_values_in_file("allocate_int.asm", vec![varname.as_str(), &int.to_string()])
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
            NAI::PrintConstStr(varname, length) => {
                asm += &replace_values_in_file("print_constant.asm", vec![&length.to_string(), varname.as_str()])
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
/// `values_to_replace`. It then returns the new edited file contents
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

    let full_filepath: String = ASM_PATH.to_owned()+filepath;
    let mut contents = fs::read_to_string(full_filepath)
        .expect("Should have been able to read that file. This problem is occurring in asm_generator:read_file()");

    let mut value_to_replace_index: usize = 0;
    for index in 0..contents.len()-1 {
        if contents.as_bytes()[index] == "<".as_bytes()[0] && contents.as_bytes()[index+1] == ">".as_bytes()[0] {
            contents = contents[0..index].to_string()
                + values_to_replace[value_to_replace_index]
                + &contents[index+2..contents.len()].to_string();
            value_to_replace_index += 1;
        }
    }
    return contents;
}