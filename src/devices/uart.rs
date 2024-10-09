use std::{io::Write, sync::{atomic::{AtomicBool, Ordering}, Arc, Condvar, Mutex}, thread};

use crate::{bus::{Device, VirtualDevice}, memory::{dram::Sizes, virtual_memory::MemorySize}, trap::Exception};

const UART_BASE: u32 = 0x1000_0000;
const UART_END: u32 = UART_BASE + 0x100;
const UART_SIZE: u32 = 0x100;
const UART_IRQ: u32 = 10;

/// Receive holding register (for input bytes).
const UART_RHR: u32 = UART_BASE + 0;
/// Transmit holding register (for output bytes).
const UART_THR: u32 = UART_BASE + 0;
/// Interrupt enable register.
const _UART_IER: u32 = UART_BASE + 1;
/// FIFO control register.
const _UART_FCR: u32 = UART_BASE + 2;
/// Interrupt status register.
/// ISR BIT-0:
///     0 = an interrupt is pending and the ISR contents may be used as a pointer to the appropriate
/// interrupt service routine.
///     1 = no interrupt is pending.
const _UART_ISR: u32 = UART_BASE + 2;
/// Line control register.
const _UART_LCR: u32 = UART_BASE + 3;
/// Line status register.
/// LSR BIT 0:
///     0 = no data in receive holding register or FIFO.
///     1 = data has been receive and saved in the receive holding register or FIFO.
/// LSR BIT 5:
///     0 = transmit holding register is full. 16550 will not accept any data for transmission.
///     1 = transmitter hold register (or FIFO) is empty. CPU can load the next character.
const UART_LSR: u32 = UART_BASE + 5;

/// The receiver (RX).
const UART_LSR_RX: u8 = 1;
/// The transmitter (TX).
const UART_LSR_TX: u8 = 1 << 5;

pub struct Uart {
    mem: Arc<(Mutex<[u8; UART_SIZE as usize]>, Condvar)>,
    interrupting: Arc<AtomicBool>,
}

impl Uart {
    pub fn new() -> Self {
        let uart = Arc::new((Mutex::new([0; UART_SIZE as usize]), Condvar::new()));
        let interrupting = Arc::new(AtomicBool::new(false));
        {
            let (uart, _cvar) = &*uart;
            let mut uart = uart.lock().expect("failed to get an UART object");
            // Transmitter hold register is empty. It allows input anytime.
            uart[(UART_LSR - UART_BASE) as usize] |= UART_LSR_TX;
        }

        // Create a new thread for waiting for input.
        let mut byte = [0; 1];
        let cloned_uart = uart.clone();
        let cloned_interrupting = interrupting.clone();
        let _uart_thread_for_read = thread::spawn(move || loop {
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
                    cloned_interrupting.store(true, Ordering::Release);
                    // Data has been receive.
                    uart[(UART_LSR - UART_BASE) as usize] |= UART_LSR_RX;
                }
                Err(e) => {
                    println!("input via UART is error: {}", e);
                }
            }
        });

        Self { mem: uart, interrupting }
    }

    pub fn new_device() -> VirtualDevice {
        VirtualDevice::new(Box::new(Self::new()), UART_BASE, UART_SIZE)
    }
}

impl Device for Uart {
    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        if !matches!(size, Sizes::Byte) || (addr < UART_BASE || addr >= UART_END) {
            return Err(Exception::LoadAccessFault);
        }

        let (mem, lock) = &*self.mem;

        match addr + UART_BASE {
            UART_RHR => {
                lock.notify_one();
                let mut mem = mem.lock().expect("Failed to get mutex lock");
                mem[(UART_LSR - UART_BASE) as usize] &= !UART_LSR_RX;
                Ok(mem[(UART_LSR - UART_BASE) as usize] as u32)
            },
            _ => {
                lock.notify_one();
                let mem = mem.lock().expect("Failed to get mutex lock");
                Ok(mem[addr as usize] as u32)
            }
        }
    }
    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception> {
        if !matches!(size, Sizes::Byte) {
            return Err(Exception::StoreAccessFault);
        }

        let (mem, _) = &*self.mem;
        let mut mem = mem.lock().expect("Failed to get mutex lock");

        match addr + UART_BASE {
            UART_THR => {
                print!("{}", value as u8 as char);
                std::io::stdout().flush().expect("failed to flush standard i/o");
            }
            _ => {
                mem[addr as usize] = value as u8;
            }
        }

        Ok(())
    }
}
