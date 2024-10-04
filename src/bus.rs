use crate::{
    log_error,
    memory::{
        dram::{Dram, Sizes, DRAM_BASE, DRAM_SIZE},
        virtual_memory::MemorySize,
    },
    trap::Exception,
};

pub trait Device {
    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception>;
    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception>;
}

pub struct VirtualDevice {
    inner_device: Box<dyn Device>,
    base: MemorySize,
    size: MemorySize,
}

impl VirtualDevice {
    pub fn new(inner_device: Box<dyn Device>, base: MemorySize, size: MemorySize) -> Self {
        Self {
            inner_device,
            base,
            size,
        }
    }

    pub fn base(&self) -> MemorySize {
        self.base
    }

    pub fn size(&self) -> MemorySize {
        self.size
    }

    pub fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        self.inner_device.load(addr, size)
    }

    pub fn store(
        &mut self,
        addr: MemorySize,
        size: Sizes,
        value: MemorySize,
    ) -> Result<(), Exception> {
        self.inner_device.store(addr, size, value)
    }
}

pub struct Bus {
    devices: Vec<VirtualDevice>,
}

impl Bus {
    pub fn new() -> Self {
        let dram = VirtualDevice::new(Box::new(Dram::new()), DRAM_BASE, DRAM_SIZE);
        Self {
            devices: vec![dram],
        }
    }

    pub fn add_device(&mut self, device: VirtualDevice) {
        self.devices.push(device);
    }

    pub fn read(&self, address: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        for device in &self.devices {
            if device.base() <= address && address < device.base() + device.size() {
                let address = address - device.base(); // local address
                return device.load(address, size);
            }
        }
        log_error!("Bus Read LoadAccessFault: {:#X}", address);
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
        log_error!("Bus Write StoreAMOAccessFault: {:#X}", address);
        Err(Exception::StoreAccessFault)
    }
}
