use std::cmp::Ordering;

use anyhow::{Context, Result};

use crate::trap::Exception;

/// (type) size of each memory cell (index)
pub type MemorySize = u32;

pub struct Memory<const LENGTH: usize> {
    array: [u8; LENGTH],
}

impl<const L: usize> Memory<L> {
    pub fn new() -> Self {
        Self { array: [0; L] }
    }

    pub fn read32(&self, index: MemorySize) -> Option<MemorySize> {
        if index + 3 > L as MemorySize {
            return None;
        }
        Some(
            self.array[index as usize] as MemorySize
                | ((self.array[(index as usize) + 1] as MemorySize) << 8)
                | ((self.array[(index as usize) + 2] as MemorySize) << 16)
                | ((self.array[(index as usize) + 3] as MemorySize) << 24),
        )
    }

    pub fn read16(&self, index: MemorySize) -> Option<u16> {
        if index + 1 > L as MemorySize {
            return None;
        }
        Some(
            (self.array[index as usize] as MemorySize
                | ((self.array[(index as usize) + 1] as MemorySize) << 8)) as u16,
        )
    }

    pub fn read8(&self, index: MemorySize) -> Option<u8> {
        if index > L as MemorySize {
            return None;
        }
        Some(self.array[index as usize])
    }

    pub fn set32(&mut self, index: MemorySize, value: MemorySize) -> Option<()> {
        if index + 3 > L as MemorySize {
            return None;
        }
        let bytes = value.to_le_bytes();
        self.array[index as usize] = bytes[0];
        self.array[(index as usize) + 1] = bytes[1];
        self.array[(index as usize) + 2] = bytes[2];
        self.array[(index as usize) + 3] = bytes[3];
        Some(())
    }

    pub fn set16(&mut self, index: MemorySize, value: MemorySize) -> Option<()> {
        if index + 1 > L as MemorySize {
            return None;
        }
        let bytes = value.to_le_bytes();
        self.array[index as usize] = bytes[0];
        self.array[(index as usize) + 1] = bytes[1];
        Some(())
    }

    pub fn set8(&mut self, index: MemorySize, value: MemorySize) -> Option<()> {
        if index > L as MemorySize {
            return None;
        }
        self.array[index as usize] = value as u8;
        Some(())
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        L
    }
}

impl<const L: usize> Default for Memory<L> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HeapMemory<const LENGTH: usize> {
    memory: Box<[u8]>,
}

impl<const L: usize> HeapMemory<L> {
    pub fn new() -> Self {
        Self {
            // the memory is on the heap but never changes size
            memory: vec![0; L].into_boxed_slice(),
        }
    }

    pub fn with_data(data: &[u8]) -> Result<Self, String> {
        // check if data is larger than memory size
        let memory = match data.len().cmp(&L) {
            Ordering::Less => {
                let mut tmp = data.to_vec();
                tmp.resize(L, 0);
                tmp.into_boxed_slice()
            }
            Ordering::Equal => data.to_vec().into_boxed_slice(),
            Ordering::Greater => {
                return Err("Data is larger than memory size".to_string());
            }
        };
        Ok(Self { memory })
    }

    pub fn resize<const NL: usize>(self) -> HeapMemory<NL> {
        let memory = if self.memory.len() < NL {
            let mut tmp = Vec::from(self.memory);
            tmp.resize(NL, 0);
            tmp.into_boxed_slice()
        } else {
            self.memory
        };
        HeapMemory { memory }
    }

    pub fn memory(&self) -> &[u8] {
        &self.memory
    }
    pub fn memory_mut(&mut self) -> &mut Box<[u8]> {
        &mut self.memory
    }

    pub fn read32(&self, index: u64) -> Result<MemorySize> {
        if index + 3 > L as u64 {
            return Err(Exception::LoadAccessFault).context(format!("index: {index}, length: {L}"));
        }
        let index = index as usize;

        Ok(i32::from_le_bytes([
            self.memory[index],
            self.memory[index + 1],
            self.memory[index + 2],
            self.memory[index + 3],
        ]) as u32)
    }

    pub fn read16(&self, index: u64) -> Result<u32> {
        if index + 1 > L as u64 {
            return Err(Exception::LoadAccessFault).context(format!("index: {index}, length: {L}"));
        }
        let index = index as usize;

        Ok(i16::from_le_bytes([self.memory[index], self.memory[index + 1]]) as u32)
    }

    pub fn read8(&self, index: u64) -> Result<u32> {
        if index > L as u64 {
            return Err(Exception::LoadAccessFault).context(format!("index: {index}, length: {L}"));
        }

        Ok(self.memory[index as usize] as i8 as u32)
    }

    pub fn write32(&mut self, index: u64, value: MemorySize) -> Result<()> {
        if index + 3 > L as u64 {
            return Err(Exception::StoreAccessFault)
                .context(format!("index: {index}, length: {L}"));
        }
        let index = index as usize;

        let bytes = value.to_le_bytes();
        self.memory[index] = bytes[0];
        self.memory[index + 1] = bytes[1];
        self.memory[index + 2] = bytes[2];
        self.memory[index + 3] = bytes[3];

        Ok(())
    }

    pub fn write16(&mut self, index: u64, value: MemorySize) -> Result<()> {
        if index + 1 > L as u64 {
            return Err(Exception::StoreAccessFault)
                .context(format!("index: {index}, length: {L}"));
        }
        let index = index as usize;

        let bytes = value.to_le_bytes();
        self.memory[index] = bytes[0];
        self.memory[index + 1] = bytes[1];

        Ok(())
    }

    pub fn write8(&mut self, index: u64, value: MemorySize) -> Result<()> {
        if index > L as u64 {
            return Err(Exception::StoreAccessFault)
                .context(format!("index: {index}, length: {L}"));
        }

        self.memory[index as usize] = value as u8;

        Ok(())
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        L
    }
}

impl<const L: usize> Default for HeapMemory<L> {
    fn default() -> Self {
        Self::new()
    }
}
