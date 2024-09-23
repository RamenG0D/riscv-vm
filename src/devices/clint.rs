use crate::{bus::Device, memory::{dram::Sizes, virtual_memory::MemorySize}, trap::Exception};

/// The address which the core-local interruptor (CLINT) starts. It contains the timer and
/// generates per-hart software interrupts and timer
/// interrupts.
pub const CLINT_BASE: MemorySize = 0x200_0000;
/// The size of CLINT.
pub const CLINT_SIZE: MemorySize = 0x10000;

/// The address of a mtimecmp register starts. A mtimecmp is a dram mapped machine mode timer
/// compare register, used to trigger an interrupt when mtimecmp is greater than or equal to mtime.
pub const CLINT_MTIMECMP: MemorySize = CLINT_BASE + 0x4000;
/// The address of a timer register. A mtime is a machine mode timer register which runs at a
/// constant frequency.
pub const CLINT_MTIME: MemorySize = CLINT_BASE + 0xbff8;

/// The core-local interruptor (CLINT).
pub struct Clint {
    mtime: MemorySize,
    mtimecmp: MemorySize,
}

impl Device for Clint {
    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        match size {
            Sizes::Word => Ok(self.load32(addr)),
            _ => Err(Exception::LoadAccessFault),
        }
    }

    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception> {
        match size {
            Sizes::Word => Ok(self.store32(addr, value)),
            _ => Err(Exception::StoreAMOAccessFault),
        }
    }
}

impl Clint {
    /// Create a new `Clint` object.
    pub fn new() -> Self {
        Self {
            mtime: 0,
            mtimecmp: 0,
        }
    }

    fn load32(&self, addr: MemorySize) -> MemorySize {
        match addr {
            CLINT_MTIMECMP => self.mtimecmp,
            CLINT_MTIME => self.mtime,
            _ => 0,
        }
    }

    fn store32(&mut self, addr: MemorySize, value: MemorySize) {
        match addr {
            CLINT_MTIMECMP => self.mtimecmp = value,
            CLINT_MTIME => self.mtime = value,
            _ => ()
        }
    }
}
