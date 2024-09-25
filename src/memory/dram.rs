use crate::trap::Exception;

use super::virtual_memory::{HeapMemory, MemorySize};

pub const DRAM_SIZE: MemorySize = 1024 * 1024 * 128; // 1GB
pub const DRAM_BASE: MemorySize = 0x8000_0000;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sizes {
    Byte,
    HalfWord,
    Word,
}

const DRAM_LEN: usize = DRAM_SIZE as usize;
pub struct Dram {
    memory: HeapMemory<DRAM_LEN>,
}

impl Dram {
    pub fn new() -> Self {
        Self {
            memory: HeapMemory::new(),
        }
    }

    pub fn read(&self, address: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        match size {
            Sizes::Byte => Ok(self.memory.read8(address)? as MemorySize),
            Sizes::HalfWord => Ok(self.memory.read16(address)? as MemorySize),
            Sizes::Word => Ok(self.memory.read32(address)? as MemorySize),
        }
    }

    pub fn write(
        &mut self,
        address: MemorySize,
        value: MemorySize,
        size: Sizes,
    ) -> Result<(), Exception> {
        match size {
            Sizes::Byte => self.memory.set8(address, value),
            Sizes::HalfWord => self.memory.set16(address, value),
            Sizes::Word => self.memory.set32(address, value),
        }
    }
}
