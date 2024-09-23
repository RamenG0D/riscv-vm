use crate::{bus::Device, memory::{dram::Sizes, virtual_memory::MemorySize}, trap::Exception};


/// The address which the platform-level interrupt controller (PLIC) starts. The PLIC connects all external interrupts in the
/// system to all hart contexts in the system, via the external interrupt source in each hart.
pub const PLIC_BASE: MemorySize = 0xc00_0000;
/// The size of PLIC.
pub const PLIC_SIZE: MemorySize = 0x4000000;

/// The address of interrupt pending bits.
pub const PLIC_PENDING: MemorySize = PLIC_BASE + 0x1000;
/// The address of the regsiters to enable interrupts for S-mode.
pub const PLIC_SENABLE: MemorySize = PLIC_BASE + 0x2080;
/// The address of the registers to set a priority for S-mode.
pub const PLIC_SPRIORITY: MemorySize = PLIC_BASE + 0x201000;
/// The address of the claim/complete registers for S-mode.
pub const PLIC_SCLAIM: MemorySize = PLIC_BASE + 0x201004;

/// The platform-level-interrupt controller (PLIC).
pub struct Plic {
    pending: MemorySize,
    senable: MemorySize,
    spriority: MemorySize,
    sclaim: MemorySize,
}

impl Device for Plic {
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

impl Plic {
    /// Create a new `Plic` object.
    pub fn new() -> Self {
        Self {
            pending: 0,
            senable: 0,
            spriority: 0,
            sclaim: 0,
        }
    }

    fn load32(&self, addr: MemorySize) -> MemorySize {
        match addr {
            PLIC_PENDING => self.pending,
            PLIC_SENABLE => self.senable,
            PLIC_SPRIORITY => self.spriority,
            PLIC_SCLAIM => self.sclaim,
            _ => 0,
        }
    }

    fn store32(&mut self, addr: MemorySize, value: MemorySize) {
        match addr {
            PLIC_PENDING => self.pending = value,
            PLIC_SENABLE => self.senable = value,
            PLIC_SPRIORITY => self.spriority = value,
            PLIC_SCLAIM => self.sclaim = value,
            _ => {}
        }
    }
}
