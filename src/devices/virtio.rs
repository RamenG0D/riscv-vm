use crate::{bus::Device, cpu::Cpu, memory::dram::Sizes, trap::Exception};

/// The address which virtio starts.
pub const VIRTIO_BASE: u32 = 0x1000_1000;
/// The address which virtio ends.
const VIRTIO_END: u32 = VIRTIO_BASE + 0x1000;

// Constants for Virtio Device Registers and Flags
pub const VIRTIO_IRQ: u32 = 1;

const VRING_DESC_SIZE: u32 = 16;
const QUEUE_SIZE: u32 = 8;
const SECTOR_SIZE: u32 = 512;

const VIRTQ_DESC_F_NEXT: u32 = 1;
const VIRTQ_DESC_F_WRITE: u32 = 2;

const _VIRTQ_DESC_F_INDIRECT: u32 = 4;

const MAGIC: u32 = VIRTIO_BASE;
const MAGIC_END: u32 = VIRTIO_BASE + 0x3;

const VERSION: u32 = VIRTIO_BASE + 0x4;
const VERSION_END: u32 = VIRTIO_BASE + 0x7;

const DEVICE_ID: u32 = VIRTIO_BASE + 0x8;
const DEVICE_ID_END: u32 = VIRTIO_BASE + 0xb;

const VENDOR_ID: u32 = VIRTIO_BASE + 0xc;
const VENDOR_ID_END: u32 = VIRTIO_BASE + 0xf;

const DEVICE_FEATURES: u32 = VIRTIO_BASE + 0x10;
const DEVICE_FEATURES_END: u32 = VIRTIO_BASE + 0x13;

const DEVICE_FEATURES_SEL: u32 = VIRTIO_BASE + 0x14;
const DEVICE_FEATURES_SEL_END: u32 = VIRTIO_BASE + 0x17;

const DRIVER_FEATURES: u32 = VIRTIO_BASE + 0x20;
const DRIVER_FEATURES_END: u32 = VIRTIO_BASE + 0x23;

const DRIVER_FEATURES_SEL: u32 = VIRTIO_BASE + 0x24;
const DRIVER_FEATURES_SEL_END: u32 = VIRTIO_BASE + 0x27;

const GUEST_PAGE_SIZE: u32 = VIRTIO_BASE + 0x28;
const GUEST_PAGE_SIZE_END: u32 = VIRTIO_BASE + 0x2b;

const QUEUE_SEL: u32 = VIRTIO_BASE + 0x30;
const QUEUE_SEL_END: u32 = VIRTIO_BASE + 0x33;

const QUEUE_NUM_MAX: u32 = VIRTIO_BASE + 0x34;
const QUEUE_NUM_MAX_END: u32 = VIRTIO_BASE + 0x37;

const QUEUE_NUM: u32 = VIRTIO_BASE + 0x38;
const QUEUE_NUM_END: u32 = VIRTIO_BASE + 0x3b;

const QUEUE_ALIGN: u32 = VIRTIO_BASE + 0x3c;
const QUEUE_ALIGN_END: u32 = VIRTIO_BASE + 0x3f;

const QUEUE_PFN: u32 = VIRTIO_BASE + 0x40;
const QUEUE_PFN_END: u32 = VIRTIO_BASE + 0x43;

const QUEUE_NOTIFY: u32 = VIRTIO_BASE + 0x50;
const QUEUE_NOTIFY_END: u32 = VIRTIO_BASE + 0x53;

const INTERRUPT_STATUS: u32 = VIRTIO_BASE + 0x60;
const INTERRUPT_STATUS_END: u32 = VIRTIO_BASE + 0x63;

const INTERRUPT_ACK: u32 = VIRTIO_BASE + 0x64;
const INTERRUPT_ACK_END: u32 = VIRTIO_BASE + 0x67;

const STATUS: u32 = VIRTIO_BASE + 0x70;
const STATUS_END: u32 = VIRTIO_BASE + 0x73;

const CONFIG: u32 = VIRTIO_BASE + 0x100;
const CONFIG_END: u32 = VIRTIO_BASE + 0x107;

// Structure representing Virtqueue addresses
#[derive(Debug, Copy, Clone)]
struct VirtqueueAddr {
    desc_addr: u32,
    avail_addr: u32,
    used_addr: u32,
}

