use anyhow::Result;

use crate::bus::{Device, VirtualDevice};
use super::virtual_memory::HeapMemory;

pub const DRAM_SIZE: u64 = 128 * 1024 * 1024; // 1 GiB
pub const DRAM_BASE: u64 = 0x8000_0000;

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

    pub fn new_device() -> VirtualDevice {
        VirtualDevice::new(Box::new(Self::new()), DRAM_BASE, DRAM_SIZE)
    }

    pub fn initialize(&mut self, data: &[u8]) {
        let mem = self.memory.memory_mut();
        if data.len() > mem.len() {
            panic!("Error: data is too large for DRAM");
        }
        mem[..data.len()].copy_from_slice(data);
    }

    pub fn read(&self, address: u64, size: Sizes) -> Result<u32> {
        match size {
            Sizes::Byte => self.memory.read8(address),
            Sizes::HalfWord => self.memory.read16(address),
            Sizes::Word => self.memory.read32(address),
        }
    }

    pub fn write(&mut self, address: u64, value: u32, size: Sizes) -> Result<()> {
        match size {
            Sizes::Byte => self.memory.write8(address, value),
            Sizes::HalfWord => self.memory.write16(address, value),
            Sizes::Word => self.memory.write32(address, value),
        }
    }
}

impl Device for Dram {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn load(&self, addr: u64, size: Sizes) -> Result<u32> {
        self.read(addr, size)
    }

    fn store(&mut self, addr: u64, size: Sizes, value: u32) -> Result<()> {
        self.write(addr, value, size)
    }
}

impl Default for Dram {
    fn default() -> Self {
        Self::new()
    }
}
