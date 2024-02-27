use crate::nar_generator::{NAI,NAR};

const asm_path: &str = "/asm/nasm-x64";

pub fn generate_asm(nar: NAR) -> String {

    let mut asm: String = String::new();

    for x in nar.data {
        asm += &generate_asm_block(x);
    }

    fn generate_asm_block(nai: NAI) -> String {
        let asm: String = String::new();



        return asm;
    }

    return asm;
}