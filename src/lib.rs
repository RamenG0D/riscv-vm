pub mod bus;
pub mod cpu;
pub mod csr;
pub mod devices;
pub mod logging;
pub mod memory;
pub mod registers;
pub mod trap;

#[inline]
pub fn convert_memory(mem: &[u8]) -> Vec<u32> {
    let mut program = Vec::new();
    for bytes in mem.chunks_exact(4) {
        let word = {
            let word = u32::from_ne_bytes(bytes.try_into().unwrap());
            word.to_le()
        };
        program.push(word);
    }
    program
}

pub fn disassemble(program: &[u32], file: &str) {
    use riscv_decoder::decoder::*;
    use std::{fs::File, io::Write};

    let mut file = File::create(file).expect("Failed to create file");

    let mut pc = 0_usize;
    while pc < program.len() {
        // debug_assert!(pc % 4 != 0, "Pc must be aligned to 4 bytes {{ PC: {pc:#X} }}");
        match program.get(pc) {
            Some(&inst) => {
                let dinst = try_decode(inst); /*if is_compressed(inst) {
                                                  pc += 2; try_decode_compressed(inst)
                                              } else {
                                                  pc += 4; try_decode(inst)
                                              };*/
                pc += 4;
                writeln!(
                    file,
                    "{:#X}: {}",
                    pc + memory::dram::DRAM_BASE as usize,
                    match dinst {
                        Ok(inst) => format!("{inst}"),
                        Err(e) => format!("Error => {e} {{ instruction: {inst:#X} }}"),
                    }
                )
                .expect("Failed to write to file");
            }
            None => {
                writeln!(file, "{pc:#010x}: EOF / End of indexs").expect("Failed to write to file");
                break;
            }
        }
    }
}

// internal export
pub mod bit_ops {
    pub use bit_ops::bitops_u32::*;

    pub fn zero_extend(value: u32) -> u32 {
        clear_bit(value, 31)
    }
}
