use crate::memory::{
    dram::{Dram, Sizes, DRAM_BASE, DRAM_SIZE},
    virtual_memory::MemorySize,
};

pub trait Device {
    fn load(&self, addr: u64, size: u64) -> Option<u64>;
    fn store(&mut self, addr: u64, size: u64, value: u64) -> Option<()>;
}

pub struct Bus {
    dram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Self { dram: Dram::new() }
    }

    pub fn read(&self, address: MemorySize, size: Sizes) -> Option<MemorySize> {
        if (address as usize) < (DRAM_BASE + DRAM_SIZE) {
            self.dram.read((address - DRAM_BASE as MemorySize) as usize, size)
        } else {
            None
        }
    }

    pub fn write(&mut self, address: MemorySize, value: MemorySize, size: Sizes) -> Option<()> {
        if (address as usize) < (DRAM_BASE + DRAM_SIZE) {
            self.dram.write((address - DRAM_BASE as MemorySize) as usize, value, size)
        } else {
            None
        }
    }
}
