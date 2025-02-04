use crate::{bus::{Device, VirtualDevice}, trap::Exception};

use super::virtual_memory::{HeapMemory, MemorySize};

pub const DRAM_SIZE: MemorySize = 128 * 1024 * 1024; // 1 GiB
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
        Self { memory: HeapMemory::new() }
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

impl Device for Dram {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        self.read(addr, size)
    }

    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception> {
        self.write(addr, value, size)
    }
}

impl Default for Dram {
    fn default() -> Self {
        Self::new()
    }
}
