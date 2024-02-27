use crate::nar_generator::{NAI,NAR};
use std::fs;

const asm_path: &str = "asm/nasm-x64/";

const output_path: &str = "output.asm";

pub fn generate_asm(nar: NAR) -> String {

    let mut asm: String = String::new();

    for x in nar.data {
        asm += "section .data\n" ;
        asm += &generate_asm_block(x);
    }
    for x in nar.bss {
        asm += "section .bss\n" ;
        asm += &generate_asm_block(x);
    }
    for x in nar.main {
        asm += "section .main\n" ;
        asm += &generate_asm_block(x);
    }

    fn generate_asm_block(nai: NAI) -> String {
        let mut asm: String = String::new();

        match nai {

            NAI::AllocateInt(varname, int) => {
                asm += &replace_values_in_file("allocate_int.asm", vec![varname.as_str(), &int.to_string()])
            }
            NAI::CreatePointer(varname) => {
                asm += &replace_values_in_file("create_bss_pointer.asm", vec![&varname]);
            }

            _ => { todo!() }
        }

        return asm;
    }

    return asm;
}

/// This function takes two parameters:
/// - `filepath: &str` the path to the assembly file to be used as template
/// - `values_to_replace` a list of values to be placed in the assembly file
///
/// This function iterates through the contents of the file until it finds "<>"
/// and replaces those two characters with a value from `values_to_replace`.
/// Next time in encounters "<>" it will replace it with the next item in
/// `values_to_replace`. It then returns the new edited file contents
///
fn replace_values_in_file(filepath: &str, values_to_replace: Vec<&str>) -> String {

    let full_filepath: String = asm_path.to_owned()+filepath;
    let mut contents = fs::read_to_string(full_filepath)
        .expect("Should have been able to read that file. This problem is occurring in asm_generator:read_file()");

    let mut value_to_replace_index: usize = 0;
    for index in 0..contents.len()-1 {
        if contents.as_bytes()[index] == "<".as_bytes()[0] && contents.as_bytes()[index+1] == ">".as_bytes()[0] {
            contents = contents[0..index].to_string() + values_to_replace[value_to_replace_index] + &contents[index+2..contents.len()].to_string();
            value_to_replace_index += 1;
        }
    }
    return contents;
}
