use crate::{
    devices::{clint::Clint, plic::Plic, uart::Uart, viritio::Virtio}, log_error, memory::{
        dram::{Dram, Sizes},
        virtual_memory::MemorySize,
    }, rom::Rom, trap::Exception
};

pub trait Device {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

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

        Self {
            devices: vec![
                Dram::new_device(),
                Rom::new_device(),
                Uart::new_device(),
                Plic::new_device(),
                Clint::new_device(),
                Virtio::new_device(),
            ],
        }
    }

    pub fn get_device<T>(&self) -> Option<&T>
    where
        T: Device + 'static,
    {
        for device in &self.devices {
            if let Some(device) = device.inner_device.as_any().downcast_ref::<T>() {
                return Some(device);
            }
        }
        None
    }

    pub fn get_device_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Device + 'static,
    {
        for device in &mut self.devices {
            if let Some(device) = device.inner_device.as_any_mut().downcast_mut::<T>() {
                return Some(device);
            }
        }
        None
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

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}
