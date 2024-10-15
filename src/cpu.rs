use crate::{
    bit_ops::*,
    bus::{Bus, Device, VirtualDevice},
    csr::{
        State, MEIP_BIT, MEPC, MIE, MIP, MSIP_BIT, MSTATUS, MSTATUS_MIE, MSTATUS_MPP, MSTATUS_MPRV,
        MTIP_BIT, SATP, SEIP_BIT, SEPC, SSIP_BIT, SSTATUS, STIP_BIT, XSTATUS_SIE,
    },
    devices::{
        clint::Clint,
        plic::Plic,
        uart::{Uart, UART_IRQ},
        viritio::{Virtio, VIRTIO_IRQ},
    },
    interrupt::Interrupt,
    log_debug, log_error, log_info, log_trace,
    memory::{
        dram::{Sizes, DRAM_BASE, DRAM_SIZE},
        virtual_memory::MemorySize,
    },
    registers::{FRegisters, XRegisterSize, XRegisters},
    trap::{Exception, Trap},
};
use riscv_decoder::{
    decoded_inst::InstructionDecoded, decoder::try_decode, instructions::compressed::is_compressed,
};

pub const POINTER_TO_DTB: u32 = 0x1020;

/// The page size (4 KiB) for the virtual memory system.
const PAGE_SIZE: u32 = 4096;

/// The privilege mode of the CPU.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

/// Access type that is used in the virtual address translation process. It decides which exception
/// should raises (InstructionPageFault, LoadPageFault or StorePageFault).
#[derive(Debug, PartialEq, PartialOrd)]
pub enum AccessType {
    /// Raises the exception InstructionPageFault. It is used for an instruction fetch.
    Instruction,
    /// Raises the exception LoadPageFault.
    Load,
    /// Raises the exception StorePageFault.
    Store,
}

// 32 bit RiscV CPU architecture
pub struct Cpu {
    pub xregs: XRegisters,
    pub fregs: FRegisters,

    /// program counter
    pc: XRegisterSize,

    /// The current privilege mode.
    mode: Mode,

    /// little endian memory / stack array
    bus: Bus,

    /// SV39 paging flag.
    enable_paging: bool,
    /// Physical page number (PPN) × PAGE_SIZE (4096).
    page_table: u32,

    /// Csr controller
    state: State,
}

