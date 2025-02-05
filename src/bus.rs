use crate::{memory::{dram::Sizes, virtual_memory::MemorySize}, trap::Exception};
use log::error;

pub trait Device {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception>;
    fn store(&mut self, addr: MemorySize, size: Sizes, value: MemorySize) -> Result<(), Exception>;

	fn increment(&mut self) {}
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

	pub fn increment(&mut self) {
		self.inner_device.increment();
	}
}

pub struct Bus {
    devices: Vec<VirtualDevice>,
}

impl Bus {
    pub fn new() -> Self {
        Self { devices: Vec::new() }
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

    pub fn read(&self, address: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        for device in &self.devices {
            if device.base() <= address && address < device.base() + device.size() {
                return device.load(address - device.base(), size);
            }
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
                return device.store(address - device.base(), size, value);
            }
        }
        error!("Bus Write StoreAMOAccessFault: {:#X}", address);
        Err(Exception::StoreAccessFault)
    }
}
