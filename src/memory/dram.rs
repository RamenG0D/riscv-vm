use super::virtual_memory::{HeapMemory, MemorySize};

pub const DRAM_SIZE: usize = 1024 * 1024 * 128; // 1GB
pub const DRAM_BASE: usize = 0x8000_0000;

pub enum Sizes {
    Byte,
    HalfWord,
    Word,
}

pub struct Dram {
    memory: HeapMemory<DRAM_SIZE>,
}

impl Dram {
    pub fn new() -> Self {
        Self {
            memory: HeapMemory::new(),
        }
    }

    pub fn read(&self, address: usize, size: Sizes) -> Option<MemorySize> {
        match size {
            Sizes::Byte =>     Some(self.memory.read8 (address) as MemorySize),
            Sizes::HalfWord => Some(self.memory.read16(address) as MemorySize),
            Sizes::Word =>     Some(self.memory.read32(address)),
        }
    }

    pub fn write(&mut self, address: usize, value: MemorySize, size: Sizes) -> Option<()> {
        match size {
            Sizes::Byte =>     Some(self.memory.set8 (address, value as MemorySize)),
            Sizes::HalfWord => Some(self.memory.set16(address, value as MemorySize)),
            Sizes::Word =>     Some(self.memory.set32(address, value)),
        }
    }
}
