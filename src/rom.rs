//! The rom module contains the read-only memory structure and implementation to read the memory. ROM includes a device tree blob (DTB) compiled from a device tree source (DTS).
use crate::bus::{Device, VirtualDevice};
use crate::log_info;
use crate::memory::dram::Sizes;
use crate::memory::virtual_memory::MemorySize;
use crate::trap::Exception;

/// The address which the mask ROM starts.
pub const MROM_BASE: u32 = 0x1000;
/// the size of the mask ROM
pub const MROM_SIZE: u32 = 0xf000;
/// The address which the mask ROM ends.
// const MROM_END: u32 = MROM_BASE + 0xf000;

const DTS: &str = r#"/dts-v1/;

/ {
    #address-cells = <0x02>;
    #size-cells = <0x02>;
    compatible = "riscv-virtio";
    model = "riscv-virtio,qemu";

    chosen {
        bootargs = "root=/dev/vda ro console=ttyS0";
        stdout-path = "/uart@10000000";
    };

    uart@10000000 {
        interrupts = <0xa>;
        interrupt-parent = <0x03>;
        clock-frequency = <0x384000>;
        reg = <0x0 0x10000000 0x0 0x100>;
        compatible = "ns16550a";
    };

    virtio_mmio@10001000 {
        interrupts = <0x01>;
        interrupt-parent = <0x03>;
        reg = <0x0 0x10001000 0x0 0x1000>;
        compatible = "virtio,mmio";
    };

    cpus {
        #address-cells = <0x01>;
        #size-cells = <0x00>;
        timebase-frequency = <0x989680>;

        cpu-map {
            cluster0 {
                core0 {
                    cpu = <0x01>;
                };
            };
        };

        cpu@0 {
            phandle = <0x01>;
            device_type = "cpu";
            reg = <0x00>;
            status = "okay";
            compatible = "riscv";
            riscv,isa = "rv32imac";
            mmu-type = "riscv,sv48";

            interrupt-controller {
                #interrupt-cells = <0x01>;
                interrupt-controller;
                compatible = "riscv,cpu-intc";
                phandle = <0x02>;
            };
        };
    };

	memory@80000000 {
		device_type = "memory";
		reg = <0x0 0x80000000 0x0 0x8000000>;
	};

    soc {
        #address-cells = <0x02>;
        #size-cells = <0x02>;
        compatible = "simple-bus";
        ranges;

        interrupt-controller@c000000 {
            phandle = <0x03>;
            riscv,ndev = <0x35>;
            reg = <0x00 0xc000000 0x00 0x4000000>;
            interrupts-extended = <0x02 0x0b 0x02 0x09>;
            interrupt-controller;
            compatible = "riscv,plic0";
            #interrupt-cells = <0x01>;
            #address-cells = <0x00>;
        };

        clint@2000000 {
            interrupts-extended = <0x02 0x03 0x02 0x07>;
            reg = <0x00 0x2000000 0x00 0x10000>;
            compatible = "riscv,clint0";
        };
    };
};"#;

/// Read a dtb file. First, create a dts file. Second, compile it to a dtb file. Finally, read the dtb file and return the binary content.
fn dtb() -> Vec<u8> {
    // instead we should use a library so we dont have to rely on the user having dtc installed
    use devicetree_tool::DeviceTree;
    // turn our dtb string into bytes
    let dt = DeviceTree::from_dts_bytes(DTS.as_bytes());
    dt.generate_dtb()

    // TODO: Make this content more flexible depending on the number of cpus.
    // Reference code is https://github.com/riscv/riscv-isa-sim/blob/66b44bfbedda562a32e4a2cd0716afbf731b69cd/riscv/dts.cc#L38-L54
    /*const DTB_NAME: &str = "device_tree_blob.dtb";
    const DTS_NAME: &str = "device_tree_source.dts";

    std::fs::write(DTS_NAME, DTS).expect("Failed to write the device tree source");

    Command::new("dtc")
        .arg("-I")
        .arg("dts")
        .arg("-O")
        .arg("dtb")
        .arg("-o")
        .arg(DTB_NAME)
        .arg(DTS_NAME)
        .output()
        .expect("Failed to compile the device tree source");

    std::fs::read("device_tree_blob.dtb").expect("Failed to read the device tree blob")*/
}

/// The read-only memory (ROM).
pub struct Rom {
    data: Vec<u8>,
}

impl Rom {
    /// Create a new `rom` object.
    pub fn new() -> Self {
        let mut dtb = dtb();
        log_info!("The size of the device tree blob (DTB): {}", dtb.len());

        // TODO: set a reset vector correctly.
        // 0x20 is the size of a reset vector.
        let mut rom = vec![0; 32];
        rom.append(&mut dtb);

        log_info!("The size of the ROM: {}", rom.len());

        let align = 0x1000;
        rom.resize((rom.len() + align - 1) / align * align, 0);

        Self::new_with_data(rom)
    }

    pub fn new_device() -> VirtualDevice {
        VirtualDevice::new(Box::new(Self::new()), MROM_BASE, MROM_SIZE)
    }

    pub fn new_with_data(data: Vec<u8>) -> Rom {
        log_info!("Initializing the ROM with the data of size: {}", data.len());
        Rom { data }
    }

    /// Load `size`-bit data from the memory.
    pub fn read(&self, addr: u32, size: Sizes) -> Result<u32, Exception> {
        match size {
            Sizes::Byte => Ok(self.read8(addr)),
            Sizes::HalfWord => Ok(self.read16(addr)),
            Sizes::Word => Ok(self.read32(addr)),
        }
    }

    /// Store `size`-bit data to the memory. Returns the exception because the ROM is read-only.
    pub fn write(&self, _addr: u32, _value: u32, _size: u8) -> Result<(), Exception> {
        Err(Exception::StoreAccessFault)
    }

    /// Read a byte from the rom.
    fn read8(&self, addr: u32) -> u32 {
        self.data[addr as usize] as u32
    }

    /// Read 2 bytes from the rom.
    fn read16(&self, addr: u32) -> u32 {
        (self.data[addr as usize] as u32) | ((self.data[(addr as usize) + 1] as u32) << 8)
    }

    /// Read 4 bytes from the rom.
    fn read32(&self, addr: u32) -> u32 {
        (self.data[addr as usize] as u32)
            | ((self.data[(addr as usize) + 1] as u32) << 8)
            | ((self.data[(addr as usize) + 2] as u32) << 16)
            | ((self.data[(addr as usize) + 3] as u32) << 24)
    }
}

impl Device for Rom {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn load(&self, addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        self.read(addr, size)
    }

    fn store(
        &mut self,
        _addr: MemorySize,
        _size: Sizes,
        _value: MemorySize,
    ) -> Result<(), Exception> {
        Err(Exception::StoreAccessFault)
    }
}

impl Default for Rom {
    fn default() -> Self {
        Self::new()
    }
}
