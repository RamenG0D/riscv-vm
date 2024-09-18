use log::error;
use riscv_vm::{cpu::Cpu, trap::Trap};

pub fn main() {
    let program = include_bytes!("../../c_test/fib.bin");

    riscv_vm::logging::init_logging();

    let mut cpu = Cpu::new();

    cpu.load_program_raw(program).expect("Failed to load program");

    while !cpu.finished() {
        match cpu.step() {
            Ok(_) => (),
            Err(e) => {
                error!("Error PC: {:#X}", cpu.get_pc());
                e.take_trap(&mut cpu);
                if e.is_fatal() {
                    error!("Fatal trap: {:#X}", e.exception_code());
                    error!("Exception: {e:#?}");
                    break;
                }
            }
        }
    }
}
