use log::error;

use crate::{devices::{clint::{Clint, CLINT_BASE, CLINT_SIZE}, plic::{Plic, PLIC_BASE, PLIC_SIZE}, uart::{Uart, UART_BASE, UART_SIZE}}, memory::{
    dram::{Dram, Sizes, DRAM_BASE},
    virtual_memory::MemorySize,
}, trap::Exception};

pub trait Device {
    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception>;
    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception>;
}

pub struct Bus {
    uart: Uart,
    plic: Plic,
    clint: Clint,
    dram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            dram: Dram::new(),
            uart: Uart::new(),
            plic: Plic::new(),
            clint: Clint::new(),
        }
    }

    pub fn read(&self, address: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        if CLINT_BASE <= address && address < CLINT_BASE + CLINT_SIZE {
            let address = address - CLINT_BASE;
            return self.clint.load(address, size);
        } else if PLIC_BASE <= address && address < PLIC_BASE + PLIC_SIZE {
            let address = address - PLIC_BASE;
            return self.plic.load(address, size);
        } else if UART_BASE <= address && address < UART_BASE + UART_SIZE {
            let address = address - UART_BASE;
            return self.uart.load(address, size);
        } else if DRAM_BASE <= address {
            let address = address - DRAM_BASE;
            return self.dram.read(address, size);
        }
        error!("Bus Read LoadAccessFault: {:#X}", address);
        Err(Exception::LoadAccessFault)
    }

    pub fn write(&mut self, address: MemorySize, value: MemorySize, size: Sizes) -> Result<(), Exception> {
        if CLINT_BASE <= address && address < CLINT_BASE + CLINT_SIZE {
            let address = address - CLINT_BASE;
            return self.clint.store(address, size, value);
        } else if PLIC_BASE <= address && address < PLIC_BASE + PLIC_SIZE {
            let address = address - PLIC_BASE;
            return self.plic.store(address, size, value);
        } else if UART_BASE <= address && address < UART_BASE + UART_SIZE {
            let address = address - UART_BASE;
            return self.uart.store(address, size, value);
        } else if DRAM_BASE <= address {
            let address = address - DRAM_BASE;
            return self.dram.write(address, value, size);
        }
        error!("Bus Write StoreAMOAccessFault: {:#X}", address);
        Err(Exception::StoreAMOAccessFault)
    }
}
