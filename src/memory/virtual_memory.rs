/// (type) size of each memory cell (index)
pub type MemorySize = u32;

/// used to ensure stored memory is always little endian when accessed and when modified
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
        let index = index as usize;
        let value = u32::from_le_bytes([
            self.array[index],
            self.array[index + 1],
            self.array[index + 2],
            self.array[index + 3],
        ]);
        Some(value)
    }

    pub fn read16(&self, index: MemorySize) -> Option<u16> {
        if index + 1 > L as MemorySize {
            return None;
        }
        let index = index as usize;
        let value = u16::from_le_bytes([self.array[index], self.array[index + 1]]);
        Some(value)
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
        let index = index as usize;
        self.array[index..index + 4].copy_from_slice(&value.to_le_bytes());
        Some(())
    }

    pub fn set16(&mut self, index: MemorySize, value: MemorySize) -> Option<()> {
        if index + 1 > L as MemorySize {
            return None;
        }
        let index = index as usize;
        self.array[index..index + 2].copy_from_slice(&value.to_le_bytes());
        Some(())
    }

    pub fn set8(&mut self, index: MemorySize, value: MemorySize) -> Option<()> {
        if index > L as MemorySize {
            return None;
        }
        self.array[index as usize] = (value as u8).to_le();
        Some(())
    }

    pub fn len(&self) -> usize {
        L
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

    pub fn read32(&self, index: usize) -> MemorySize {
        let value = u32::from_le_bytes([
            self.memory[index],
            self.memory[index + 1],
            self.memory[index + 2],
            self.memory[index + 3],
        ]);
        value
    }

    pub fn read16(&self, index: usize) -> u16 {
        let value = u16::from_le_bytes([self.memory[index], self.memory[index + 1]]);
        value
    }

    pub fn read8(&self, index: usize) -> u8 {
        self.memory[index]
    }

    pub fn set32(&mut self, index: usize, value: MemorySize) {
        self.memory[index..index + 4].copy_from_slice(&value.to_le_bytes());
    }

    pub fn set16(&mut self, index: usize, value: MemorySize) {
        self.memory[index..index + 2].copy_from_slice(&value.to_le_bytes());
    }

    pub fn set8(&mut self, index: usize, value: MemorySize) {
        self.memory[index] = (value as u8).to_le();
    }

    pub fn len(&self) -> usize {
        L
    }
}
