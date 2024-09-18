use log::error;

use crate::{memory::{
    dram::{Dram, Sizes, DRAM_BASE, DRAM_SIZE},
    virtual_memory::MemorySize,
}, trap::Exception};

pub trait Device {
    fn load(&self, addr: MemorySize, size: MemorySize) -> Option<MemorySize>;
    fn store(&mut self, addr: MemorySize, size: MemorySize, value: MemorySize) -> Option<()>;
}

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Self { dram: Dram::new() }
    }

    pub fn read(&self, address: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        if (address as usize) < (DRAM_BASE + DRAM_SIZE) {
            let addr = address.checked_sub(DRAM_BASE as MemorySize).ok_or_else(|| {
                error!("Failed to Subtract DRAM_BASE from address: {:#X}", address);
                error!("LoadAccessFault: {:#X}", address);
                Exception::LoadAccessFault
            })? as usize;
            self.dram.read(addr, size)
        } else {
            error!("LoadAccessFault: {:#X}", address);
            Err(Exception::LoadAccessFault)
        }
    }

    pub fn write(&mut self, address: MemorySize, value: MemorySize, size: Sizes) -> Result<(), Exception> {
        if (address as usize) < (DRAM_BASE + DRAM_SIZE) {
            let addr = address.checked_sub(DRAM_BASE as MemorySize).ok_or_else(|| {
                error!("Failed to Subtract DRAM_BASE from address: {:#X}", address);
                error!("StoreAMOAccessFault: {:#X}", address);
                Exception::StoreAMOAccessFault
            })? as usize;
            self.dram.write(addr, value, size)
        } else {
            error!("StoreAMOAccessFault: {:#X}", address);
            Err(Exception::StoreAMOAccessFault)
        }
    }
}
