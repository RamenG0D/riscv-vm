use riscv_vm::{
    cpu::Cpu,
    memory::dram::DRAM_BASE,
};

pub fn main() {
    riscv_vm::logging::init_logging();

    let mut cpu = Cpu::new();

    println!("Loading program...");
    cpu.load_program_raw(include_bytes!("../../c_test/fib.bin"))
        .expect("Failed to load program");
    println!("Program LOADED");

    const PLEN: usize = include_bytes!("../../c_test/fib.bin").len();
    while !cpu.finished() || cpu.get_pc() >= (PLEN + DRAM_BASE) as u32 {
        cpu.step().unwrap();
    }

    println!("{}", cpu.to_string());
}
