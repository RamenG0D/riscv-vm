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
		let memory = if data.len() < L {
			let mut tmp = data.to_vec();
			tmp.resize(L, 0);
			tmp.into_boxed_slice()
		} else if data.len() == L {
			data.to_vec().into_boxed_slice()
		} else {
			return Err("Data is larger than memory size".to_string());
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

	pub fn memory(&self) -> &Box<[u8]> {
		&self.memory
	}
	pub fn memory_mut(&mut self) -> &mut Box<[u8]> {
		&mut self.memory
	}

    pub fn read32(&self, index: MemorySize) -> Result<MemorySize, Exception> {
        if index + 3 > L as MemorySize {
            return Err(Exception::LoadAccessFault);
        }
        Ok(self.memory[index as usize] as MemorySize
            | ((self.memory[(index as usize) + 1] as MemorySize) << 8)
            | ((self.memory[(index as usize) + 2] as MemorySize) << 16)
            | ((self.memory[(index as usize) + 3] as MemorySize) << 24))
    }

    pub fn read16(&self, index: MemorySize) -> Result<u16, Exception> {
        if index + 1 > L as MemorySize {
            return Err(Exception::LoadAccessFault);
        }
        Ok((self.memory[index as usize] as MemorySize
            | ((self.memory[(index as usize) + 1] as MemorySize) << 8)) as u16)
    }

    pub fn read8(&self, index: MemorySize) -> Result<u8, Exception> {
        if index > L as MemorySize {
            return Err(Exception::LoadAccessFault);
        }
        Ok(self.memory[index as usize])
    }

    pub fn set32(&mut self, index: MemorySize, value: MemorySize) -> Result<(), Exception> {
        if index + 3 > L as MemorySize {
            return Err(Exception::StoreAccessFault);
        }
        self.memory[index as usize] = (value & 0xFF) as u8;
        self.memory[(index as usize) + 1] = ((value >> 8) & 0xFF) as u8;
        self.memory[(index as usize) + 2] = ((value >> 16) & 0xFF) as u8;
        self.memory[(index as usize) + 3] = ((value >> 24) & 0xFF) as u8;
        Ok(())
    }

    pub fn set16(&mut self, index: MemorySize, value: MemorySize) -> Result<(), Exception> {
        if index + 1 > L as MemorySize {
            return Err(Exception::StoreAccessFault);
        }
        self.memory[index as usize] = (value & 0xFF) as u8;
        self.memory[(index as usize) + 1] = ((value >> 8) & 0xFF) as u8;
        Ok(())
    }

    pub fn set8(&mut self, index: MemorySize, value: MemorySize) -> Result<(), Exception> {
        if index > L as MemorySize {
            return Err(Exception::StoreAccessFault);
        }
        self.memory[index as usize] = (value & 0xFF) as u8;
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
