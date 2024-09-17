use riscv_vm::{cpu::Cpu, memory::dram::DRAM_BASE};
use log::{error, info};

pub fn main() {
    let program = include_bytes!("fs.img");

    riscv_vm::logging::init_logging();

    let mut cpu = Cpu::new();

    cpu.load_program_raw(program).expect("Failed to load program");

    while cpu.get_pc() < (DRAM_BASE + program.len()) as u32 {
        match cpu.step() {
            Ok(_) => info!("PC: {}", cpu.get_pc()),
            Err(e) => error!("Error: {e}"),
        }
    }
}
