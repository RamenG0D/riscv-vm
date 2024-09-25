use log::error;

use crate::{
    memory::{
        dram::{Dram, Sizes, DRAM_BASE},
        virtual_memory::MemorySize,
    },
    trap::Exception,
};

pub trait Device {
    fn base(&self) -> MemorySize;
    fn size(&self) -> MemorySize;

    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception>;
    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception>;
}

pub struct Bus {
    devices: Vec<Box<dyn Device>>,
    dram: Dram,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            dram: Dram::new(),
        }
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
        self.devices.push(device);
    }

    pub fn read(&self, address: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        for device in &self.devices {
            if device.base() <= address && address < device.base() + device.size() {
                let address = address - device.base();
                return device.load(address, size);
            }
        }
        if DRAM_BASE <= address {
            let address = address - DRAM_BASE;
            return self.dram.read(address, size);
        }
        error!("Bus Read LoadAccessFault: {:#X}", address);
        Err(Exception::LoadAccessFault)
    }

    pub fn write(
        &mut self,
        address: MemorySize,
        value: MemorySize,
        size: Sizes,
    ) -> Result<(), Exception> {
        for device in &mut self.devices {
            if device.base() <= address && address < device.base() + device.size() {
                let address = address - device.base();
                return device.store(address, size, value);
            }
        }
        if DRAM_BASE <= address {
            let address = address - DRAM_BASE;
            return self.dram.write(address, value, size);
        }
        error!("Bus Write StoreAMOAccessFault: {:#X}", address);
        Err(Exception::StoreAMOAccessFault)
    }
}
