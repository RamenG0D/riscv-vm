use riscv_vm::{
    cpu::{Cpu, RegisterSize},
    memory::dram::{DRAM_BASE, DRAM_SIZE},
};

pub fn main() {
    riscv_vm::logging::init_logging();

    let mut cpu = Cpu::new();

    println!("Loading program...");
    cpu.load_program_raw(include_bytes!("../../linux_kernel/kernel"))
        .expect("Failed to load program");
    println!("Program LOADED");

    const PLEN: usize = include_bytes!("../../linux_kernel/kernel").len();
    while cpu.get_pc() < (DRAM_BASE + DRAM_SIZE) as RegisterSize {
        match cpu.execute() {
            Ok(_) => (),
            Err(e) => eprintln!("Error: {e}"),
        }
        if cpu.get_pc() >= DRAM_BASE as RegisterSize + PLEN as RegisterSize{
            break;
        }
        cpu.set_pc(cpu.get_pc() + 4);

        // println!("PC: {:#x}", cpu.get_pc());
    }

    println!("{}", cpu.to_string());
}
