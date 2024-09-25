use log::error;
use std::sync::{atomic::AtomicBool, Arc, Condvar, Mutex};

use crate::{
    bus::Device,
    memory::{dram::Sizes, virtual_memory::MemorySize},
    trap::Exception,
};

/// The address which UART starts, same as QEMU virt machine.
pub const UART_BASE: MemorySize = 0x1000_0000;
/// The size of UART.
pub const UART_SIZE: MemorySize = 0x100;

/// Receive holding register (for input bytes).
pub const UART_RHR: MemorySize = UART_BASE + 0;
/// Transmit holding register (for output bytes).
pub const UART_THR: MemorySize = UART_BASE + 0;
/// Line control register.
pub const UART_LCR: MemorySize = UART_BASE + 3;
/// Line status register.
/// LSR BIT 0:
///     0 = no data in receive holding register or FIFO.
///     1 = data has been receive and saved in the receive holding register or FIFO.
/// LSR BIT 5:
///     0 = transmit holding register is full. 16550 will not accept any data for transmission.
///     1 = transmitter hold register (or FIFO) is empty. CPU can load the next character.
pub const UART_LSR: MemorySize = UART_BASE + 5;

/// The receiver (RX) bit.
pub const UART_LSR_RX: u8 = 1;
/// The transmitter (TX) bit.
pub const UART_LSR_TX: u8 = 1 << 5;

pub struct Uart {
    uart: Arc<(Mutex<[u8; UART_SIZE as usize]>, Condvar)>,
    interrupting: Arc<AtomicBool>,
}

impl Device for Uart {
    #[inline(always)]
    fn base(&self) -> MemorySize { UART_BASE }
    #[inline(always)]
    fn size(&self) -> MemorySize { UART_SIZE }

    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        match size {
            Sizes::Byte => Ok(self.load8(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }
    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception> {
        match size {
            Sizes::Byte => Ok(self.store8(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Uart {
    pub fn new() -> Self {
        let uart = Arc::new((Mutex::new([0; UART_SIZE as usize]), Condvar::new()));
        let interrupting = Arc::new(AtomicBool::new(false));
        {
            let (uart, _cvar) = &*uart;
            let mut uart = uart.lock().expect("failed to get an UART object");
            // Transmitter hold register is empty.
            uart[(UART_LSR - UART_BASE) as usize] |= UART_LSR_TX;
        }

        let mut byte = [0; 1];
        let cloned_uart = uart.clone();
        let cloned_interrupting = interrupting.clone();
        let _uart_thread_for_read = std::thread::spawn(move || loop {
            use std::io::Read;
            match std::io::stdin().read(&mut byte) {
                Ok(_) => {
                    let (uart, cvar) = &*cloned_uart;
                    let mut uart = uart.lock().expect("failed to get an UART object");
                    // Wait for the thread to start up.
                    while (uart[(UART_LSR - UART_BASE) as usize] & UART_LSR_RX) == 1 {
                        uart = cvar.wait(uart).expect("the mutex is poisoned");
                    }
                    uart[0] = byte[0];
                    cloned_interrupting.store(true, std::sync::atomic::Ordering::Release);
                    // Data has been receive.
                    uart[(UART_LSR - UART_BASE) as usize] |= UART_LSR_RX;
                }
                Err(e) => error!("{e}"),
            }
        });
        Self { uart, interrupting }
    }

    fn load8(&self, addr: MemorySize) -> MemorySize {
        let (uart, cvar) = &*self.uart;
        let mut uart = uart.lock().expect("failed to get an UART object");
        match addr {
            UART_RHR => {
                cvar.notify_one();
                uart[(UART_LSR - UART_BASE) as usize] &= !UART_LSR_RX;
                uart[(UART_RHR - UART_BASE) as usize] as MemorySize
            }
            _ => uart[addr as usize] as MemorySize,
        }
    }

    fn store8(&mut self, addr: MemorySize, value: MemorySize) {
        let (uart, _cvar) = &*self.uart;
        let mut uart = uart.lock().expect("failed to get an UART object");
        match addr {
            UART_THR => {
                print!("{}", value as u8 as char);
            }
            _ => {
                let addr = addr - UART_BASE;
                uart[addr as usize] = value as u8;
            }
        }
    }
}
