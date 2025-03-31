use anyhow::{Context, Result};

use crate::{
    memory::{dram::Sizes, virtual_memory::MemorySize},
    trap::Exception,
};

pub trait Device {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn load(&self, addr: u64, size: Sizes) -> Result<MemorySize>;
    fn store(&mut self, addr: u64, size: Sizes, value: MemorySize) -> Result<()>;

    fn increment(&mut self) {}
}

pub struct VirtualDevice {
    inner_device: Box<dyn Device>,
    base: u64,
    size: u64,
}

impl VirtualDevice {
    pub fn new(inner_device: Box<dyn Device>, base: u64, size: u64) -> Self {
        Self {
            inner_device,
            base,
            size,
        }
    }

    pub fn base(&self) -> u64 {
        self.base
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn load(&self, addr: u64, size: Sizes) -> Result<MemorySize> {
        self.inner_device.load(addr, size)
    }

    pub fn store(&mut self, addr: u64, size: Sizes, value: MemorySize) -> Result<()> {
        self.inner_device.store(addr, size, value)
    }

    pub fn increment(&mut self) {
        self.inner_device.increment();
    }
}

pub struct Bus {
    devices: Vec<VirtualDevice>,
}

impl Default for Bus {
    fn default() -> Self {
        Self::new()
    }
}

impl Bus {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
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

    pub fn get_devices_mut(&mut self) -> &mut Vec<VirtualDevice> {
        &mut self.devices
    }

    pub fn get_devices(&self) -> &Vec<VirtualDevice> {
        &self.devices
    }

    pub fn add_device(&mut self, device: VirtualDevice) {
        self.devices.push(device);
    }

    pub fn read(&self, address: u64, size: Sizes) -> Result<MemorySize> {
        for device in &self.devices {
            if device.base() <= address && address < device.base() + device.size() {
                return device.load(address - device.base(), size);
            }
        }

        Err(Exception::LoadAccessFault).context(format!("address: {address:#08X}, size: {size:?}"))
    }

    pub fn write(&mut self, address: u64, value: MemorySize, size: Sizes) -> Result<()> {
        for device in &mut self.devices {
            if device.base() <= address && address < device.base() + device.size() {
                return device.store(address - device.base(), size, value);
            }
        }

        Err(Exception::StoreAccessFault).context(format!("address: {address}, size: {size:?}"))
    }
}
