use crate::{
    bus::{Device, VirtualDevice},
    csr::{State, MIP, MSIP_BIT, MTIP_BIT},
    memory::{dram::Sizes, virtual_memory::MemorySize},
    trap::Exception,
};

const CLINT_BASE: u32 = 0x2000000;
const CLINT_END: u32 = CLINT_BASE + 0x10000;

/// The address that a msip register starts. A msip is a machine mode software interrupt pending
/// register, used to assert a software interrupt for a CPU.
const MSIP: u32 = CLINT_BASE;
/// The address that a msip register ends. `msip` is a 4-byte register.
const MSIP_END: u32 = MSIP + 0x4;

/// The address that a mtimecmp register starts. A mtimecmp is a memory mapped machine mode timer
/// compare register, used to trigger an interrupt when mtimecmp is greater than or equal to mtime.
const MTIMECMP: u32 = CLINT_BASE + 0x4000;
/// The address that a mtimecmp register ends. `mtimecmp` is a 8-byte register.
const MTIMECMP_END: u32 = MTIMECMP + 0x8;

/// The address that a timer register starts. A mtime is a machine mode timer register which runs
/// at a constant frequency.
const MTIME: u32 = CLINT_BASE + 0xbff8;
/// The address that a timer register ends. `mtime` is a 8-byte register.
const MTIME_END: u32 = MTIME + 0x8;

pub struct Clint {
    mtime: u32,
    mtimecmp: u32,
    msip: u32,
}

impl Device for Clint {
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

impl Clint {
    pub fn new() -> Self {
        Self {
            mtime: 0,
            mtimecmp: 0,
            msip: 0,
        }
    }

    pub fn new_device() -> VirtualDevice {
        VirtualDevice::new(Box::new(Self::new()), CLINT_BASE, CLINT_END)
    }

    /// Increment the mtimer register. It's not a real-time value. The MTIP bit (MIP, 7) is enabled
    /// when `mtime` is greater than or equal to `mtimecmp`.
    pub fn increment(&mut self, state: &mut State) {
        self.mtime = self.mtime.wrapping_add(1);
        // Sync TIME csr.
        // state.write(TIME, self.mtime);

        if (self.msip & 1) != 0 {
            // Enable the MSIP bit (MIP, 3).
            state.write(MIP, state.read(MIP) | MSIP_BIT);
        }

        // 3.1.10 Machine Timer Registers (mtime and mtimecmp)
        // "The interrupt remains posted until mtimecmp becomes greater than mtime (typically as a
        // result of writing mtimecmp)."
        if self.mtimecmp > self.mtime {
            // Clear the MTIP bit (MIP, 7).
            state.write(MIP, state.read(MIP) & !MTIP_BIT);
        }

        // 3.1.10 Machine Timer Registers (mtime and mtimecmp)
        // "A timer interrupt becomes pending whenever mtime contains a value greater than or equal
        // to mtimecmp, treating the values as unsigned integers."
        if self.mtime >= self.mtimecmp {
            // Enable the MTIP bit (MIP, 7).
            state.write(MIP, state.read(MIP) | MTIP_BIT);
        }
    }

    pub fn read(&self, addr: u32, size: Sizes) -> Result<u32, Exception> {
        let (reg, offset) = match addr + CLINT_BASE {
            addr @ MSIP..=MSIP_END => (self.msip, addr - MSIP),
            addr @ MTIMECMP..=MTIMECMP_END => (self.mtimecmp, addr - MTIMECMP),
            addr @ MTIME..=MTIME_END => (self.mtime, addr - MTIME),
            _ => return Err(Exception::LoadAccessFault),
        };

        match size {
            Sizes::Byte => Ok((reg >> (offset * 8)) & 0xff),
            Sizes::HalfWord => Ok((reg >> (offset * 8)) & 0xffff),
            Sizes::Word => Ok(reg),
        }
    }

    pub fn write(&mut self, addr: u32, data: u32, size: Sizes) -> Result<(), Exception> {
        let (reg, offset) = match addr + CLINT_BASE {
            addr @ MSIP..=MSIP_END => (&mut self.msip, addr - MSIP),
            addr @ MTIMECMP..=MTIMECMP_END => (&mut self.mtimecmp, addr - MTIMECMP),
            addr @ MTIME..=MTIME_END => (&mut self.mtime, addr - MTIME),
            _ => return Err(Exception::StoreAccessFault),
        };

        match size {
            Sizes::Byte => *reg = (*reg & !(0xff << (offset * 8))) | (data << (offset * 8)),
            Sizes::HalfWord => *reg = (*reg & !(0xffff << (offset * 8))) | (data << (offset * 8)),
            Sizes::Word => *reg = data,
        }

        Ok(())
    }
}

impl Default for Clint {
    fn default() -> Self {
        Self::new()
    }
}