impl VirtqueueAddr {
    fn new(virtio: &Virtio) -> Self {
        let base_addr = virtio.queue_pfn * virtio.guest_page_size;
        let align = virtio.queue_align;
        let size = virtio.queue_num;
        let avail_ring_end = base_addr + (16 * size) + (6 + 2 * size);

        Self {
            desc_addr: base_addr,
            avail_addr: base_addr + 16 * size,
            used_addr: ((avail_ring_end / align) + 1) * align,
        }
    }
}

// Structure representing a Virtqueue Descriptor
#[derive(Debug)]
struct VirtqDesc {
    addr: u32,
    len: u32,
    flags: u32,
    next: u32,
}

impl VirtqDesc {
    fn new(cpu: &mut Cpu, addr: u32) -> Result<Self, Exception> {
        Ok(Self {
            addr: cpu.bus_read(addr, Sizes::Word)?,
            len: cpu.bus_read(addr.wrapping_add(8), Sizes::Word)?,
            flags: cpu.bus_read(addr.wrapping_add(12), Sizes::HalfWord)?,
            next: cpu.bus_read(addr.wrapping_add(14), Sizes::HalfWord)?,
        })
    }
}

// Structure representing available ring in Virtqueue
#[derive(Debug)]
struct VirtqAvail {
    flags: u16,
    idx: u16,
    ring_start_addr: u32,
}

impl VirtqAvail {
    fn new(cpu: &mut Cpu, addr: u32) -> Result<Self, Exception> {
        Ok(Self {
            flags: cpu.bus_read(addr, Sizes::HalfWord)? as u16,
            idx: cpu.bus_read(addr.wrapping_add(2), Sizes::HalfWord)? as u16,
            ring_start_addr: addr.wrapping_add(4),
        })
    }
}

// Main Virtio Device structure
pub struct Virtio {
    id: u32,
    device_features: [u32; 2],
    device_features_sel: u32,
    driver_features: [u32; 2],
    driver_features_sel: u32,
    guest_page_size: u32,
    queue_num: u32,
    queue_align: u32,
    queue_pfn: u32,
    queue_notify: u32,
    interrupt_status: u32,
    status: u32,
    config: [u8; 8],
    disk: Vec<u8>,
    virtqueue: Option<VirtqueueAddr>,
}

impl Virtio {
    // Create a new Virtio device with default values
    pub fn new() -> Self {
        let mut config = [0; 8];
        config[1] = 0x20;
        config[2] = 0x03;

        Self {
            id: 0,
            device_features: Virtio::device_features(),
            device_features_sel: 0,
            driver_features: [0; 2],
            driver_features_sel: 0,
            guest_page_size: 0,
            queue_num: 0,
            queue_align: 0x1000,
            queue_pfn: 0,
            queue_notify: u32::MAX,
            interrupt_status: 0,
            status: 0,
            config,
            disk: Vec::new(),
            virtqueue: None,
        }
    }

    // Define device features
    fn device_features() -> [u32; 2] {
        let mut features = [0; 2];
        features[1] |= 1 << 3;
        features
    }

    // Initialize the Virtqueue
    fn init_virtqueue(&mut self) {
        let queue = VirtqueueAddr::new(self);
        self.virtqueue = Some(queue);
    }

    // Get Virtqueue or initialize it if not set
    fn virtqueue(&self) -> VirtqueueAddr {
        self.virtqueue.unwrap_or_else(|| VirtqueueAddr::new(self))
    }

    // Reset the Virtio device
    fn reset(&mut self) {
        self.id = 0;
        self.interrupt_status = 0;
    }

    // Check if the Virtio device is interrupting
    pub fn is_interrupting(&mut self) -> bool {
        if self.queue_notify != u32::MAX {
            self.queue_notify = u32::MAX;
            true
        } else {
            false
        }
    }

    // Initialize the Virtio disk with a binary
    pub fn initialize(&mut self, binary: Vec<u8>) {
        self.disk.extend(binary);
    }