impl Cpu {
    pub fn new() -> Self {
        log_trace!("Initializing CPU...");
        let mut registers = XRegisters::new();
        registers[2] = DRAM_BASE + DRAM_SIZE; // stack pointer
        registers[11] = POINTER_TO_DTB; // pointer to device tree blob

        let cpu = Self {
            xregs: registers,
            fregs: FRegisters::new(),
            pc: DRAM_BASE,
            mode: Mode::Machine,
            bus: Bus::new(),
            state: State::new(),
            enable_paging: false,
            page_table: 0,
        };
        log_trace!("CPU initialized");
        cpu
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    /// Start executing the emulator with limited range of program. This method is for test.
    /// No interrupts happen.
    pub fn test_start(&mut self, start: u32, end: u32) {
        println!("----- test start -----");
        let mut count = 0;
        loop {
            count += 1;
            if self.pc < start || end <= self.pc {
                return;
            }
            // This is a workaround for unit tests to finish the execution.
            if count > 1000 {
                return;
            }

            let inst = self.fetch().expect("failed to fetch an instruction");
            match self.execute(inst) {
                Ok(_) => {
                    println!("pc: {:#x}", self.pc.wrapping_sub(4));
                    Trap::Requested
                }
                Err(exception) => {
                    println!("pc: {:#x}, exception: {:?}", self.pc, exception);
                    exception.take_trap(self)
                }
            };
        }
    }

    pub fn bus_read(&self, addr: u32, size: Sizes) -> Result<u32, Exception> {
        self.bus.read(addr, size)
    }

    pub fn bus_write(&mut self, addr: u32, value: u32, size: Sizes) -> Result<(), Exception> {
        self.bus.write(addr, value, size)
    }

    pub fn get_device<T>(&self) -> Option<&T>
    where
        T: Device + 'static,
    {
        self.bus.get_device()
    }

    pub fn get_device_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Device + 'static,
    {
        self.bus.get_device_mut()
    }

    pub fn add_device(&mut self, device: VirtualDevice) {
        self.bus.add_device(device);
    }

    pub fn get_mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn get_pc(&self) -> XRegisterSize {
        self.pc
    }
    pub fn set_pc(&mut self, pc: XRegisterSize) {
        self.pc = pc;
    }

    pub fn read_csr(&self, addr: u16) -> u32 {
        self.state.read(addr)
    }
    pub fn write_csr(&mut self, addr: u16, value: u32) {
        self.state.write(addr, value)
    }

    pub fn get_register(&self, register: XRegisterSize) -> Result<&XRegisterSize, String> {
        match register {
            0..=31 => Ok(&self.xregs[register as usize]),
            32 => Ok(&self.pc),
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
            0..=31 => Ok(&mut self.xregs[register as usize]),
            32 => Ok(&mut self.pc),
            _ => Err(format!(
                "The register '{register}' is not an addressable register?"
            )),
        }
    }

    /// Read `size`-bit data from the system bus with the translation a virtual address to a physical address
    /// if it is enabled.
    fn read(&mut self, v_addr: MemorySize, size: Sizes) -> Result<MemorySize, Exception> {
        let previous_mode = self.mode;

        // 3.1.6.3 Memory Privilege in mstatus Register
        // "When MPRV=1, load and store memory addresses are translated and protected, and
        // endianness is applied, as though the current privilege mode were set to MPP."
        if self.state.read_mstatus(MSTATUS_MPRV) == 1 {
            self.mode = match self.state.read_mstatus(MSTATUS_MPP) {
                0b00 => Mode::User,
                0b01 => Mode::Supervisor,
                0b11 => Mode::Machine,
                _ => panic!("Invalid MPP value"),
            };
        }

        let p_addr = self.translate(v_addr, AccessType::Load)?;
        let result = self.bus_read(p_addr, size);

        if self.state.read_mstatus(MSTATUS_MPRV) == 1 {
            self.mode = previous_mode;
        }

        result
    }

    /// Update the physical page number (PPN) and the addressing mode.
    fn update_paging(&mut self) {
        log_info!("Update the paging information");

        // Read the physical page number (PPN) of the root page table, i.e., its
        // supervisor physical address divided by 4 KiB.
        self.page_table = self.state.read_bits(SATP, ..22) * PAGE_SIZE;
        log_info!("Page table: {:#X}", self.page_table);

        // Read the MODE field, which selects the current address-translation scheme.
        let mode = self.state.read_bit(SATP, 31);
        log_info!("Mode: {:#X}", mode);

        // Enable the SV39 paging if the value of the mode field is 8.
        self.enable_paging = mode == 8;
        log_info!("Paging is enabled: {}", self.enable_paging);
    }

    /// Check interrupt flags for all devices that can interrupt.
    pub fn check_pending_interrupt(&mut self) -> Option<Interrupt> {
        // global interrupt: PLIC (Platform Local Interrupt Controller) dispatches global
        //                   interrupts to multiple harts.
        // local interrupt: CLINT (Core Local Interrupter) dispatches local interrupts to a hart
        //                  which directly connected to CLINT.

        // 3.1.6.1 Privilege and Global Interrupt-Enable Stack in mstatus register
        // "When a hart is executing in privilege mode x, interrupts are globally enabled when
        // xIE=1 and globally disabled when xIE=0."
        match self.mode {
            Mode::Machine => {
                // Check if the MIE bit is enabled.
                if self.state.read_mstatus(MSTATUS_MIE) == 0 {
                    return None;
                }
            }
            Mode::Supervisor => {
                // Check if the SIE bit is enabled.
                if self.state.read_sstatus(XSTATUS_SIE) == 0 {
                    return None;
                }
            }
            _ => {}
        }

        // TODO: Take interrupts based on priorities.

        // Check external interrupt for uart and virtio.
        let irq;
        if self
            .get_device::<Uart>()
            .expect("UART is not found")
            .is_interrupting()
        {
            log_info!("UART interrupt");
            irq = UART_IRQ;
        } else if self
            .get_device_mut::<Virtio>()
            .expect("Virtio is not found")
            .is_interrupting()
        {
            log_info!("Virtio interrupt");
            // An interrupt is raised after a disk access is done.
            Virtio::disk_access(self).expect("failed to access the disk");
            irq = VIRTIO_IRQ;
        } else {
            irq = 0;
        }

        if irq != 0 {
            // TODO: assume that hart is 0
            // TODO: write a value to MCLAIM if the mode is machine
            let plic = self.get_device_mut::<Plic>().expect("PLIC is not found");
            plic.update_pending(irq);
            self.write_csr(MIP, self.read_csr(MIP) | SEIP_BIT);
        }

        // 3.1.9 Machine Interrupt Registers (mip and mie)
        // "An interrupt i will be taken if bit i is set in both mip and mie, and if interrupts are
        // globally enabled. By default, M-mode interrupts are globally enabled if the hart’s
        // current privilege mode is less than M, or if the current privilege mode is M and the MIE
        // bit in the mstatus register is set. If bit i in mideleg is set, however, interrupts are
        // considered to be globally enabled if the hart’s current privilege mode equals the
        // delegated privilege mode (S or U) and that mode’s interrupt enable bit (SIE or UIE in
        // mstatus) is set, or if the current privilege mode is less than the delegated privilege
        // mode."
        let pending = self.read_csr(MIE) & self.read_csr(MIP);

        if (pending & MEIP_BIT) != 0 {
            self.write_csr(MIP, self.read_csr(MIP) & !MEIP_BIT);
            return Some(Interrupt::MachineExternalInterrupt);
        }
        if (pending & MSIP_BIT) != 0 {
            self.write_csr(MIP, self.read_csr(MIP) & !MSIP_BIT);
            return Some(Interrupt::MachineSoftwareInterrupt);
        }
        if (pending & MTIP_BIT) != 0 {
            self.write_csr(MIP, self.read_csr(MIP) & !MTIP_BIT);
            return Some(Interrupt::MachineTimerInterrupt);
        }
        if (pending & SEIP_BIT) != 0 {
            self.write_csr(MIP, self.read_csr(MIP) & !SEIP_BIT);
            return Some(Interrupt::SupervisorExternalInterrupt);
        }
        if (pending & SSIP_BIT) != 0 {
            self.write_csr(MIP, self.read_csr(MIP) & !SSIP_BIT);
            return Some(Interrupt::SupervisorSoftwareInterrupt);
        }
        if (pending & STIP_BIT) != 0 {
            self.write_csr(MIP, self.read_csr(MIP) & !STIP_BIT);
            return Some(Interrupt::SupervisorTimerInterrupt);
        }

        None
    }

    /// Write `size`-bit data to the system bus with the translation a virtual address to a physical
    /// address if it is enabled.
    fn write(
        &mut self,
        v_addr: MemorySize,
        value: MemorySize,
        size: Sizes,
    ) -> Result<(), Exception> {
        let previous_mode = self.mode;

        // 3.1.6.3 Memory Privilege in mstatus Register
        // "When MPRV=1, load and store memory addresses are translated and protected, and
        // endianness is applied, as though the current privilege mode were set to MPP."
        if self.state.read_mstatus(MSTATUS_MPRV) == 1 {
            self.mode = match self.state.read_mstatus(MSTATUS_MPP) {
                0b00 => Mode::User,
                0b01 => Mode::Supervisor,
                0b11 => Mode::Machine,
                _ => panic!("Invalid MPP value"),
            };
        }

        // "The SC must fail if a write from some other device to the bytes accessed by the LR can
        // be observed to occur between the LR and SC."
        /*if self.reservation_set.contains(&v_addr) {
            self.reservation_set.retain(|&x| x != v_addr);
        }*/

        let p_addr = self.translate(v_addr, AccessType::Store)?;
        let result = self.bus_write(p_addr, value, size);

        if self.state.read_mstatus(MSTATUS_MPRV) == 1 {
            self.mode = previous_mode;
        }

        result
    }

    fn translate(
        &mut self,
        addr: MemorySize,
        access_type: AccessType,
    ) -> Result<MemorySize, Exception> {
        bitfield::bitfield! {
            struct PTE(u32);
            impl Debug;
            u32;
            v, set_v: 0, 0;
            r, set_r: 1, 1;
            w, set_w: 2, 2;
            x, set_x: 3, 3;
            u, set_u: 4, 4;
            a, set_a: 6, 6;
            d, set_d: 7, 7;
            ppn, set_ppn: 31, 10;
        }
        impl PTE {
            fn new(value: u32) -> Self {
                PTE(value)
            }
        }

        if !self.enable_paging || self.mode == Mode::Machine {
            return Ok(addr);
        }

        // 4.3.2 Virtual Address Translation Process
        // (The RISC-V Instruction Set Manual Volume II-Privileged Architecture_20190608)
        // A virtual address va is translated into a physical address pa as follows:
        let levels = 2;
        let vpn = [
            (addr >> 12) & 0x1ff,
            (addr >> 21) & 0x1ff,
            (addr >> 30) & 0x1ff,
        ];

        // 1. Let a be satp.ppn × PAGESIZE, and let i = LEVELS − 1. (For Sv32, PAGESIZE=212
        //    and LEVELS=2.)
        let mut a = self.page_table;
        let mut i: i32 = levels - 1;
        let mut pte;
        const PTE_SIZE: u32 = 4;
        loop {
            // 2. Let pte be the value of the PTE at address a+va.vpn[i]×PTESIZE. (For Sv32,
            //    PTESIZE=4.) If accessing pte violates a PMA or PMP check, raise an access
            //    exception corresponding to the original access type.
            pte = PTE::new(self.bus_read(a + vpn[i as usize] * PTE_SIZE, Sizes::Word)?);

            // 3. If pte.v = 0, or if pte.r = 0 and pte.w = 1, stop and raise a page-fault
            //    exception corresponding to the original access type.
            if pte.v() == 0 || (pte.r() == 0 && pte.w() == 1) {
                return match access_type {
                    AccessType::Instruction => Err(Exception::InstructionPageFault(addr)),
                    AccessType::Load => Err(Exception::LoadPageFault(addr)),
                    AccessType::Store => Err(Exception::StorePageFault(addr)),
                };
            }

            // 4. Otherwise, the PTE is valid. If pte.r = 1 or pte.x = 1, go to step 5.
            //    Otherwise, this PTE is a pointer 0b00000000000000000000000010001000; to the next level of the page table.
            //    Let i = i − 1. If i < 0, stop and raise a page-fault exception
            //    corresponding to the original access type. Otherwise,
            //    let a = pte.ppn × PAGESIZE and go to step 2.
            if pte.r() == 1 || pte.x() == 1 {
                break;
            }
            i -= 1;
            a = pte.ppn() * PAGE_SIZE;
            if i < 0 {
                match access_type {
                    AccessType::Instruction => return Err(Exception::InstructionPageFault(addr)),
                    AccessType::Load => return Err(Exception::LoadPageFault(addr)),
                    AccessType::Store => return Err(Exception::StorePageFault(addr)),
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
        let ppn = [
            (pte.ppn() >> 10) & 0x1ff,
            (pte.ppn() >> 19) & 0x1ff,
            (pte.ppn() >> 28) & 0x03ff_ffff,
        ];
        if i > 0 {
            for j in (0..i).rev() {
                if ppn[j as usize] != 0 {
                    // A misaligned superpage.
                    match access_type {
                        AccessType::Instruction => {
                            return Err(Exception::InstructionPageFault(addr))
                        }
                        AccessType::Load => return Err(Exception::LoadPageFault(addr)),
                        AccessType::Store => return Err(Exception::StorePageFault(addr)),
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
        if pte.a() == 0 || (access_type == AccessType::Store && pte.d() == 0) {
            // Set pte.a to 1 and, if the memory access is a store, also set pte.d to 1.
            pte.set_a(1);
            if access_type == AccessType::Store {
                pte.set_d(1);
            }

            // TODO: PMA or PMP check.

            // Update the value of address satp.ppn × PAGESIZE + va.vpn[i] × PTESIZE with new pte
            // value.
            // TODO: If this is enabled, running xv6 fails.
            //self.bus_write(self.page_table + vpn[i as usize] * 8, pte, 64)?;
        }

        // 8. The translation is successful. The translated physical address is given as
        //    follows:
        //    • pa.pgoff = va.pgoff.
        //    • If i > 0, then this is a superpage translation and pa.ppn[i−1:0] =
        //    va.vpn[i−1:0].
        //    • pa.ppn[LEVELS−1:i] = pte.ppn[LEVELS−1:i].
        let offset = addr & 0xfff;
        match i {
            0 => {
                let ppn = (pte.ppn() >> 10) & 0x3ff_ffff; // TODO: Check if this (0x3ff_ffff) is correct
                Ok((ppn << 12) | offset)
            }
            1 => {
                // Superpage translation. A superpage is a memory page of larger size than an
                // ordinary page (4 KiB). It reduces TLB misses and improves performance.
                Ok((ppn[2] << 30) | (ppn[1] << 21) | (vpn[0] << 12) | offset)
            }
            2 => {
                // Superpage translation. A superpage is a memory page of larger size than an
                // ordinary page (4 KiB). It reduces TLB misses and improves performance.
                Ok((ppn[2] << 30) | (vpn[1] << 21) | (vpn[0] << 12) | offset)
            }
            _ => match access_type {
                AccessType::Instruction => Err(Exception::InstructionPageFault(addr)),
                AccessType::Load => Err(Exception::LoadPageFault(addr)),
                AccessType::Store => Err(Exception::StorePageFault(addr)),
            },
        }
    }

    pub fn fetch(&mut self) -> Result<InstructionDecoded, Exception> {
        let p_pc = self.translate(self.pc, AccessType::Instruction)?;

        // The result of the read method can be `Exception::LoadAccessFault`. In fetch(), an error
        // should be `Exception::InstructionAccessFault`.
        let inst = match self.bus_read(p_pc, Sizes::Word) {
            Ok(value) => value,
            Err(_) => {
                return Err(Exception::InstructionAccessFault);
            }
        };

        if is_compressed(inst) {
            self.pc += 2;
        } else {
            self.pc += 4;
        }

        // decode the instruction (automatically detects if compressed)
        let inst = try_decode(inst).map_err(|e| {
            log_error!("Illegal instruction: {:#X} => {e:?}", inst);
            Exception::IllegalInstruction(inst)
        })?;

        log_debug!("{:#08X}: {}", p_pc, inst);

        Ok(inst)
    }

    pub fn execute(&mut self, inst: InstructionDecoded) -> Result<(), Exception> {
        // x0 must always be zero (irl the circuit is literally hardwired to electriacal equivalent of 0)
        self.xregs[0] = 0;

        match inst {
            InstructionDecoded::Add { rd, rs1, rs2 } => {
                log_trace!("ADD: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                log_trace!("rs1 = {rs1}, rs2 = {rs2}");
                self.xregs[rd as usize] = rs1.wrapping_add(rs2) as XRegisterSize;
            }
            InstructionDecoded::Addi { rd, rs1, imm } => {
                log_trace!("ADDI: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_add(imm as i32) as XRegisterSize;
            }
            InstructionDecoded::Sub { rd, rs1, rs2 } => {
                log_trace!("SUB: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_sub(rs2) as XRegisterSize;
            }
            InstructionDecoded::And { rd, rs1, rs2 } => {
                log_trace!("AND: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 & rs2) as u32;
            }
            InstructionDecoded::Andi { rd, rs1, imm } => {
                log_trace!("ANDI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                self.xregs[rd as usize] = (rs1 & imm as i32) as u32;
            }
            InstructionDecoded::Or { rd, rs1, rs2 } => {
                log_trace!("OR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 | rs2;
            }
            InstructionDecoded::Ori { rd, rs1, imm } => {
                log_trace!("ORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1 | imm;
            }
            InstructionDecoded::Xor { rd, rs1, rs2 } => {
                log_trace!("XOR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 ^ rs2;
            }
            InstructionDecoded::Xori { rd, rs1, imm } => {
                log_trace!("XORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1 ^ imm;
            }
            InstructionDecoded::Sll { rd, rs1, rs2 } => {
                log_trace!("SLL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 << rs2;
            }
            InstructionDecoded::Slli { rd, rs1, imm } => {
                log_trace!("SLLI: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);

                let rs1 = self.xregs[rs1 as usize];

                self.xregs[rd as usize] = rs1 << imm;
            }
            InstructionDecoded::Srl { rd, rs1, rs2 } => {
                log_trace!("SRL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1.wrapping_shr(rs2);
            }
            InstructionDecoded::Srli { rd, rs1, imm } => {
                log_trace!("SRLI: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let rs1 = self.xregs[rs1 as usize] as i32;

                self.xregs[rd as usize] = (rs1 >> imm) as u32;
            }
            InstructionDecoded::Sra { rd, rs1, rs2 } => {
                log_trace!("SRA: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 >> rs2;
            }
            InstructionDecoded::Srai { rd, rs1, imm } => {
                log_trace!("SRAI: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let rs1 = self.xregs[rs1 as usize];

                self.xregs[rd as usize] = rs1 >> imm;
            }
            InstructionDecoded::Lui { rd, imm } => {
                log_trace!("LUI: rd: {rd}, imm: {}", imm << 12);
                self.xregs[rd as usize] = imm << 12;
            }
            InstructionDecoded::AuiPc { rd, imm } => {
                log_trace!("AUIPC: rd: {rd}, imm: {imm}");
                self.xregs[rd as usize] = self.pc.wrapping_add(imm << 12).wrapping_sub(4) as XRegisterSize;
            }
            InstructionDecoded::Jal { rd, imm } => {
                log_trace!("JAL: rd: {rd}, imm: {imm}");
                self.xregs[rd as usize] = self.pc;

                let npc = self.pc.wrapping_add(imm).wrapping_sub(4);
                // npc = crate::bit_ops::zero_extend(npc);

                self.pc = npc;
            }
            InstructionDecoded::Jalr { rd, rs1, imm } => {
                log_trace!(
                    "JALR: rd = {rd}, rs1 = {rs1}, imm = {imm}",
                    imm = imm as i32
                );
                let t = self.pc;
                let target = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                self.pc = target;
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::Beq { rs1, rs2, imm } => {
                log_trace!("BEQ: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                log_trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 == rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    log_trace!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bne { rs1, rs2, imm } => {
                log_trace!("BNE: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                log_trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 != rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    log_trace!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Blt { rs1, rs2, imm } => {
                log_trace!("BLT: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                log_trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 < rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    log_trace!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bge { rs1, rs2, imm } => {
                log_trace!("BGE: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                log_trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 >= rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    log_trace!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bltu { rs1, rs2, imm } => {
                log_trace!("BLTU: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                log_trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 < rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    log_trace!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bgeu { rs1, rs2, imm } => {
                log_trace!("BGEU: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                log_trace!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 >= rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    log_trace!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Lb { rd, rs1, imm } => {
                log_trace!("LB: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                log_trace!("Reading from address: {:#X}", addr);
                let value = self.read(addr, Sizes::Byte)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lh { rd, rs1, imm } => {
                log_trace!("LH: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                log_trace!("Reading from address: {:#X}", addr);
                let value = self.read(addr, Sizes::HalfWord)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lw { rd, rs1, imm } => {
                log_trace!("LW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                log_trace!("Reading from address: {:#X}", addr);
                let value = self.read(addr, Sizes::Word)?;
                log_trace!("Read value: {:#X}", value);
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lbu { rd, rs1, imm } => {
                log_trace!("LBU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );

                let addr = self.xregs[rs1 as usize].wrapping_add(imm);

                log_trace!("Reading from address: {:#X}", addr);

                // the read value must be zero-extended to 32 bits
                let value = self.read(addr, Sizes::Byte)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Lhu { rd, rs1, imm } => {
                log_trace!("LHU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );

                let addr = self.xregs[rs1 as usize].wrapping_add(imm);

                log_trace!("Reading from address: {:#X}", addr);

                // the read value must be zero-extended to 32 bits
                let value = self.read(addr, Sizes::HalfWord)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Lwu { rd, rs1, imm } => {
                log_trace!("LWU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );

                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;

                log_trace!("Reading from address: {:#X}", addr);

                let value = self.read(addr, Sizes::Word)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Sb { rs1, rs2, imm } => {
                log_trace!("SB: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                log_trace!(
                    "value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rs1 as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                log_trace!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize] as u8 as u32;
                log_trace!("Writing value: {:#X}", value);
                self.write(addr, value, Sizes::Byte)?;
            }
            InstructionDecoded::Sh { rs1, rs2, imm } => {
                log_trace!("SH: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                log_trace!(
                    "value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rs1 as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                log_trace!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize] as u16 as u32;
                log_trace!("Writing value: {:#X}", value);
                self.write(addr, value, Sizes::HalfWord)?;
            }
            InstructionDecoded::Sw { rs1, rs2, imm } => {
                log_trace!("SW: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                log_trace!(
                    "value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rs1 as usize],
                    self.xregs[rs1 as usize]
                );
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                log_trace!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize];
                log_trace!("Writing value: {:#X}", value);
                self.write(addr, value, Sizes::Word)?;
            }
            InstructionDecoded::ECall => match self.mode {
                Mode::User => {
                    log_trace!("ECall => User mode");
                    return Err(Exception::EnvironmentCallFromUMode);
                }
                Mode::Supervisor => {
                    log_trace!("ECall => Supervisor mode");
                    return Err(Exception::EnvironmentCallFromSMode);
                }
                Mode::Machine => {
                    log_trace!("ECall => Machine mode");
                    return Err(Exception::EnvironmentCallFromMMode);
                }
            },
            InstructionDecoded::EBreak => {
                log_trace!("EBREAK");
                // ebreak
                // Makes a request of the debugger bu raising a Breakpoint
                // exception.
                return Err(Exception::Breakpoint);
            }
            InstructionDecoded::SRet => {
                log_trace!("SRET");
                // The SRET instruction returns from a supervisor-mode exception
                // handler. It does the following operations:
                // - Sets the pc to CSRs[sepc].
                // - Sets the privilege mode to CSRs[sstatus].SPP.
                // - Sets CSRs[sstatus].SIE to CSRs[sstatus].SPIE.
                // - Sets CSRs[sstatus].SPIE to 1.
                // - Sets CSRs[sstatus].SPP to 0.
                self.pc = self.read_csr(SEPC);
                // When the SRET instruction is executed to return from the trap
                // handler, the privilege level is set to user mode if the SPP
                // bit is 0, or supervisor mode if the SPP bit is 1. The SPP bit
                // is the 8th of the SSTATUS csr.
                self.mode = match (self.read_csr(SSTATUS) >> 8) & 1 {
                    1 => Mode::Supervisor,
                    _ => Mode::User,
                };
                // The SPIE bit is the 5th and the SIE bit is the 1st of the
                // SSTATUS csr.
                self.write_csr(
                    SSTATUS,
                    if ((self.read_csr(SSTATUS) >> 5) & 1) == 1 {
                        self.read_csr(SSTATUS) | (1 << 1)
                    } else {
                        self.read_csr(SSTATUS) & !(1 << 1)
                    },
                );
                self.write_csr(SSTATUS, self.read_csr(SSTATUS) | (1 << 5));
                self.write_csr(SSTATUS, self.read_csr(SSTATUS) & !(1 << 8));
            }
            InstructionDecoded::MRet => {
                log_trace!("MRET");
                // The MRET instruction returns from a machine-mode exception
                // handler. It does the following operations:
                // - Sets the pc to CSRs[mepc].
                // - Sets the privilege mode to CSRs[mstatus].MPP.
                // - Sets CSRs[mstatus].MIE to CSRs[mstatus].MPIE.
                // - Sets CSRs[mstatus].MPIE to 1.
                // - Sets CSRs[mstatus].MPP to 0.
                self.pc = self.read_csr(MEPC).wrapping_sub(4);
                log_info!("MRET: pc = {:#X}", self.pc);
                // MPP is two bits wide at [11..12] of the MSTATUS csr.
                self.mode = match get_bits(self.read_csr(MSTATUS), 2, 11) {
                    2 => {
                        log_debug!("MRET => Machine mode");
                        Mode::Machine
                    }
                    1 => {
                        log_debug!("MRET => Supervisor mode");
                        Mode::Supervisor
                    }
                    m => {
                        log_debug!("MRET {m} => User mode");
                        Mode::User
                    }
                };
                // The MPIE bit is the 7th and the MIE bit is the 3rd of the
                // MSTATUS csr.
                self.write_csr(
                    MSTATUS,
                    if is_set(self.read_csr(MSTATUS), 7) {
                        self.read_csr(MSTATUS) | set_bit(0, 3)
                    } else {
                        self.read_csr(MSTATUS) & !set_bit(0, 3)
                    },
                );
                self.write_csr(MSTATUS, self.read_csr(MSTATUS) | (1 << 7));
                self.write_csr(MSTATUS, self.read_csr(MSTATUS) & !(0b11 << 11));
            }
            InstructionDecoded::SFenceVma => {
                log_trace!("SFENCE.VMA");
                // do nothing
            }
            InstructionDecoded::CsrRw { rd, rs1, imm } => {
                log_trace!("CSRRW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as u16);
                self.write_csr(imm as u16, self.xregs[rs1 as usize]);
                self.xregs[rd as usize] = t;

                if imm as u16 == SATP {
                    self.update_paging();
                }
            }
            InstructionDecoded::CsrRs { rd, rs1, imm } => {
                log_trace!("CSRRS: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let t = self.read_csr(imm as u16);
                self.write_csr(imm as u16, t | self.xregs[rs1 as usize]);
                self.xregs[rd as usize] = t;

                if imm as u16 == SATP {
                    self.update_paging();
                }
            }
            InstructionDecoded::CsrRc { rd, rs1, imm } => {
                log_trace!("CSRRC: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let t = self.read_csr(imm as u16);
                self.write_csr(imm as u16, t & (!self.xregs[rs1 as usize]));
                self.xregs[rd as usize] = t;

                if imm as u16 == SATP {
                    self.update_paging();
                }
            }
            InstructionDecoded::CsrRwi { rd, rs1, imm } => {
                log_trace!("CSRRWI: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let zimm = rs1;
                self.xregs[rd as usize] = self.read_csr(imm as u16);
                self.write_csr(imm as u16, zimm);

                if imm as u16 == SATP {
                    self.update_paging();
                }
            }
            InstructionDecoded::CsrRsi { rd, rs1, imm } => {
                log_trace!("CSRRSI: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let zimm = rs1;
                let t = self.read_csr(imm as u16);
                self.write_csr(imm as u16, t | zimm);
                self.xregs[rd as usize] = t;

                if imm as u16 == SATP {
                    self.update_paging();
                }
            }
            InstructionDecoded::CsrRci { rd, rs1, imm } => {
                log_trace!("CSRRCI: rd: {rd}, rs1: {rs1}, imm: {imm}");

                let zimm = rs1;
                let t = self.read_csr(imm as u16);
                self.write_csr(imm as u16, t & (!zimm));
                self.xregs[rd as usize] = t;

                if imm as u16 == SATP {
                    self.update_paging();
                }
            }
            InstructionDecoded::Slt { rd, rs1, rs2 } => {
                log_trace!("SLT: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }
            InstructionDecoded::Slti { rd, rs1, imm } => {
                log_trace!("SLTI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = if rs1 < imm { 1 } else { 0 };
            }
            InstructionDecoded::Sltiu { rd, rs1, imm } => {
                log_trace!("SLTIU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = if rs1 < imm { 1 } else { 0 };
            }
            InstructionDecoded::Sltu { rd, rs1, rs2 } => {
                log_trace!("SLTU: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize]
                );
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }

            // FENCE and FENCE.I are used to order device I/O and memory accesses which we don't need to implement
            // so we just treat them as no-ops
            InstructionDecoded::Fence { .. } => {
                log_trace!("FENCE");
                // do nothing
            }
            InstructionDecoded::FenceI { .. } => {
                log_trace!("FENCE.I");
                // do nothing
            }

            // RV32D
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
            InstructionDecoded::FcvtSW { .. } => todo!(),
            InstructionDecoded::FcvtSWU { .. } => todo!(),
            InstructionDecoded::FcvtWS { .. } => todo!(),
            InstructionDecoded::FcvtWUS { .. } => todo!(),
            InstructionDecoded::FmvXW { .. } => todo!(),
            InstructionDecoded::FmvWX { .. } => todo!(),
            InstructionDecoded::FeqS { .. } => todo!(),
            InstructionDecoded::FltS { .. } => todo!(),
            InstructionDecoded::FleS { .. } => todo!(),
            InstructionDecoded::AmoswapW { .. } => todo!(),
            InstructionDecoded::AmoaddW { .. } => todo!(),
            InstructionDecoded::AmoandW { .. } => todo!(),
            InstructionDecoded::AmoorW { .. } => todo!(),
            InstructionDecoded::AmoxorW { .. } => todo!(),
            InstructionDecoded::AmomaxW { .. } => todo!(),
            InstructionDecoded::AmominW { .. } => todo!(),
            InstructionDecoded::FClassS { .. } => todo!(),

            // RV32M
            InstructionDecoded::Mul { rd, rs1, rs2 } => {
                log_trace!("MUL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 * rs2) as XRegisterSize;
            }
            InstructionDecoded::Mulh { .. } => todo!(),
            InstructionDecoded::Mulsu { .. } => todo!(),
            InstructionDecoded::Mulu { rd, rs1, rs2 } => {
                log_trace!("MULU: rd = {rd}, rs1 = {rs1}, rs2 = {rs2}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1.wrapping_mul(rs2)) as XRegisterSize;
            }
            InstructionDecoded::Div { rd, rs1, rs2 } => {
                log_trace!("DIV: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 / rs2) as XRegisterSize;
            }
            InstructionDecoded::Divu { .. } => todo!(),
            InstructionDecoded::Rem { rd, rs1, rs2 } => {
                log_trace!("REM: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 % rs2) as XRegisterSize;
            }
            InstructionDecoded::Remu { rd, rs1, rs2 } => {
                log_trace!("REMU: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                log_trace!(
                    "value of rd = {}, value of rs1 = {}, value of rs2 = {}",
                    self.xregs[rd as usize],
                    self.xregs[rs1 as usize],
                    self.xregs[rs2 as usize]
                );
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 % rs2) as XRegisterSize;
            }

            // RV32A
            InstructionDecoded::LrW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                log_trace!("LR.W: rd: {rd}, rs1: {rs1}, rs2: {rs2}, rl: {rl}, aq: {aq}");
                let addr = self.xregs[rs1 as usize];
                let value = self.bus_read(addr, Sizes::Word)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::ScW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                log_trace!("SC.W: rd: {rd}, rs1: {rs1}, rs2: {rs2}, rl: {rl}, aq: {aq}");
                let addr = self.xregs[rs1 as usize];
                let value = self.xregs[rs2 as usize];
                let reserved = self.bus_read(addr, Sizes::Word)?;
                if reserved == 1 {
                    self.bus_write(addr, value, Sizes::Word)?;
                    self.xregs[rd as usize] = 0;
                } else {
                    self.xregs[rd as usize] = 1;
                }
            }

            // RV32C
            InstructionDecoded::CAddi4Spn { .. } => todo!(),
            InstructionDecoded::CNop { .. } => todo!(),
            InstructionDecoded::CSlli { .. } => todo!(),
        }

        Ok(())
    }

    pub fn dump_memory(&self, addr: MemorySize, size: MemorySize) {
        log_debug!("{:-^80}", "memory");
        for i in 0..size {
            let addr = addr + i;
            let value = self.bus_read(addr, Sizes::Byte).unwrap();
            log_debug!("{:#08x}: {:#02x}", addr, value);
        }
        log_debug!("{:-^80}", "");
    }

    pub fn dump_registers(&mut self) {
        const RVABI: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];
        log_trace!("{:-^80}", "registers");
        log_trace!("{:3}({:^4}) = {:<#18x}", "pc", "pc", self.pc);
        self.xregs[0] = 0;
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
            log_trace!("{line}");
        }
    }

    #[inline]
    pub fn finished(&self) -> bool {
        self.pc >= (DRAM_BASE + DRAM_SIZE) as XRegisterSize
    }

    /// Execute a cycle on peripheral devices.
    pub fn devices_increment(&mut self) {
        // TODO: mtime in Clint and TIME in CSR should be the same value.
        // Increment the timer register (mtimer) in Clint.
        let state = (&mut self.state) as *mut State;
        let clint = self.get_device_mut::<Clint>().expect("Clint not found");
        clint.increment(unsafe { state.as_mut().expect("Failed to retrieve state") });
        // Increment the value in the TIME and CYCLE registers in CSR.
        self.state.increment_time();
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let inst = self.fetch()?;
        // Execute an instruction.
        let trap = match self.execute(inst) {
            Ok(_) => Trap::Requested, // Return a placeholder trap
            Err(exception) => exception.take_trap(self),
        };

        if let Trap::Fatal = trap {
            log_error!("pc: {:#x}, trap {:#?}", self.get_pc(), trap);
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        loop {
            self.devices_increment();

            if let Some(interrupt) = self.check_pending_interrupt() {
                interrupt.take_trap(self);
            }

            self.step()?;
        }
    }

    pub fn initialize_dram(&mut self, program: &[u8]) -> Result<(), Exception> {
        log_debug!("{:-^80}", "initializing dram");
        let mut addr = DRAM_BASE as MemorySize;
        for bytes in program.chunks_exact(4) {
            let word = u32::from_ne_bytes(bytes.try_into().unwrap()).to_le();
            log_trace!("{:#x}: {:#x}", addr, word);
            self.bus_write(addr, word, Sizes::Word)?;
            addr += 4;
        }
        log_debug!("{:-^80}", format!(" Loaded bytes {} ", addr - DRAM_BASE as MemorySize));

        Ok(())
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
