
// The Virtio Constants (Magic!)
// qemu puts UART registers here in physical memory.
const UART: u32 = 0x10000000;
const UART_IRQ: u32 = 10;

// virtio mmio interface
const VIRTIO: u32 = 0x10001000;
const VIRTIO_IRQ: u32 = 1;

// A Virtio (Virtual I/O) Driver implementation
pub struct Virtio {
    /// the ID of the Driver
    id: u32,
    /// the status of the device
    status: u32,
    /// the queue notify
    queue_notify: u32,

    // Disk / Memory for the Virtual Machine
    disk: Vec<u8>,
}

impl Virtio {
    // Create a new Virtio device
    pub fn new(disk: Vec<u8>) -> Self {
        Self {
            id: 0,
            status: 0,
            queue_notify: u32::MAX, // Todo: Find out what the default value should be
            disk
        }
    }

    /// Return true if an interrupt is pending.
    pub fn is_interrupting(&mut self) -> bool {
        if self.queue_notify != u32::MAX {
            // reset the queue_notify
            self.queue_notify = u32::MAX;
            return true;
        }
        false
    }

    // Read from the disk
    pub fn read_disk(&self, addr: u32) -> u32 {
        self.disk[addr as usize] as u32
    }

    // Write to the disk
    pub fn write_disk(&mut self, addr: u32, data: u32) {
        self.disk[addr as usize] = data as u8;
    }

    // Reset the device
    pub fn reset(&mut self) {
        todo!()
    }
}

use riscv_vm::{cpu::Cpu, devices::{clint::Clint, plic::Plic, uart::Uart}, log_info, trap::Exception};
use log::LevelFilter;

// The main function
// were going to test the Virtio implementation
fn main() {
    riscv_vm::logging::init_logging(LevelFilter::Trace);

    let mut cpu = Cpu::new();
    cpu.add_device(Uart::new_device());
    cpu.add_device(Plic::new_device());
    cpu.add_device(Clint::new_device());

    cpu.load_program_raw(include_bytes!("../../c_test/fib.bin")).unwrap();

    match cpu.run() {
        Ok(_) => (),
        Err(Exception::IllegalInstruction) => eprintln!("Illegal Instruction"),
        Err(Exception::LoadAccessFault) => eprintln!("Load Access Fault"),
        Err(e) => eprintln!("Trap / Err: {e:#?}"),
    }

    log_info!("Finished running the program!");
}