    // Handle read operations from Virtio device registers
    pub fn read(&self, addr: u32, size: Sizes) -> Result<u32, Exception> {
        let (reg, offset) = match addr {
            MAGIC..=MAGIC_END => (0x74726976, addr - MAGIC),
            VERSION..=VERSION_END => (0x1, addr - VERSION),
            DEVICE_ID..=DEVICE_ID_END => (0x2, addr - DEVICE_ID),
            VENDOR_ID..=VENDOR_ID_END => (0x554d4551, addr - VENDOR_ID),
            DEVICE_FEATURES..=DEVICE_FEATURES_END => (
                self.device_features[self.device_features_sel as usize],
                addr - DEVICE_FEATURES,
            ),
            QUEUE_NUM_MAX..=QUEUE_NUM_MAX_END => (QUEUE_SIZE, addr - QUEUE_NUM_MAX),
            QUEUE_PFN..=QUEUE_PFN_END => (self.queue_pfn, addr - QUEUE_PFN),
            INTERRUPT_STATUS..=INTERRUPT_STATUS_END => (self.interrupt_status, addr - INTERRUPT_STATUS),
            STATUS..=STATUS_END => (self.status, addr - STATUS),
            CONFIG..=CONFIG_END => {
                if size != Sizes::Byte {
                    return Err(Exception::StoreAMOAccessFault);
                }
                let index = addr - CONFIG;
                return Ok(self.config[index as usize] as u32);
            }
            _ => return Err(Exception::LoadAccessFault),
        };

        let value = match size {
            Sizes::Byte => (reg >> (offset * 8)) & 0xff,
            Sizes::HalfWord => (reg >> (offset * 8)) & 0xffff,
            Sizes::Word => reg,
        };

        Ok(value)
    }

    // Handle write operations to Virtio device registers
    pub fn write(&mut self, addr: u32, value: u32, size: Sizes) -> Result<(), Exception> {
        let (mut reg, offset) = match addr {
            DEVICE_FEATURES_SEL..=DEVICE_FEATURES_SEL_END => (self.device_features_sel, addr - DEVICE_FEATURES_SEL),
            DRIVER_FEATURES..=DRIVER_FEATURES_END => (self.driver_features[self.driver_features_sel as usize], addr - DRIVER_FEATURES),
            DRIVER_FEATURES_SEL..=DRIVER_FEATURES_SEL_END => (self.driver_features_sel, addr - DRIVER_FEATURES_SEL),
            GUEST_PAGE_SIZE..=GUEST_PAGE_SIZE_END => (self.guest_page_size, addr - GUEST_PAGE_SIZE),
            QUEUE_SEL..=QUEUE_SEL_END => {
                if value != 0 {
                    panic!("Multiple virtual queues are not supported.");
                }
                return Ok(());
            }
            QUEUE_NUM..=QUEUE_NUM_END => (self.queue_num, addr - QUEUE_NUM),
            QUEUE_ALIGN..=QUEUE_ALIGN_END => (self.queue_align, addr - QUEUE_ALIGN),
            QUEUE_PFN..=QUEUE_PFN_END => (self.queue_pfn, addr - QUEUE_PFN),
            QUEUE_NOTIFY..=QUEUE_NOTIFY_END => (self.queue_notify, addr - QUEUE_NOTIFY),
            INTERRUPT_ACK..=INTERRUPT_ACK_END => (self.interrupt_status, addr - INTERRUPT_ACK),
            STATUS..=STATUS_END => (self.status, addr - STATUS),
            CONFIG..=CONFIG_END => {
                if size != Sizes::Byte {
                    return Err(Exception::StoreAMOAccessFault);
                }
                let index = addr - CONFIG;
                self.config[index as usize] = (value >> (index * 8)) as u8;
                return Ok(());
            }
            _ => return Err(Exception::StoreAMOAccessFault),
        };

        // Apply the write operation based on size
        match size {
            Sizes::Byte => {
                reg = reg & (!(0xff << (offset * 8)));
                reg |= (value & 0xff) << (offset * 8);
            }
            Sizes::HalfWord => {
                reg &= !(0xffff << (offset * 8));
                reg |= (value & 0xffff) << (offset * 8);
            }
            Sizes::Word => reg = value,
        }

        // Update the correct register after write
        match addr {
            DEVICE_FEATURES_SEL..=DEVICE_FEATURES_SEL_END => self.device_features_sel = reg,
            DRIVER_FEATURES..=DRIVER_FEATURES_END => self.driver_features[self.driver_features_sel as usize] = reg,
            DRIVER_FEATURES_SEL..=DRIVER_FEATURES_SEL_END => self.driver_features_sel = reg,
            GUEST_PAGE_SIZE..=GUEST_PAGE_SIZE_END => self.guest_page_size = reg,
            QUEUE_NUM..=QUEUE_NUM_END => self.queue_num = reg,
            QUEUE_ALIGN..=QUEUE_ALIGN_END => self.queue_align = reg,
            QUEUE_PFN..=QUEUE_PFN_END => self.queue_pfn = reg,
            QUEUE_NOTIFY..=QUEUE_NOTIFY_END => self.queue_notify = reg,
            INTERRUPT_ACK..=INTERRUPT_ACK_END => self.interrupt_status = reg,
            STATUS..=STATUS_END => {
                self.status = reg;
                if self.status == 0 {
                    self.reset();
                }
                if self.status & 0x4 == 1 {
                    self.init_virtqueue();
                }
                if (self.status & 128) == 1 {
                    panic!("Virtio: device status FAILED");
                }
            }
            _ => return Err(Exception::StoreAMOAccessFault),
        }

        Ok(())
    }

