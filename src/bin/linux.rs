use riscv_vm::cpu::Cpu;
use log::error;

pub fn main() {
    riscv_vm::logging::init_logging();

    let mut cpu = Cpu::new();

    let program = include_bytes!("../../xv6-rv32/fs.img");

    cpu.load_program_raw(program).expect("Failed to load program");

    while cpu.get_pc() < program.len() as u32 {
        match cpu.step() {
            Ok(_) => (),
            Err(e) => error!("Error: {e}"),
        }
    }
}
