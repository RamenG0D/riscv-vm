//! The virtio_blk module implements a virtio block device.
//!
//! The spec for Virtual I/O Device (VIRTIO) Version 1.1:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html
//! 5.2 Block Device:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/cs01/virtio-v1.1-cs01.html#x1-2390002

use std::io::{BufRead, Write};

use riscv_vm::{
    cpu::Cpu, devices::viritio::Virtio, log_info
};

enum UserInput {
    Yes,
    No,
    Skip,
    ContinueUntilAddr(u32),
}
fn step_input() -> Option<UserInput> {
    // println!("please provide an action: ");
    // println!("\ty/yes: to continue the program");
    // println!("\tn/no: to exit the program");
    // println!("\tskip: to skip the current step ( +4 to pc )");
    // println!("\tc/continue <address>: to continue the program until a specific address");
    // println!("\tr/reset: to reset the program");
    print!("> ");
    std::io::stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    let _ = std::io::stdin().lock().read_line(&mut buf).expect("Failed to read line");
    let buf = buf.split_ascii_whitespace().collect::<Vec<&str>>();
    if buf.is_empty() {
        return None;
    }
    let command = buf[0].trim();
    match command {
        "y" | "yes" => Some(UserInput::Yes),
        "n" | "no" => Some(UserInput::No),
        "skip" => Some(UserInput::Skip),
        "c" | "continue" => {
            if buf.len() < 2 {
                println!("Please provide an address to continue the program");
                None
            } else {
                let addr = u32::from_str_radix(buf[1], 16).expect("Failed to parse the address");
                Some(UserInput::ContinueUntilAddr(addr))
            }
        }
        "r" | "reset" => {
            // you can reset a specific value here or reset the state
            // sub commands are as follows:
            // - pc: to reset the program counter
            // - reg <register_name>: to reset a specific register
            // - all: to reset all the registers
            None
        }
        _ => None,
    }
}

// The main function
// were going to test the Virtio implementation
fn main() {
    // riscv_vm::logging::init_logging(log::LevelFilter::Trace);

    let mut cpu = Cpu::new();

    // get the file path from the args
    let args = std::env::args().collect::<Vec<String>>();
    // get the first arg
    let a0 = if args.len() >= 2 {
        &args[1]
    } else {
        eprintln!("Please provide a *.bin file to emulate and add its path as an argument to run the program");
        return;
    };

    // read the file
    log_info!("Reading the file: {a0}");
    let file = std::fs::read(a0).expect("Failed to read the file");
    cpu.initialize_dram(&file).expect("Failed to load the program");

    if args.len() >= 3 {
        let file = std::fs::read(&args[2]).expect("Failed to read the file");
        let drive = cpu.get_device_mut::<Virtio>().expect("Failed to get Virtio device");
        log_info!("Initializing the Virtio block device with the file: {}", args[2]);
        drive.initialize(file);
    }

    // debug the memory
    // cpu.dump_memory(DRAM_BASE, 0x100);

    loop {
        /*let command = step_input();
        match command {
            Some(UserInput::Yes) => {}
            Some(UserInput::No) => break,
            Some(UserInput::Skip) => {
                cpu.set_pc(cpu.get_pc().wrapping_add(4));
            }
            Some(UserInput::ContinueUntilAddr(addr)) => {
                while cpu.get_pc() != addr {
                    cpu.step().expect("Failed to step the program");
                }
            }
            None => continue,
        }*/

        match cpu.step() {
            Ok(()) => {}
            Err(_) => {
                cpu.dump_registers();
                break;
            }
        }
    }

    // match cpu.run() {
    //     Ok(_) => {}
    //     Err(e) => {
    //         log_info!("Failed to run the program: {e}");
    //         panic!("Fail PC: {:#x}", cpu.get_pc());
    //     }
    // }

    log_info!("Finished running the program!");
}