    // Read from the Virtio disk
    fn read_disk(&self, addr: u32) -> u32 {
        self.disk[addr as usize] as u32
    }

    // Write to the Virtio disk
    fn write_disk(&mut self, addr: u32, value: u32) {
        self.disk[addr as usize] = value as u8;
    }

    // Handle disk access through Virtqueue
    pub fn disk_access(cpu: &mut Cpu) -> Result<(), Exception> {
        cpu.virtio_mut().interrupt_status |= 0x1;

        let virtq = cpu.virtio().virtqueue();
        let avail = VirtqAvail::new(cpu, virtq.avail_addr)?;

        let head_index = cpu.bus_read(
            avail.ring_start_addr + (avail.idx as u32 % QUEUE_SIZE),
            Sizes::HalfWord,
        )?;

        let desc0 = VirtqDesc::new(cpu, virtq.desc_addr + VRING_DESC_SIZE * head_index)?;
        assert_eq!(desc0.flags & VIRTQ_DESC_F_NEXT, 1);

        let desc1 = VirtqDesc::new(cpu, virtq.desc_addr + VRING_DESC_SIZE * desc0.next)?;
        assert_eq!(desc1.flags & VIRTQ_DESC_F_NEXT, 1);

        let sector = cpu.bus_read(desc0.addr.wrapping_add(8), Sizes::Word)?;

        match desc1.flags & VIRTQ_DESC_F_WRITE == 0 {
            true => {
                for i in 0..desc1.len {
                    let data = cpu.bus_read(desc1.addr + i, Sizes::Byte)?;
                    cpu.virtio_mut().write_disk(sector * SECTOR_SIZE + i, data);
                }
            }
            false => {
                for i in 0..desc1.len {
                    let data = cpu.virtio().read_disk(sector * SECTOR_SIZE + i);
                    cpu.bus_write(desc1.addr + i, data, Sizes::Byte)?;
                }
            }
        };

        let desc2 = VirtqDesc::new(cpu, virtq.desc_addr + VRING_DESC_SIZE * desc1.next)?;
        assert_eq!(desc2.flags & VIRTQ_DESC_F_NEXT, 0);

        cpu.bus_write(desc2.addr, 0, Sizes::Byte)?;
        cpu.bus_write(
            virtq.used_addr.wrapping_add(4).wrapping_add((cpu.virtio().id % QUEUE_SIZE) * 8),
            head_index,
            Sizes::Word,
        )?;

        cpu.virtio_mut().id = cpu.virtio().id.wrapping_add(1);
        cpu.bus_write(virtq.used_addr.wrapping_add(2), cpu.virtio().id, Sizes::HalfWord)?;

        Ok(())
    }
}

impl Device for Virtio {
    fn base(&self) -> u32 {
        VIRTIO_BASE
    }

    fn size(&self) -> u32 {
        VIRTIO_END
    }

    fn load(&self, addr: u32, size: Sizes) -> Result<u32, Exception> {
        self.read(addr, size)
    }

    fn store(&mut self, addr: u32, size: Sizes, value: u32) -> Result<(), Exception> {
        self.write(addr, value, size)
    }
}
