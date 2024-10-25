use std::ops::{Index, IndexMut};

pub const REGISTERS_COUNT: usize = 32;

pub type XRegisterSize = u32;
pub type FRegisterSize = f32;

pub struct XRegisters {
    regs: [XRegisterSize; REGISTERS_COUNT],
}

impl Index<usize> for XRegisters {
    type Output = XRegisterSize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.regs[index]
    }
}

impl IndexMut<usize> for XRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.regs[index]
    }
}

impl XRegisters {
    pub fn new() -> Self {
        Self {
            regs: [0; REGISTERS_COUNT],
        }
    }

    pub fn get(&self, index: usize) -> XRegisterSize {
        self.regs[index]
    }

    pub fn set(&mut self, index: usize, value: XRegisterSize) {
        self.regs[index] = value;
    }
}

impl Default for XRegisters {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FRegisters {
    regs: [FRegisterSize; REGISTERS_COUNT],
}

impl Index<usize> for FRegisters {
    type Output = FRegisterSize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.regs[index]
    }
}

impl IndexMut<usize> for FRegisters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.regs[index]
    }
}

impl FRegisters {
    pub fn new() -> Self {
        Self {
            regs: [0.0; REGISTERS_COUNT],
        }
    }

    pub fn get(&self, index: usize) -> FRegisterSize {
        self.regs[index]
    }

    pub fn set(&mut self, index: usize, value: FRegisterSize) {
        self.regs[index] = value;
    }
}

impl Default for FRegisters {
    fn default() -> Self {
        Self::new()
    }
}
