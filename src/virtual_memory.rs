/// (type) size of each memory cell (index)
pub type MemorySize = u32;

/// used to ensure stored memory is always little endian when accessed and when modified
pub struct Memory<const LENGTH: usize> {
    array: [MemorySize; LENGTH],
}

impl<const L: usize> Memory<L> {
    pub fn new() -> Self {
        Self { array: [0; L] }
    }

    pub fn get(&self, index: MemorySize) -> Option<MemorySize> {
        if index < L as MemorySize {
            Some(self.array[index as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: MemorySize, value: MemorySize) -> Result<(), String> {
        if index < L as MemorySize {
            self.array[index as usize] = value.to_le();
            Ok(())
        } else {
            Err(format!("Index out of bounds"))
        }
    }

    pub fn len(&self) -> usize {
        L
    }

    pub fn as_slice(&self) -> &[MemorySize] {
        &self.array
    }

    pub fn as_mut_slice(&mut self) -> &mut [MemorySize] {
        &mut self.array
    }
}

pub struct HeapMemory<const LENGTH: usize> {
    memory: Box<[MemorySize]>,
}

impl<const L: usize> HeapMemory<L> {
    pub fn new() -> Self {
        Self {
            // the memory is on the heap but never changes size
            memory: vec![0; L].into_boxed_slice(),
        }
    }

    pub fn get(&self, index: MemorySize) -> Option<MemorySize> {
        if index < L as MemorySize {
            Some(self.memory[index as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: MemorySize, value: MemorySize) -> Result<(), String> {
        if index < L as MemorySize {
            self.memory[index as usize] = value.to_le();
            Ok(())
        } else {
            Err(format!("Index out of bounds"))
        }
    }

    pub fn len(&self) -> usize {
        L
    }

    pub fn as_slice(&self) -> &[MemorySize] {
        &self.memory
    }

    pub fn as_mut_slice(&mut self) -> &mut [MemorySize] {
        &mut self.memory
    }
}
