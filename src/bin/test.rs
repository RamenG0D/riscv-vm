//! The virtio_blk module implements a virtio block device.
//!
//! The spec for Virtual I/O Device (VIRTIO) Version 1.1:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/virtio-v1.1.html
//! 5.2 Block Device:
//! https://docs.oasis-open.org/virtio/virtio/v1.1/cs01/virtio-v1.1-cs01.html#x1-2390002

use riscv_vm::{
    cpu::Cpu, devices::viritio::Virtio, log_info
};

// The main function
// were going to test the Virtio implementation
fn main() {
    riscv_vm::logging::init_logging(log::LevelFilter::Debug);

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

    match cpu.run() {
        Ok(_) => log_info!("Program exited successfully"),
        Err(e) => log_info!("Program exited with an error: {:?}", e),
    }

    log_info!("Finished running the program!");
}
