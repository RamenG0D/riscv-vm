use crate::{bus::{Device, VirtualDevice}, memory::{dram::Sizes, virtual_memory::MemorySize}, trap::Exception};


const PLIC_BASE: u32 = 0xc000000;
const PLIC_END: u32 = 0x208000;

const SOURCE_PRIORITY: u32 = PLIC_BASE;
const SOURCE_PRIORITY_END: u32 = PLIC_BASE + 0xfff;

const PENDING: u32 = PLIC_BASE + 0x1000;
const PENDING_END: u32 = PLIC_BASE + 0x107f;

const ENABLE: u32 = PLIC_BASE + 0x2000;
const ENABLE_END: u32 = PLIC_BASE + 0x20ff;

const THRESHOLD_AND_CLAIM: u32 = PLIC_BASE + 0x200000;
const THRESHOLD_AND_CLAIM_END: u32 = PLIC_BASE + 0x201007;

const WORD_SIZE: u32 = 0x4;
const CONTEXT_OFFSET: u32 = 0x1000;
const SOURCE_NUM: u32 = 1024;


/// The Platform Level Interrupt Controller ( PLIC. )
pub struct Plic {
    /// The interrupt priority for each interrupt source. A priority value of 0 is reserved to mean
    /// "never interrupt" and effectively disables the interrupt. Priority 1 is the lowest active
    /// priority, and priority 7 is the highest.
    priority: [u32; SOURCE_NUM as usize],
    /// Interrupt pending bits. If bit 1 is set, a global interrupt 1 is pending. A pending bit in
    /// the PLIC core can be cleared by setting the associated enable bit then performing a claim.
    pending: [u32; 32],
    /// Interrupt Enable Bit of Interrupt Source #0 to #1023 for 2 contexts.
    enable: [u32; 64],
    /// The settings of a interrupt priority threshold of each context. The PLIC will mask all PLIC
    /// interrupts of a priority less than or equal to `threshold`.
    threshold: [u32; 2],
    /// The ID of the highest priority pending interrupt or zero if there is no pending interrupt
    /// for each context.
    claim: [u32; 2],
}

impl Plic {
    // Create a new Plic device
    pub fn new() -> Self {
        Self {
            priority: [0; SOURCE_NUM as usize],
            pending: [0; 32],
            enable: [0; 64],
            threshold: [0; 2],
            claim: [0; 2],
        }
    }

    pub fn new_device() -> VirtualDevice {
        VirtualDevice::new(Box::new(Self::new()), PLIC_BASE, PLIC_END)
    }

       /// Sets IRQ bit in `pending`.
       pub fn update_pending(&mut self, irq: u32) {
        let index = irq.wrapping_div(WORD_SIZE);
        self.pending[index as usize] = self.pending[index as usize] | (1 << irq);

        self.update_claim(irq);
    }

    /// Clears IRQ bit in `pending`.
    fn clear_pending(&mut self, irq: u32) {
        let index = irq.wrapping_div(WORD_SIZE);
        self.pending[index as usize] = self.pending[index as usize] & !(1 << irq);

        self.update_claim(0);
    }

    /// Sets IRQ bit in `claim` for context 1.
    fn update_claim(&mut self, irq: u32) {
        // TODO: Support highest priority to the `claim` register.
        // claim[1] is claim/complete registers for S-mode (context 1). SCLAIM.
        if self.is_enable(1, irq) || irq == 0 {
            self.claim[1] = irq as u32;
        }
    }

    /// Returns true if the enable bit for the `irq` of the `context` is set.
    fn is_enable(&self, context: u32, irq: u32) -> bool {
        let index = (irq.wrapping_rem(SOURCE_NUM)).wrapping_div(WORD_SIZE * 8);
        let offset = (irq.wrapping_rem(SOURCE_NUM)).wrapping_rem(WORD_SIZE * 8);
        return ((self.enable[(context * 32 + index) as usize] >> offset) & 1) == 1;
    }

