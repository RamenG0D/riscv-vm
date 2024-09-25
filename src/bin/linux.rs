use riscv_vm::{cpu::Cpu, devices::{uart::Uart, virtio::Virtio}, trap::Trap};

pub fn main() {
    riscv_vm::logging::init_logging(std::io::stdout());

    let program = include_bytes!("../../c_test/kernel.bin");
    let mut cpu = Cpu::new();

    cpu.add_device(Box::new(Uart::new()));
    cpu.add_device(Box::new(Virtio::new()));

    cpu.load_program_raw(program)
        .expect("Failed to load program");

    // riscv_vm::disassemble(&riscv_vm::convert_memory(program), "disasm.txt");

    let cpu_step = |cpu: &mut Cpu| {
        match cpu.step() {
            Ok(_) => (),
            Err(e) => {
                log::error!("{:?}", e);
                if e.is_fatal() {
                    return true;
                }
                e.take_trap(cpu);
            }
        }
        return false;
    };
    let step_input = |cpu: &mut Cpu| {
        let mut input = String::new();
        use std::io::Write;
        input.clear();
        println!("Press 'y' to continue, 'n' to exit");
        let _ = std::io::stdout().flush();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        input = input.to_lowercase();
        match input.trim() {
            "y" => cpu_step(cpu),
            "n" => std::process::exit(0),
            _ => false,
        }
    };

    while !cpu.finished() {
        // step_input(&mut cpu);
        if cpu_step(&mut cpu) {
            break;
        }
    }

    // riscv_vm::logging::init_logging(std::io::stdout());
    cpu.dump_registers();
}
