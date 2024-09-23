use riscv_vm::{cpu::Cpu, trap::Trap};

pub fn main() {
    riscv_vm::logging::init_logging(std::io::stdout());

    let program = include_bytes!("../../c_test/fib.bin");
    let mut cpu = Cpu::new();
    cpu.load_program_raw(program).expect("Failed to load program");

    cpu.disassemble("disasm.txt", cpu.get_pc(), cpu.get_pc() + program.len() as u32);

    while !cpu.finished() /*&& cpu.get_pc() < riscv_vm::memory::dram::DRAM_BASE + 100*/ {
        match cpu.step() {
            Ok(_) => (),
            Err(e) => {
                log::error!("{:?}", e);
                if e.is_fatal() {
                    break;
                }
                e.take_trap(&mut cpu);
            }
        }
    }

    // riscv_vm::logging::init_logging(std::io::stdout());
    cpu.dump_registers();
}