    /// Load `size`-bit data from a register located at `addr` in PLIC.
    pub fn read(&self, addr: u32, size: Sizes) -> Result<u32, Exception> {
        // TODO: should support byte-base access.
        if !matches!(size, Sizes::Word) {
            return Err(Exception::LoadAccessFault);
        }

        match addr {
            SOURCE_PRIORITY..=SOURCE_PRIORITY_END => {
                if (addr - SOURCE_PRIORITY).wrapping_rem(WORD_SIZE) != 0 {
                    return Err(Exception::LoadAccessFault);
                }
                let index = (addr - SOURCE_PRIORITY).wrapping_div(WORD_SIZE);
                Ok(self.priority[index as usize] as u32)
            }
            PENDING..=PENDING_END => {
                if (addr - PENDING).wrapping_rem(WORD_SIZE) != 0 {
                    return Err(Exception::LoadAccessFault);
                }
                let index = (addr - PENDING).wrapping_div(WORD_SIZE);
                Ok(self.pending[index as usize] as u32)
            }
            ENABLE..=ENABLE_END => {
                if (addr - ENABLE).wrapping_rem(WORD_SIZE) != 0 {
                    return Err(Exception::LoadAccessFault);
                }
                let index = (addr - ENABLE).wrapping_div(WORD_SIZE);
                Ok(self.enable[index as usize] as u32)
            }
            THRESHOLD_AND_CLAIM..=THRESHOLD_AND_CLAIM_END => {
                let context = (addr - THRESHOLD_AND_CLAIM).wrapping_div(CONTEXT_OFFSET);
                let offset = addr - (THRESHOLD_AND_CLAIM + CONTEXT_OFFSET * context);
                if offset == 0 {
                    Ok(self.threshold[context as usize] as u32)
                } else if offset == 4 {
                    Ok(self.claim[context as usize] as u32)
                } else {
                    return Err(Exception::LoadAccessFault);
                }
            }
            _ => return Err(Exception::LoadAccessFault),
        }
    }

    /// Store `size`-bit data to a register located at `addr` in PLIC.
    pub fn write(&mut self, addr: u32, value: u32, size: Sizes) -> Result<(), Exception> {
        // TODO: should support byte-base access.
        if !matches!(size, Sizes::Word) {
            return Err(Exception::StoreAccessFault);
        }

        match addr {
            SOURCE_PRIORITY..=SOURCE_PRIORITY_END => {
                if (addr - SOURCE_PRIORITY).wrapping_rem(WORD_SIZE) != 0 {
                    return Err(Exception::StoreAccessFault);
                }
                let index = (addr - SOURCE_PRIORITY).wrapping_div(WORD_SIZE);
                self.priority[index as usize] = value as u32;
            }
            PENDING..=PENDING_END => {
                if (addr - PENDING).wrapping_rem(WORD_SIZE) != 0 {
                    return Err(Exception::StoreAccessFault);
                }
                let index = (addr - PENDING).wrapping_div(WORD_SIZE);
                self.pending[index as usize] = value as u32;
            }
            ENABLE..=ENABLE_END => {
                if (addr - ENABLE).wrapping_rem(WORD_SIZE) != 0 {
                    return Err(Exception::StoreAccessFault);
                }
                let index = (addr - ENABLE).wrapping_div(WORD_SIZE);
                self.enable[index as usize] = value as u32;
            }
            THRESHOLD_AND_CLAIM..=THRESHOLD_AND_CLAIM_END => {
                let context = (addr - THRESHOLD_AND_CLAIM).wrapping_div(CONTEXT_OFFSET);
                let offset = addr - (THRESHOLD_AND_CLAIM + CONTEXT_OFFSET * context);
                if offset == 0 {
                    self.threshold[context as usize] = value as u32;
                } else if offset == 4 {
                    //self.claim[context as usize] = value as u32;

                    // Clear pending bit.
                    self.clear_pending(value);
                } else {
                    return Err(Exception::StoreAccessFault);
                }
            }
            _ => return Err(Exception::StoreAccessFault),
        }

        Ok(())
    }
}

impl Device for Plic {
    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        self.read(addr, size)
    }
    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception> {
        self.write(addr, value, size)
    }
}


#[test]
fn test_plic() {
    todo!()
}

