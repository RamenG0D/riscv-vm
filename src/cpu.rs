use crate::{
    bit_ops::zero_extend, bus::{Bus, Device, VirtualDevice}, csr::{ CpuCsr, Csr, CsrAddress, MEPC, MSTATUS, SATP }, memory::{
        dram::{Sizes, DRAM_BASE, DRAM_SIZE},
        virtual_memory::MemorySize,
    }, registers::{FRegisters, XRegisterSize, XRegisters}, trap::{Exception, Trap}
};
use bit_ops::BitOps;
use log::{debug, error, info, trace, warn};
use riscv_decoder::{
    decoded_inst::InstructionDecoded, decoder::try_decode
};

pub const POINTER_TO_DTB: u32 = 0x1020;

/// The page size (4 KiB) for the virtual memory system.
const PAGE_SIZE: u32 = 4096;
/// The size of a page table entry.
const PTE_SIZE: u32 = 4;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessType {
	Executable,
	Readable,
	Writable,
	None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Privilege {
	User,
	Supervisor,
	Machine,
	// Reserved,
}

pub struct Mem {
	/// program counter
    pc: XRegisterSize,

    /// little endian memory
    bus: Bus,

	/// Csr registers
	csr: CpuCsr,

	/// the current Privilege level of the cpu dur5
	privilege: Privilege,

	/// SV32 paging flag.
    enable_paging: bool,
    /// Physical page number (PPN)
    ppn: u32,
}

impl Mem {
	pub fn new() -> Self {
		Self {
			pc: DRAM_BASE,
            bus: Bus::new(),
			csr: CpuCsr::new(),
			enable_paging: false,
			ppn: 0,
			privilege: Privilege::Machine
		}
	}

	pub fn dump_csr(&self) {
		self.csr.dump();
	}

	fn walk_page(&mut self, address: XRegisterSize, level: u32, pppn: u32, vpns: &[u32; 2], access: AccessType) -> Result<XRegisterSize, Exception> {
		info!("Traversing page: VA: {:#X} Level: {:#X} PPN: {:#X} VPNs: {{ {3:}({3:#X}), {4:}({4:#X}) }}", address, level, pppn, vpns[0], vpns[1]);
		let pte_address = (pppn * PAGE_SIZE) + (vpns[level as usize] * PTE_SIZE);
		let pte = self.read_raw(pte_address, Sizes::Word)?;
		info!("PTE: {:#X}", pte);
		let ppn = pte.get_bits(22, 10);
		let ppns = [
			pte.get_bits(10, 10),
			pte.get_bits(12, 20),
		];
		// let _rsw = pte.get_bits(2, 8);
		let d = pte.is_set(7);
		let a = pte.is_set(6);
		// let _g = pte.is_set(5);
		// let _u = pte.is_set(4);
		let x = pte.is_set(3);
		let w = pte.is_set(2);
		let r = pte.is_set(1);
		let v = pte.is_set(0);

		info!("d: {d}");
		info!("a: {a}");
		info!("x: {x}");
		info!("w: {w}");
		info!("r: {r}");
		info!("v: {v}");

		if !v || (!r && w) {
			warn!("Page not valid or writable");
			return Err(Exception::LoadPageFault(address));
		}

		if !r && !x {
			return match level {
				0 => {
					warn!("Leaf page not readable or executable");
					Err(Exception::LoadPageFault(address))
				}
				_ => {
					info!("Walking to next level");
					self.walk_page(address, level - 1, ppn, vpns, access)
				}
			};
		}

		// Leaf page found

		if a || (match &access { AccessType::Writable => !d, _ => false }) {
			info!("Page accessed or dirty");
			let mut new_pte = pte.set_bit(6);
			if access == AccessType::Writable {
				new_pte = new_pte.set_bit(7);
			}
			self.write_raw(pte_address, new_pte, Sizes::Word)?;
		}

		match access {
			AccessType::Executable => if !x {
				warn!("Page not executable");
				return Err(Exception::LoadPageFault(address));
			},
			AccessType::Readable => if !r {
				warn!("Page not readable");
				return Err(Exception::LoadPageFault(address));
			},
			AccessType::Writable => if !w {
				warn!("Page not writable");
				return Err(Exception::LoadPageFault(address));
			},
			_ => {}
		};

		let offset = bit_ops::bitops_u32::get_bits(address, 12, 0);
		// @TODO: Optimize
		let p_address = match level {
			1 => {
				if ppns[0] != 0 {
					warn!("ppns[0] != 0");
					return Err(Exception::LoadPageFault(address));
				}
				(ppns[1] << 22) | (vpns[0] << 12) | offset
			},
			0 => (pppn << 12) | offset,
			_ => unreachable!(),
		};

		info!("Hit: {:#X}", p_address);

		Ok(p_address)
	}

	/*
	The risc-v Sv32 scheme has two levels of page-table
	pages. A page-table page contains 1024 32-bit PTEs.
	A 32-bit virtual address is split into five fields:
	  22..31 -- 10 bits of level-1 index.
	  12..21 -- 10 bits of level-0 index.
	   0..11 -- 12 bits of byte offset within the page
	The 32 bit PTE looks like this:
	  20..31 -- 12 bits of level-1 index.
	  10..19 -- 12 bits of level-0 index.
	   8.. 9 --  2 bits reserved for OS
	   0.. 7 -- flags: Valid/Read/Write/Execute/User/Global/Accessed/Dirty
	*/
	fn translate_address(&mut self, addr: XRegisterSize, access: AccessType) -> Result<XRegisterSize, Exception> {
		if !self.enable_paging {
			return Ok(addr);
		}

		info!("Translating address: {:#X}", addr);
		info!("Privilege: {:?}", self.privilege);

		// 4.3.2 Virtual Address Translation Process
        // (The RISC-V Instruction Set Manual Volume II-Privileged Architecture_20190608)
        // A virtual address va is translated into a physical address pa as follows:
        /*let vpns = [
			addr.get_bits(10, 12),
			addr.get_bits(10, 22),
		];
		let levels = 2;
		info!("ADDRESS = {0:#X}[{0:#b}]", addr);
		info!("VPN[0]  = {0:#010X}[{0:#012b}]", vpns[0]);
		info!("VPN[1]  = {0:#010X}[{0:#012b}]", vpns[1]);

        // 1. Let a be satp.ppn × PAGESIZE, and let i = LEVELS − 1. (For Sv32, PAGESIZE=212 and LEVELS=2.)
        let mut page = self.ppn * PAGE_SIZE;
        let mut i: i32 = levels - 1;
        let mut pte;

		info!("PAGE: {:#X}", self.ppn);

        loop {
            // 2. Let pte be the value of the PTE at address a+va.vpn[i]×PTESIZE. (For Sv32,
            //    PTESIZE=4.) If accessing pte violates a PMA or PMP check, raise an access
            //    exception corresponding to the original access type.
			info!("PTE ADDRESS: {:#X}", vpns[i as usize] * PTE_SIZE);
            pte = self.read_raw(vpns[i as usize] * PTE_SIZE, Sizes::Word)?;
			info!("PTE: {:#X}", pte);

            // 3. If pte.v = 0, or if pte.r = 0 and pte.w = 1, stop and raise a page-fault
            //    exception corresponding to the original access type.
            let v = pte.is_set(0);
            let r = pte.is_set(1);
            let w = pte.is_set(2);
            let x = pte.is_set(3);

			info!("V: {v}, R: {r}, W: {w}, X: {x}");

            if !v || (!r && w) {
				error!("Page not valid or writable");
                match &access {
                    AccessType::Executable => return Err(Exception::InstructionPageFault(addr)),
                    AccessType::Readable => return Err(Exception::LoadPageFault(addr)),
                    AccessType::Writable => return Err(Exception::StorePageFault(addr)),
					AccessType::None => return Err(Exception::InstructionPageFault(addr)),
                }
            }

            // 4. Otherwise, the PTE is valid. If pte.r = 1 or pte.x = 1, go to step 5.
            //    Otherwise, this PTE is a pointer to the next level of the page table.
            //    Let i = i − 1. If i < 0, stop and raise a page-fault exception
            //    corresponding to the original access type. Otherwise,
            //    let a = pte.ppn × PAGESIZE and go to step 2.
            if r || x {
				info!("Skipping to step 5");
                break;
            }

            i -= 1;

			let ppn = pte.get_bits(22, 10);
            page = ppn * PAGE_SIZE;

			info!("PPN: {:#X}", ppn);
			info!("Page: {:#X}", page);

			if i < 0 {
                match &access {
                    AccessType::Executable => return Err(Exception::InstructionPageFault(addr)),
                    AccessType::Readable => return Err(Exception::LoadPageFault(addr)),
                    AccessType::Writable => return Err(Exception::StorePageFault(addr)),
					AccessType::None => return Err(Exception::InstructionPageFault(addr)),
                }
            }
        }

        // TODO: implement step 5
        // 5. A leaf PTE has been found. Determine if the requested memory access is
        //    allowed by the pte.r, pte.w, pte.x, and pte.u bits, given the current
        //    privilege mode and the value of the SUM and MXR fields of the mstatus
        //    register. If not, stop and raise a page-fault exception corresponding
        //    to the original access type.

        // 3.1.6.3 Memory Privilege in mstatus Register
        // "The MXR (Make eXecutable Readable) bit modifies the privilege with which loads access
        // virtual memory. When MXR=0, only loads from pages marked readable (R=1 in Figure 4.15)
        // will succeed. When MXR=1, loads from pages marked either readable or executable
        // (R=1 or X=1) will succeed. MXR has no effect when page-based virtual memory is not in
        // effect. MXR is hardwired to 0 if S-mode is not supported."

        // "The SUM (permit Supervisor User Memory access) bit modifies the privilege with which
        // S-mode loads and stores access virtual memory. When SUM=0, S-mode memory accesses to
        // pages that are accessible by U-mode (U=1 in Figure 4.15) will fault. When SUM=1, these
        // accesses are permitted.  SUM has no effect when page-based virtual memory is not in
        // effect. Note that, while SUM is ordinarily ignored when not executing in S-mode, it is
        // in effect when MPRV=1 and MPP=S. SUM is hardwired to 0 if S-mode is not supported."

        // 6. If i > 0 and pte.ppn[i−1:0] != 0, this is a misaligned superpage; stop and
        //    raise a page-fault exception corresponding to the original access type.
		let ppns = [
			pte.get_bits(10, 10),
			pte.get_bits(12, 20),
		];

		info!("PPNs: {{ {0:}({0:#X}), {1:}({1:#X}) }}", ppns[0], ppns[1]);

        if i > 0 {
			info!("Checking for misaligned superpage");

            for j in (0..i).rev() {
                if ppns[j as usize] != 0 {
					info!("superpage is misaligned");

                    // A misaligned superpage.
                    match &access {
                        AccessType::Executable => return Err(Exception::InstructionPageFault(addr)),
                        AccessType::Readable => return Err(Exception::LoadPageFault(addr)),
                        AccessType::Writable => return Err(Exception::StorePageFault(addr)),
						AccessType::None => return Err(Exception::InstructionPageFault(addr)),
                    }
                }
            }
        }

        // 7. If pte.a = 0, or if the memory access is a store and pte.d = 0, either raise
        //    a page-fault exception corresponding to the original access type, or:
        //    • Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
        //    • If this access violates a PMA or PMP check, raise an access exception
        //    corresponding to the original access type.
        //    • This update and the loading of pte in step 2 must be atomic; in particular,
        //    no intervening store to the PTE may be perceived to have occurred in-between.
        let a = pte.is_set(6);
        let d = pte.is_set(7);

		info!("A: {a}, D: {d}");

        if !a || (access == AccessType::Writable && !d) {
			info!("Setting pte.a to 1 and pte.d to 1");

            // Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
            pte = if let AccessType::Writable = access {
				pte.set_bit(6).set_bit(7)
            } else {
				pte.set_bit(6)
			};

            // TODO: PMA or PMP check.

            // Update the value of address satp.ppn × PAGESIZE + va.vpn[i] × PTESIZE with new pte
            // value.
            // TODO: If this is enabled, running xv6 fails.
            // self.bus.write(self.page_table + vpn[i as usize] * 8, pte, 64)?;
        }

        // 8. The translation is successful. The translated physical address is given as
        //    follows:
        //    • pa.pgoff = va.pgoff.
        //    • If i > 0, then this is a superpage translation and pa.ppn[i−1:0] =
        //    va.vpn[i−1:0].
        //    • pa.ppn[LEVELS−1:i] = pte.ppn[LEVELS−1:i].
        let offset = addr.get_bits(11, 0);

		info!("Offset: {:#X}", offset);
		info!("I: {:#X}", i);

        let paddr = match i {
			1 => {
				if ppns[0] != 0 {
					warn!("ppns[0] != 0");
					return Err(Exception::LoadPageFault(addr));
				}
				(ppns[1] << 22) | (vpns[0] << 12) | offset
			},
			0 => (page << 12) | offset,
            _ => match access {
                AccessType::Executable => return Err(Exception::InstructionPageFault(addr)),
                AccessType::Readable => return Err(Exception::LoadPageFault(addr)),
                AccessType::Writable => return Err(Exception::StorePageFault(addr)),
				AccessType::None => return Err(Exception::InstructionPageFault(addr)),
            },
        };

		info!("Translated address: {:#X}", paddr);

		Ok(paddr)*/

		match &self.privilege {
			Privilege::Machine => match &access {
				AccessType::Executable => Ok(addr),
				_ => match self.read_csr(MSTATUS).get_bit(17) {
					0 => Ok(addr),
					_ => {
						let privilege_mode = match self.read_csr(MSTATUS).get_bits(2, 9) {
							0 => Privilege::User,
							1 => Privilege::Supervisor,
							_ => Privilege::Machine,
						};
						match privilege_mode {
							Privilege::Machine => Ok(addr),
							_ => {
								let current_privilege_mode = self.get_privilege().clone();
								self.set_privilege(privilege_mode);
								let result = self.translate_address(addr, access);
								self.set_privilege(current_privilege_mode);
								result
							}
						}
					}
				}
			},
			Privilege::User | Privilege::Supervisor => {
				let vpns = [
					addr.get_bits(10, 12),
					addr.get_bits(10, 22),
				];

				self.walk_page(addr, 1, self.ppn, &vpns, access)
			}
		}
	}

	fn read_raw(&mut self, addr: XRegisterSize, size: Sizes) -> Result<XRegisterSize, Exception> {
		self.bus.read(addr, size)
	}
	fn write_raw(&mut self, addr: XRegisterSize, value: XRegisterSize, size: Sizes) -> Result<(), Exception> {
		self.bus.write(addr, value, size)
	}
}

impl Cpu for Mem {
	fn get_pc(&self) -> XRegisterSize {
		self.pc
	}
	fn set_pc(&mut self, value: XRegisterSize) {
		self.pc = value;
	}

	fn read(&mut self, addr: XRegisterSize, size: Sizes, access: AccessType) -> Result<XRegisterSize, Exception> {
		let paddr = self.translate_address(addr, access)?;
		self.bus.read(paddr, size)
	}
	fn write(&mut self, addr: XRegisterSize, value: MemorySize, size: Sizes, access: AccessType) -> Result<(), Exception> {
		let paddr = self.translate_address(addr, access)?;
		self.bus.write(paddr, value, size)
	}

	fn state(&mut self) -> &impl Csr {
		&self.csr
	}
	fn state_mut(&mut self) -> &mut impl Csr {
		&mut self.csr
	}

	fn get_privilege(&self) -> Privilege {
		self.privilege
	}
	fn set_privilege(&mut self, npriv: Privilege) -> () {
		self.privilege = npriv;
	}

	fn update_paging(&mut self, value: u32) -> () {
		info!("Updating paging");

		// code in xv6-riscv used to enable paging
		// #define SATP_SV32 (1L << 31)
		// #define MAKE_SATP(pagetable) (SATP_SV32 | (((uint32)pagetable) >> 12)) // 32 bit
		// w_satp(MAKE_SATP(kernel_pagetable));
		self.ppn = value.get_bits(22, 0);
		self.enable_paging = value.is_set(31);
		// self.enable_paging = value & 0x80000000 != 0;
		// self.ppn = value & 0x3fffff;

		// println!("SATP: {0:}({0:#X})[{0:#032b}]", value);
		// println!("PPN: {0:}({0:#X})[{0:#032b}]", self.ppn);

		info!("Paging enabled: {}, ppn: {:#X}", self.enable_paging, self.ppn);
	}
}

// 32 bit RiscV CPU architecture
pub struct Riscv32Cpu {
	exec: Executor,
	mem: Mem,
}

impl Cpu for Riscv32Cpu {
	fn get_pc(&self) -> XRegisterSize {
		self.mem.get_pc()
	}
	fn get_privilege(&self) -> Privilege {
		self.mem.get_privilege()
	}
	fn set_pc(&mut self, value: XRegisterSize) -> () {
		self.mem.set_pc(value);
	}
	fn read(&mut self, addr: XRegisterSize, size: Sizes, access: AccessType) -> Result<XRegisterSize, Exception> {
		self.mem.read(addr, size, access)
	}
	fn write(&mut self, addr: XRegisterSize, value: MemorySize, size: Sizes, access: AccessType) -> Result<(), Exception> {
		self.mem.write(addr, value, size, access)
	}
	fn state(&mut self) -> &impl Csr {
		self.mem.state()
	}
	fn state_mut(&mut self) -> &mut impl Csr {
		self.mem.state_mut()
	}
	fn set_privilege(&mut self, npriv: Privilege) -> () {
		self.mem.set_privilege(npriv)
	}
	fn update_paging(&mut self, value: u32) -> () {
		self.mem.update_paging(value)
	}
}

impl Riscv32Cpu {
	pub fn new() -> Self {
        trace!("Initializing CPU...");
        let mut exec = Executor::new();
        exec.xregs[2] = DRAM_BASE + DRAM_SIZE; // stack pointer
        exec.xregs[11] = POINTER_TO_DTB; // pointer to device tree blob

        Self {
			mem: Mem::new(),
			exec,
        }
    }

	pub fn dump_csr(&self) {
		self.mem.dump_csr();
	}

	pub fn get_interface(&mut self) -> &mut Mem {
		&mut self.mem
	}

	pub fn read(&mut self, addr: MemorySize, size: Sizes, access: AccessType) -> Result<MemorySize, Exception> {
		self.mem.read(addr, size, access)
	}

	pub fn write(&mut self, addr: MemorySize, value: MemorySize, size: Sizes, access: AccessType) -> Result<(), Exception> {
		self.mem.write(addr, value, size, access)
	}

	pub fn get_pc(&self) -> MemorySize {
		self.mem.get_pc()
	}

	pub fn set_pc(&mut self, pc: MemorySize) {
		self.mem.set_pc(pc);
	}

	pub fn dump_registers(&self) {
		self.exec.dump_registers(&self.mem);
	}

    pub fn get_device<T>(&self) -> Option<&T>
    where
        T: Device + 'static,
    {
        self.mem.bus.get_device()
    }

    pub fn get_device_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Device + 'static,
    {
        self.mem.bus.get_device_mut()
    }

	pub fn get_devices_mut(&mut self) -> &mut Vec<VirtualDevice> {
		self.mem.bus.get_devices_mut()
	}

	pub fn get_devices(&self) -> &Vec<VirtualDevice> {
		self.mem.bus.get_devices()
	}

    pub fn add_device(&mut self, device: VirtualDevice) {
        self.mem.bus.add_device(device);
    }

    pub fn get_register(&self, register: XRegisterSize) -> Result<&XRegisterSize, String> {
        match register {
            0..=31 => Ok(&self.exec.xregs[register as usize]),
            32 => Ok(&self.mem.pc),
            _ => Err(format!(
                "The register '{register}' is not an addressable register?"
            )),
        }
    }

    pub fn get_register_mut(
        &mut self,
        register: XRegisterSize,
    ) -> Result<&mut XRegisterSize, String> {
        match register {
            0..=31 => Ok(&mut self.exec.xregs[register as usize]),
            32 => Ok(&mut self.mem.pc),
            _ => Err(format!(
                "The register '{register}' is not an addressable register?"
            )),
        }
    }

	pub fn dump_memory(&mut self, addr: MemorySize, size: MemorySize) {
        debug!("{:-^80}", "memory");
        for i in 0..size {
            let value = self.read(addr + i, Sizes::Byte, AccessType::Readable).unwrap();
            debug!("{:#08x}: {:#02x}", addr + i, value);
        }
        debug!("{:-^80}", "");
    }

	fn devices_increment(&mut self) {
		for device in self.get_devices_mut().iter_mut() {
			device.increment();
		}
	}

	pub fn step(&mut self) -> Result<(), Exception> {
		self.devices_increment();

		let inst = self.fetch()?;

		// Execute an instruction.
		let trap = match self.exec.execute(&mut self.mem, inst) {
			Ok(_) => Trap::Requested, // Return a placeholder trap
			Err(exception) => {
				warn!("Taking trap: {:#?}", exception);
				exception.take_trap(self.get_interface())
			}
		};

		if let Trap::Fatal = trap {
			error!("pc: {:#x}, trap {:#?}", self.get_pc(), trap);
		}

		Ok(())
	}

	pub fn run(&mut self) -> Result<(), Exception> {
		loop {
			self.step()?;
		}
	}

    pub fn fetch(&mut self) -> Result<InstructionDecoded, Exception> {
        // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
        // should be `Exception::InstructionAccessFault`.
		// and we also need to check if we are reading from a virtual address or a physical address
		let paddr = self.mem.translate_address(self.get_pc(), AccessType::Executable)?;
		let inst = self.mem.read_raw(paddr, Sizes::Word)?;

        /*if is_compressed(inst) {
            self.mem.pc += 2;
        } else {
            self.mem.pc += 4;
        }*/
		self.mem.pc += 4;

        // decode the instruction (automatically detects if compressed)
        let inst = try_decode(inst).map_err(|e| {
            error!("Illegal instruction: {:#X} => {e:?}", inst);
            Exception::IllegalInstruction(inst)
        })?;

        debug!("{:#08X}: {inst}", self.get_pc() - 4);

        Ok(inst)
    }
}

pub trait Cpu {
	fn set_pc(&mut self, value: XRegisterSize) -> ();
	fn get_pc(&self) -> XRegisterSize;

	fn write(&mut self, addr: XRegisterSize, value: MemorySize, size: Sizes, access: AccessType) -> Result<(), Exception>;
	fn read(&mut self, addr: XRegisterSize, size: Sizes, access: AccessType) -> Result<XRegisterSize, Exception>;

	fn state(&mut self) -> &impl Csr;
	fn state_mut(&mut self) -> &mut impl Csr;

	fn read_csr(&mut self, addr: CsrAddress) -> u32 {
		self.state().read(addr)
	}
	fn write_csr(&mut self, addr: CsrAddress, value: u32) {
		self.state_mut().write(addr, value);
		if addr == SATP {
			self.update_paging(value);
		}
	}

	fn set_privilege(&mut self, npriv: Privilege) -> ();
	fn get_privilege(&self) -> Privilege;

	fn update_paging(&mut self, value: u32) -> ();
}

struct Executor {
	xregs: XRegisters,
    fregs: FRegisters,
}

impl Executor {
	pub fn new() -> Self {
		Self {
			fregs: FRegisters::new(),
			xregs: XRegisters::new(),
		}
	}

    pub fn execute(&mut self, cpu: &mut impl Cpu, inst: InstructionDecoded) -> Result<(), Exception> {
        // x0 must always be zero (irl the circuit is literally hardwired to electriacal equivalent of 0)
        self.xregs[0] = 0;

        match inst {
            InstructionDecoded::Add { rd, rs1, rs2 } => {
                trace!("ADD: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                trace!("rs1 = {rs1}, rs2 = {rs2}");
                self.xregs[rd as usize] = (rs1 + rs2) as XRegisterSize;
            }
            InstructionDecoded::Addi { rd, rs1, imm } => {
                trace!("ADDI: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_add(imm as i32) as XRegisterSize;
            }
            InstructionDecoded::Sub { rd, rs1, rs2 } => {
                trace!("SUB: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_sub(rs2) as XRegisterSize;
            }
            InstructionDecoded::And { rd, rs1, rs2 } => {
                trace!("AND: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 & rs2) as u32;
            }
            InstructionDecoded::Andi { rd, rs1, imm } => {
                trace!("ANDI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                self.xregs[rd as usize] = (rs1 & imm as i32) as u32;
            }
            InstructionDecoded::Or { rd, rs1, rs2 } => {
                trace!("OR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 | rs2;
            }
            InstructionDecoded::Ori { rd, rs1, imm } => {
                trace!("ORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1 | imm;
            }
            InstructionDecoded::Xor { rd, rs1, rs2 } => {
                trace!("XOR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 ^ rs2;
            }
            InstructionDecoded::Xori { rd, rs1, imm } => {
                trace!("XORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1 ^ imm;
            }
            InstructionDecoded::Sll { rd, rs1, rs2 } => {
                trace!("SLL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 << rs2;
            }
            InstructionDecoded::Slli { rd, rs1, imm } => {
                trace!("SLLI: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);

                let rs1 = self.xregs[rs1 as usize];

                self.xregs[rd as usize] = rs1 << imm;
            }
            InstructionDecoded::Srl { rd, rs1, rs2 } => {
                trace!("SRL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 >> rs2) as u32;
            }
            InstructionDecoded::Srli { rd, rs1, imm } => {
                trace!("SRLI: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let rs1 = self.xregs[rs1 as usize] as i32;

                self.xregs[rd as usize] = (rs1 >> imm) as u32;
            }
            InstructionDecoded::Sra { rd, rs1, rs2 } => {
                trace!("SRA: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 >> rs2) as u32;
            }
            InstructionDecoded::Srai { rd, rs1, imm } => {
                trace!("SRAI: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let rs1 = self.xregs[rs1 as usize] as i32;

                self.xregs[rd as usize] = (rs1 >> imm as i32) as u32;
            }
            InstructionDecoded::Lui { rd, imm } => {
                trace!("LUI: rd: {rd}, imm: {}", imm << 12);
                self.xregs[rd as usize] = imm << 12;
            }
            InstructionDecoded::AuiPc { rd, imm } => {
                trace!("AUIPC: rd: {rd}, imm: {imm}");
                self.xregs[rd as usize] = cpu.get_pc().wrapping_add(imm << 12).wrapping_sub(4) as XRegisterSize;
            }
            InstructionDecoded::Jal { rd, imm } => {
                trace!("JAL: rd: {rd}, imm: {imm}");
                self.xregs[rd as usize] = cpu.get_pc();

                let npc = cpu.get_pc().wrapping_add(imm).wrapping_sub(4);
                cpu.set_pc(npc);
            }
            InstructionDecoded::Jalr { rd, rs1, imm } => {
                trace!(
                    "JALR: rd = {rd}, rs1 = {rs1}, imm = {imm}",
                    imm = imm as i32
                );
                let t = cpu.get_pc();
                let target = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                cpu.set_pc(target);
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::Beq { rs1, rs2, imm } => {
                trace!("BEQ: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 == rs2 {
                    let (pc, imm) = (cpu.get_pc() as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    trace!("Branching to {:#X}", npc);
                    cpu.set_pc(npc);
                }
            }
            InstructionDecoded::Bne { rs1, rs2, imm } => {
                trace!("BNE: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 != rs2 {
                    let (pc, imm) = (cpu.get_pc() as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    trace!("Branching to {:#X}", npc);
                    cpu.set_pc(npc);
                }
            }
            InstructionDecoded::Blt { rs1, rs2, imm } => {
                trace!("BLT: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 < rs2 {
                    let (pc, imm) = (cpu.get_pc() as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    trace!("Branching to {:#X}", npc);
                    cpu.set_pc(npc);
                }
            }
            InstructionDecoded::Bge { rs1, rs2, imm } => {
                trace!("BGE: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 >= rs2 {
                    let (pc, imm) = (cpu.get_pc() as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    trace!("Branching to {:#X}", npc);
                    cpu.set_pc(npc);
                }
            }
            InstructionDecoded::Bltu { rs1, rs2, imm } => {
                trace!("BLTU: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 < rs2 {
                    let (pc, imm) = (cpu.get_pc() as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    trace!("Branching to {:#X}", npc);
                    cpu.set_pc(npc);
                }
            }
            InstructionDecoded::Bgeu { rs1, rs2, imm } => {
                trace!("BGEU: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 >= rs2 {
                    let (pc, imm) = (cpu.get_pc() as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    trace!("Branching to {:#X}", npc);
                    cpu.set_pc(npc);
                }
            }
            InstructionDecoded::Lb { rd, rs1, imm } => {
                trace!("LB: rd: {rd}, rs1: {rs1}, imm: {imm}");
                trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                trace!("Reading from address: {:#X}", addr);
                let value = cpu.read(addr, Sizes::Byte, AccessType::Readable)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lh { rd, rs1, imm } => {
                trace!("LH: rd: {rd}, rs1: {rs1}, imm: {imm}");
                trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                trace!("Reading from address: {:#X}", addr);
                let value = cpu.read(addr, Sizes::HalfWord, AccessType::Readable)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lw { rd, rs1, imm } => {
                trace!("LW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                trace!("Reading from address: {:#X}", addr);
                let value = cpu.read(addr, Sizes::Word, AccessType::Readable)?;
                trace!("Read value: {:#X}", value);
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lbu { rd, rs1, imm } => {
                trace!("LBU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );

                let addr = self.xregs[rs1 as usize].wrapping_add(imm);

                trace!("Reading from address: {:#X}", addr);

                // the read value must be zero-extended to 32 bits
                let value = cpu.read(addr, Sizes::Byte, AccessType::Readable)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Lhu { rd, rs1, imm } => {
                trace!("LHU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );

                let addr = self.xregs[rs1 as usize].wrapping_add(imm);

                trace!("Reading from address: {:#X}", addr);

                // the read value must be zero-extended to 32 bits
                let value = cpu.read(addr, Sizes::HalfWord, AccessType::Readable)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lwu { rd, rs1, imm } => {
                trace!("LWU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );

                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;

                trace!("Reading from address: {:#X}", addr);

                let value = cpu.read(addr, Sizes::Word, AccessType::Readable)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Sb { rs1, rs2, imm } => {
                trace!("SB: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                trace!(
                    "value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rs1 as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                trace!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize] as u8 as u32;
                trace!("Writing value: {:#X}", value);
                cpu.write(addr, value, Sizes::Byte, AccessType::Writable)?;
            }
            InstructionDecoded::Sh { rs1, rs2, imm } => {
                trace!("SH: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                trace!(
                    "value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rs1 as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                trace!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize] as u16 as u32;
                trace!("Writing value: {:#X}", value);
                cpu.write(addr, value, Sizes::HalfWord, AccessType::Writable)?;
            }
            InstructionDecoded::Sw { rs1, rs2, imm } => {
                trace!("SW: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                trace!(
                    "value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rs1 as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                trace!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize];
                trace!("Writing value: {:#X}", value);
                cpu.write(addr, value, Sizes::Word, AccessType::Writable)?;
            }
            InstructionDecoded::Fld { rd, rs1, imm } => {
                trace!("FLD: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                let value = cpu.read(addr, Sizes::Word, AccessType::Readable)?;
                self.fregs[rd as usize] = value as f64;
            }
            InstructionDecoded::Fsd { rs1, rs2, imm } => {
                trace!("FSD: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                let value = self.fregs[rs2 as usize] as u32;
                cpu.write(addr, value, Sizes::Word, AccessType::Writable)?;
            }
            InstructionDecoded::ECall => todo!(),
            InstructionDecoded::EBreak => {
                trace!("EBREAK");
                return Err(Exception::Breakpoint);
            }
            InstructionDecoded::SRet => todo!(),
            InstructionDecoded::MRet => {
				trace!("MRet");
				let mepc = cpu.read_csr(MEPC);
				cpu.set_pc(mepc);
				let status = cpu.read_csr(MSTATUS);
				let mpie = (status >> 7) & 1;
				let mpp = (status >> 11) & 0x3;
				let p = match mpp {
					0 => Privilege::User,
					1 => Privilege::Supervisor,
					3 => Privilege::Machine,
					_ => panic!("Unknown privilege uncoding")
				};
				let mprv = match p {
					Privilege::Machine => (status >> 17) & 1,
					_ => 0
				};
				// Override MIE[3] with MPIE[7], set MPIE[7] to 1, set MPP[12:11] to 0
				// and override MPRV[17]
				let new_status = (status & !0x21888) | (mprv << 17) | (mpie << 3) | (1 << 7);
				cpu.write_csr(MSTATUS, new_status);
				cpu.set_privilege(p);
			}
            InstructionDecoded::SFenceVma => {
                trace!("SFENCE.VMA");
                // do nothing
            }
            InstructionDecoded::CsrRw { rd, rs1, imm } => if rd == 0 {
				trace!("CSRRW: rd: {rd}, rs1: {rs1}, imm: {imm}");
				// The CSRRW (Atomic Read/Write CSR) instruction atomically swaps values in the CSRs and integer registers.
				// CSRRW reads the old value of the CSR, zero-extends the value to XLEN bits, then writes it to integer register rd.
				// The initial value in rs1 is written to the CSR.
				// If rd=x0, then the instruction shall not read the CSR and
				// shall not cause any of the side effects that might occur on a CSR read.
				let imm = imm as CsrAddress;
				let data = cpu.read_csr(imm);
				let tmp = self.xregs[rs1 as usize];
				self.xregs[rd as usize] = data;
				cpu.write_csr(imm, tmp);
			} else {
				info!("CSRRW: rd: {rd}, rs1: {rs1}, imm: {imm:#X}");
				warn!("CSRRW: rd is zero, ignoring");
			}
            InstructionDecoded::CsrRs { rd, rs1, imm } => {
				trace!("CSRRS: rd: {rd}, rs1: {rs1}, imm: {imm}");
				// The CSRRS (Atomic Read and Set Bits in CSR) instruction reads the value of the CSR,
				let imm = imm as CsrAddress;
				let old_value = cpu.read_csr(imm);
				// The initial value in integer register rs1 is treated as a bit mask that specifies bit positions to be set in the CSR.
				let mask = self.xregs[rs1 as usize];
				// Any bit that is high in rs1 will cause the corresponding bit to be set in the CSR, if that CSR bit is writable.
				let new_value = old_value | mask;
				// Other bits in the CSR are not explicitly written.
				cpu.write_csr(imm, new_value);
				// zero-extends the value to XLEN bits, and writes it to integer register rd.
				self.xregs[rd as usize] = old_value;
			}
            InstructionDecoded::CsrRc { rd, rs1, imm } => {
				trace!("CSRRC: rd: {rd}, rs1: {rs1}, imm: {imm}");
				todo!()
			}
            InstructionDecoded::CsrRwi { rd, rs1, imm } => {
				trace!("CSRRWI: rd: {rd}, rs1: {rs1}, imm: {imm}");
				todo!()
			}
            InstructionDecoded::CsrRsi { rd, rs1, imm } => {
				trace!("CSRRSI: rd: {rd}, rs1: {rs1}, imm: {imm}");
				todo!()
			}
            InstructionDecoded::CsrRci { rd, rs1, imm } => {
				trace!("CSRRCI: rd: {rd}, rs1: {rs1}, imm: {imm}");
				todo!()
			}
            InstructionDecoded::Slt { rd, rs1, rs2 } => {
				trace!("SLT: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
				let rs1 = self.xregs[rs1 as usize] as i32;
				let rs2 = self.xregs[rs2 as usize] as i32;
				self.xregs[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
			}
            InstructionDecoded::Slti { rd, rs1, imm } => {
				trace!("SLTI: rd: {rd}, rs1: {rs1}, imm: {imm}");
				let rs1 = self.xregs[rs1 as usize] as i32;
				self.xregs[rd as usize] = if rs1 < imm as i32 { 1 } else { 0 };
			}
            InstructionDecoded::Sltiu { rd, rs1, imm } => {
				trace!("SLTIU: rd: {rd}, rs1: {rs1}, imm: {imm}");
				let rs1 = self.xregs[rs1 as usize];
				self.xregs[rd as usize] = if rs1 < imm { 1 } else { 0 };
			}
            InstructionDecoded::Sltu { rd, rs1, rs2 } => {
				trace!("SLTU: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
				let rs1 = self.xregs[rs1 as usize];
				let rs2 = self.xregs[rs2 as usize];
				self.xregs[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
			}
            // FENCE and FENCE.I are used to order device I/O and memory accesses which we don't need to implement
            // so we just treat them as no-ops
            InstructionDecoded::Fence { .. } => {
                trace!("FENCE");
                // do nothing
            }
            InstructionDecoded::FenceI { .. } => {
                trace!("FENCE.I");
                // do nothing
            }

            // RV32F
            InstructionDecoded::Flw { .. } => todo!(),
            InstructionDecoded::Fsw { .. } => todo!(),
            InstructionDecoded::FmaddS { .. } => todo!(),
            InstructionDecoded::FmsubS { .. } => todo!(),
            InstructionDecoded::FnmaddS { .. } => todo!(),
            InstructionDecoded::FnmsubS { .. } => todo!(),
            InstructionDecoded::FaddS { .. } => todo!(),
            InstructionDecoded::FsubS { .. } => todo!(),
            InstructionDecoded::FmulS { .. } => todo!(),
            InstructionDecoded::FdivS { .. } => todo!(),
            InstructionDecoded::FsqrtS { .. } => todo!(),
            InstructionDecoded::FsgnjS { .. } => todo!(),
            InstructionDecoded::FsgnjnS { .. } => todo!(),
            InstructionDecoded::FsgnjxS { .. } => todo!(),
            InstructionDecoded::FminS { .. } => todo!(),
            InstructionDecoded::FmaxS { .. } => todo!(),
            InstructionDecoded::FcvtSW { ..  } => todo!(),
            InstructionDecoded::FcvtSWU { .. } => todo!(),
            InstructionDecoded::FcvtWS { .. } => todo!(),
            InstructionDecoded::FcvtWUS { .. } => todo!(),
            InstructionDecoded::FmvXW { .. } => todo!(),
            InstructionDecoded::FmvWX { .. } => todo!(),
            InstructionDecoded::FeqS { .. } => todo!(),
            InstructionDecoded::FltS { .. } =>todo!(),
            InstructionDecoded::FleS { .. } => todo!(),
			InstructionDecoded::FClassS { .. } => todo!(),

            // RV32M
            InstructionDecoded::Mul { rd, rs1, rs2 } => {
                trace!("MUL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_mul(rs2) as XRegisterSize;
            }
            InstructionDecoded::Mulh { rd, rs1, rs2 } => {
                trace!("MULH: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                // multiply the two 32-bit signed integers and return the upper 32 bits of the result
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = ((rs1 as i64 * rs2 as i64) >> 32) as XRegisterSize;
            }
            InstructionDecoded::Mulsu { .. } => todo!(),
            InstructionDecoded::Mulu { rd, rs1, rs2 } => {
                trace!("MULU: rd = {rd}, rs1 = {rs1}, rs2 = {rs2}");
                trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = zero_extend(rs1.wrapping_mul(rs2) as XRegisterSize);
            }
            InstructionDecoded::Div { rd, rs1, rs2 } => {
                trace!("DIV: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_div(rs2) as XRegisterSize;
            }
            InstructionDecoded::Divu { rd, rs1, rs2 } => {
                trace!("DIVU: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = zero_extend(rs1.wrapping_div(rs2) as XRegisterSize);
            }
            InstructionDecoded::Rem { rd, rs1, rs2 } => {
                trace!("REM: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_rem(rs2) as XRegisterSize;
            }
            InstructionDecoded::Remu { rd, rs1, rs2 } => {
                trace!("REMU: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = zero_extend(rs1.wrapping_rem(rs2) as XRegisterSize);
            }

            // RV32A
			InstructionDecoded::AmoswapW { rd, rs1, rs2, aq, rl } => {
				trace!("AMOSWAP.W: rd: {rd}, rs1: {rs1}, rs2: {rs2}, aq: {aq}, rl: {rl}");
				// Description
				// atomically load a 32-bit signed data value from the address in rs1, place the value into register rd,
				// swap the loaded value and the original 32-bit signed value in rs2, then store the result back to the address in rs1.

				let addr = self.xregs[rs1 as usize];
				let value = self.xregs[rs2 as usize];
				let old_value = cpu.read(addr, Sizes::Word, AccessType::Readable)?;
				cpu.write(addr, value, Sizes::Word, AccessType::Writable)?;
				self.xregs[rd as usize] = old_value;
			}
            InstructionDecoded::AmoaddW { .. } => todo!(),
            InstructionDecoded::AmoandW { .. } => todo!(),
            InstructionDecoded::AmoorW { .. } => todo!(),
            InstructionDecoded::AmoxorW { .. } => todo!(),
            InstructionDecoded::AmomaxW { .. } => todo!(),
            InstructionDecoded::AmominW { .. } => todo!(),
            InstructionDecoded::LrW { .. } => todo!(),
            InstructionDecoded::ScW { .. } => todo!(),

            // RV32C
            InstructionDecoded::CAddi4Spn { .. } => todo!(),
            InstructionDecoded::CNop { .. } => todo!(),
            InstructionDecoded::CSlli { .. } => todo!(),
            InstructionDecoded::FcvtSD { .. } => todo!(),
            InstructionDecoded::FcvtDS { .. } => todo!(),
            InstructionDecoded::FaddD { .. } => todo!(),
            InstructionDecoded::FsubD { .. } => todo!(),
            InstructionDecoded::FmulD { .. } => todo!(),
            InstructionDecoded::FdivD { .. } => todo!(),
            InstructionDecoded::FsqrtD { .. } => todo!(),
            InstructionDecoded::FsgnjD { .. } => todo!(),
            InstructionDecoded::FsgnjnD { .. } => todo!(),
            InstructionDecoded::FsgnjxD { .. } => todo!(),
            InstructionDecoded::FminD { .. } => todo!(),
            InstructionDecoded::FmaxD { .. } => todo!(),
            InstructionDecoded::FeqD { .. } => todo!(),
            InstructionDecoded::FltD { .. } => todo!(),
            InstructionDecoded::FleD { .. } => todo!(),
            InstructionDecoded::FClassD { .. } => todo!(),

			// TODO: figure out where these go
			InstructionDecoded::FcvtWD { .. } => todo!(),
			InstructionDecoded::FcvtWUD { .. } => todo!(),
			InstructionDecoded::FcvtDW { .. } => todo!(),
			InstructionDecoded::FcvtDWU { .. } => todo!(),
			InstructionDecoded::FmaddD { .. } => todo!(),
			InstructionDecoded::FmsubD { .. } => todo!(),
			InstructionDecoded::FnmaddD { .. } => todo!(),
			InstructionDecoded::FnmsubD { .. } => todo!(),
        }

        Ok(())
    }

    pub fn dump_registers(&self, cpu: &impl Cpu) {
        const RVABI: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];
        info!("{:-^80}", "registers");
        info!("{0:3}({0:^4}) = {1:<#18x}", "pc", cpu.get_pc());
        for i in (0..32).step_by(4) {
            let (i0, i1, i2, i3) = (
                format!("x{}", i),
                format!("x{}", i + 1),
                format!("x{}", i + 2),
                format!("x{}", i + 3),
            );
            let line = format!(
                "{:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x} {:3}({:^4}) = {:<#18x}",
                i0, RVABI[i], self.xregs[i],
                i1, RVABI[i + 1], self.xregs[i + 1],
                i2, RVABI[i + 2], self.xregs[i + 2],
                i3, RVABI[i + 3], self.xregs[i + 3],
            );
            info!("{}", line);
        }
		info!("{:-^80}", "");
    }
}
