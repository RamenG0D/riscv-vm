use crate::{
    bit_ops::zero_extend, bus::{Bus, Device}, convert_memory, csr::{Csr, Mode, MEPC, MSTATUS, SEPC, SSTATUS}, devices::virtio::Virtio, memory::{
        dram::{Sizes, DRAM_BASE, DRAM_SIZE},
        virtual_memory::MemorySize,
    }, registers::{FRegisters, XRegisterSize, XRegisters}, trap::Exception
};
use log::{debug, info, trace};
use riscv_decoder::{
    decoded_inst::InstructionDecoded,
    decoder::{try_decode, try_decode_compressed},
    instructions::compressed::is_compressed,
};

// 32 bit RiscV CPU architecture
pub struct Cpu {
    xregs: XRegisters,
    fregs: FRegisters,

    /// program counter
    pc: XRegisterSize,

    /// The current privilege mode.
    mode: Mode,

    /// little endian memory / stack array
    bus: Bus,

    /// Csr controller
    state: Csr,
}

impl Cpu {
    pub fn new() -> Self {
        info!("Initializing CPU...");
        let mut registers = XRegisters::new();
        // registers[1] = DRAM_BASE; // return address

        registers[2] = DRAM_BASE + DRAM_SIZE; // stack pointer

        let cpu = Self {
            xregs: registers,
            fregs: FRegisters::new(),

            pc: DRAM_BASE as u32,

            mode: Mode::Machine,

            bus: Bus::new(),

            state: Csr::new(),
        };
        info!("CPU initialized");
        cpu
    }

    pub fn virtio(&self) -> &Virtio {
        self.bus.get_virtio()
    }
    pub fn virtio_mut(&mut self) -> &mut Virtio {
        self.bus.get_virtio_mut()
    }

    pub fn bus_read(&self, addr: u32, size: Sizes) -> Result<u32, Exception> {
        self.bus.read(addr, size)
    }

    pub fn bus_write(&mut self, addr: u32, value: u32, size: Sizes) -> Result<(), Exception> {
        self.bus.write(addr, value, size)
    }

    pub fn add_device(&mut self, device: Box<dyn Device>) {
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

    pub fn read_csr(&self, addr: usize) -> u32 {
        self.state.read_csr(addr).expect("Failed to read CSR")
    }
    pub fn write_csr(&mut self, addr: usize, value: u32) {
        self.state
            .write_csr(addr, value)
            .expect("Failed to write CSR");
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

    pub fn fetch(&mut self) -> Result<InstructionDecoded, Exception> {
        debug_assert!(self.pc % 4 == 0, "PC is not aligned to 4 bytes");

        log::info!("PC: {:#X}", self.pc);

        let inst = self.bus.read(self.pc, Sizes::Word)?;

        if is_compressed(inst) {
            self.pc += 2;
            try_decode_compressed(inst).map_err(|e| {
                log::error!(
                    "Failed to decode compressed instruction: {:#X} => {e:?}",
                    inst
                );
                Exception::IllegalInstruction
            })
        } else {
            self.pc += 4;
            try_decode(inst).map_err(|e| {
                log::error!("Failed to decode instruction: {:#X} => {e:?}", inst);
                Exception::IllegalInstruction
            })
        }
    }

    pub fn execute(&mut self, inst: InstructionDecoded) -> Result<(), Exception> {
        // x0 must always be zero (irl the circuit is literally hardwired to electriacal equivalent of 0)
        self.xregs[0] = 0;

        match inst {
            InstructionDecoded::Add { rd, rs1, rs2 } => {
                debug!("ADD: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                debug!("rs1 = {rs1}, rs2 = {rs2}");
                self.xregs[rd as usize] = rs1.wrapping_add(rs2) as XRegisterSize;
            }
            InstructionDecoded::Addi { rd, rs1, imm } => {
                debug!("ADDI: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_add(imm as i32) as XRegisterSize;
            }
            InstructionDecoded::Sub { rd, rs1, rs2 } => {
                debug!("SUB: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = rs1.wrapping_sub(rs2) as XRegisterSize;
            }
            InstructionDecoded::And { rd, rs1, rs2 } => {
                debug!("AND: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 & rs2;
            }
            InstructionDecoded::Andi { rd, rs1, imm } => {
                debug!("ANDI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1 & imm;
            }
            InstructionDecoded::Or { rd, rs1, rs2 } => {
                debug!("OR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 | rs2;
            }
            InstructionDecoded::Ori { rd, rs1, imm } => {
                debug!("ORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1 | imm;
            }
            InstructionDecoded::Xor { rd, rs1, rs2 } => {
                debug!("XOR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1 ^ rs2;
            }
            InstructionDecoded::Xori { rd, rs1, imm } => {
                debug!("XORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1 ^ imm;
            }
            InstructionDecoded::Sll { rd, rs1, rs2 } => {
                debug!("SLL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1.wrapping_shl(rs2);
            }
            InstructionDecoded::Slli { rd, rs1, imm } => {
                debug!("SLLI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1.wrapping_shr(imm);
            }
            InstructionDecoded::Srl { rd, rs1, rs2 } => {
                debug!("SRL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1.wrapping_shr(rs2);
            }
            InstructionDecoded::Srli { rd, rs1, imm } => {
                debug!("SRLI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1.wrapping_shr(imm);
            }
            InstructionDecoded::Sra { rd, rs1, rs2 } => {
                debug!("SRA: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = rs1.wrapping_shr(rs2);
            }
            InstructionDecoded::Srai { rd, rs1, imm } => {
                debug!("SRAI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = rs1.wrapping_shr(imm);
            }
            InstructionDecoded::Lui { rd, imm } => {
                debug!("LUI: rd: {rd}, imm: {imm}");
                debug!("imm = {}", imm << 12);
                self.xregs[rd as usize] = imm << 12;
            }
            InstructionDecoded::AuiPc { rd, imm } => {
                debug!("AUIPC: rd: {rd}, imm: {imm}");
                self.xregs[rd as usize] =
                    (self.pc as i32).wrapping_add(imm as i32).wrapping_sub(4) as u32;
            }
            InstructionDecoded::Jal { rd, imm } => {
                debug!("JAL: rd: {rd}, imm: {}", imm as i32);
                self.xregs[rd as usize] = self.pc; // save pc without + 4 because its already moved
                let (pc, imm) = (self.pc as i32, imm as i32);
                let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                debug!("Jumping to {:#X}", npc);
                self.pc = npc;
            }
            InstructionDecoded::Jalr { rd, rs1, imm } => {
                debug!("JALR: rd: {rd}, rs1: {rs1}, imm: {imm}");

                debug!("RA is currently: {:#X}", self.xregs[1]);

                self.xregs[rd as usize] = self.pc;
                let (reg, imm) = (self.xregs[rs1 as usize] as i32, imm as i32);
                let npc = reg.wrapping_add(imm) as XRegisterSize;
                debug!("Jumping to {:#X}", npc);
                self.pc = npc;
            }
            InstructionDecoded::Beq { rs1, rs2, imm } => {
                debug!("BEQ: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                if rs1 == rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    debug!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bne { rs1, rs2, imm } => {
                debug!("BNE: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                debug!("rs1 = {rs1}, rs2 = {rs2}");
                if rs1 != rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    debug!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Blt { rs1, rs2, imm } => {
                debug!("BLT: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                if rs1 < rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    debug!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bge { rs1, rs2, imm } => {
                debug!("BGE: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                if rs1 >= rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    debug!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bltu { rs1, rs2, imm } => {
                debug!("BLTU: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                if rs1 < rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    debug!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Bgeu { rs1, rs2, imm } => {
                debug!("BGEU: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                if rs1 >= rs2 {
                    let (pc, imm) = (self.pc as i32, imm as i32);
                    let npc = pc.wrapping_add(imm).wrapping_sub(4) as XRegisterSize;
                    debug!("Branching to {:#X}", npc);
                    self.pc = npc;
                }
            }
            InstructionDecoded::Lb { rd, rs1, imm } => {
                debug!("LB: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Reading from address: {:#X}", addr);
                let value = self.bus.read(addr, Sizes::Byte)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Lh { rd, rs1, imm } => {
                debug!("LH: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Reading from address: {:#X}", addr);
                let value = self.bus.read(addr, Sizes::HalfWord)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Lw { rd, rs1, imm } => {
                debug!("LW: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Reading from address: {:#X}", addr);
                let value = self.bus.read(addr, Sizes::Word)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lbu { rd, rs1, imm } => {
                debug!("LBU: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Reading from address: {:#X}", addr);
                // the read value must be zero-extended to 32 bits
                let value = self.bus.read(addr, Sizes::Byte)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Lhu { rd, rs1, imm } => {
                debug!("LHU: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Reading from address: {:#X}", addr);
                // the read value must be zero-extended to 32 bits
                let value = self.bus.read(addr, Sizes::HalfWord)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Lwu { rd, rs1, imm } => {
                debug!("LWU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Reading from address: {:#X}", addr);
                let value = self.bus.read(addr, Sizes::Word)?;
                self.xregs[rd as usize] = zero_extend(value);
            }
            InstructionDecoded::Sb { rs1, rs2, imm } => {
                debug!("SB: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize] as u8 as u32;
                self.bus.write(addr, value, Sizes::Byte)?;
            }
            InstructionDecoded::Sh { rs1, rs2, imm } => {
                debug!("SH: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize] as u16 as u32;
                self.bus.write(addr, value, Sizes::HalfWord)?;
            }
            InstructionDecoded::Sw { rs1, rs2, imm } => {
                debug!("SW: rs1: {rs1}, rs2: {rs2}, imm: {}", imm as i32);
                let addr = (self.xregs[rs1 as usize] as i32).wrapping_add(imm as i32) as u32;
                debug!("Writing to address: {:#X}", addr);
                let value = self.xregs[rs2 as usize];
                self.bus.write(addr, value, Sizes::Word)?;
            }
            InstructionDecoded::ECall => {
                debug!("ECALL");
                match self.mode {
                    Mode::User => {
                        trace!("ECall => User mode");
                        return Err(Exception::EnvironmentCallFromUMode);
                    }
                    Mode::Supervisor => {
                        trace!("ECall => Supervisor mode");
                        return Err(Exception::EnvironmentCallFromSMode);
                    }
                    Mode::Machine => {
                        trace!("ECall => Machine mode");
                        return Err(Exception::EnvironmentCallFromMMode);
                    }
                }
            }
            InstructionDecoded::EBreak => {
                debug!("EBREAK");
                // ebreak
                // Makes a request of the debugger bu raising a Breakpoint
                // exception.
                return Err(Exception::Breakpoint);
            }
            InstructionDecoded::SRet => {
                debug!("SRET");
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
                debug!("MRET");
                // The MRET instruction returns from a machine-mode exception
                // handler. It does the following operations:
                // - Sets the pc to CSRs[mepc].
                // - Sets the privilege mode to CSRs[mstatus].MPP.
                // - Sets CSRs[mstatus].MIE to CSRs[mstatus].MPIE.
                // - Sets CSRs[mstatus].MPIE to 1.
                // - Sets CSRs[mstatus].MPP to 0.
                self.pc = self.read_csr(MEPC);
                // MPP is two bits wide at [11..12] of the MSTATUS csr.
                self.mode = match (self.read_csr(MSTATUS) >> 11) & 0b11 {
                    2 => Mode::Machine,
                    1 => Mode::Supervisor,
                    _ => Mode::User,
                };
                // The MPIE bit is the 7th and the MIE bit is the 3rd of the
                // MSTATUS csr.
                self.write_csr(
                    MSTATUS,
                    if ((self.read_csr(MSTATUS) >> 7) & 1) == 1 {
                        self.read_csr(MSTATUS) | (1 << 3)
                    } else {
                        self.read_csr(MSTATUS) & !(1 << 3)
                    },
                );
                self.write_csr(MSTATUS, self.read_csr(MSTATUS) | (1 << 7));
                self.write_csr(MSTATUS, self.read_csr(MSTATUS) & !(0b11 << 11));
            }
            InstructionDecoded::SFenceVma => {
                debug!("SFENCE.VMA");
                // do nothing
            }
            InstructionDecoded::CsrRw { rd, rs1, imm } => {
                debug!("CSRRW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as usize);
                self.write_csr(imm as usize, self.xregs[rs1 as usize]);
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::CsrRs { rd, rs1, imm } => {
                debug!("CSRRS: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as usize);
                info!("OLD CSR: {:#X}", t);
                self.write_csr(imm as usize, t | self.xregs[rs1 as usize]);
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::CsrRc { rd, rs1, imm } => {
                debug!("CSRRC: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as usize);
                self.write_csr(imm as usize, t & (!self.xregs[rs1 as usize]));
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::CsrRwi { rd, rs1, imm } => {
                debug!("CSRRWI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                self.xregs[rd as usize] = self.read_csr(imm as usize);
                self.write_csr(imm as usize, rs1);
            }
            InstructionDecoded::CsrRsi { rd, rs1, imm } => {
                debug!("CSRRSI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as usize);
                self.write_csr(imm as usize, t | rs1);
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::CsrRci { rd, rs1, imm } => {
                let t = self.read_csr(imm as usize);
                self.write_csr(imm as usize, t & (!rs1));
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::Slti { rd, rs1, imm } => {
                debug!("SLTI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = if rs1 < imm { 1 } else { 0 };
            }
            InstructionDecoded::Sltiu { rd, rs1, imm } => {
                debug!("SLTIU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = if rs1 < imm { 1 } else { 0 };
            }
            InstructionDecoded::Slt { rd, rs1, rs2 } => {
                debug!("SLT: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }
            InstructionDecoded::Sltu { rd, rs1, rs2 } => {
                debug!("SLTU: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                self.xregs[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }

            InstructionDecoded::Fence {
                rd,
                rs1,
                fm,
                pred,
                succ,
            } => {
                debug!("FENCE: rd: {rd}, rs1: {rs1}, fm: {fm}, pred: {pred}, succ: {succ}");
                info!("Not Currently Implemented");
            }
            InstructionDecoded::FenceI {
                rd,
                rs1,
                fm,
                pred,
                succ,
            } => {
                debug!("FENCE.I: rd: {rd}, rs1: {rs1}, fm: {fm}, pred: {pred}, succ: {succ}");
                info!("Not Currently Implemented");
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
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                self.xregs[rd as usize] = (rs1 * rs2) as XRegisterSize;
            }
            InstructionDecoded::Mulh { .. } => todo!(),
            InstructionDecoded::Mulsu { .. } => todo!(),
            InstructionDecoded::Mulu { .. } => todo!(),
            InstructionDecoded::Div { .. } => todo!(),
            InstructionDecoded::Divu { .. } => todo!(),
            InstructionDecoded::Rem { .. } => todo!(),
            InstructionDecoded::Remu { .. } => todo!(),

            // RV32A
            InstructionDecoded::LrW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                debug!("LR.W: rd: {rd}, rs1: {rs1}, rs2: {rs2}, rl: {rl}, aq: {aq}");
                let addr = self.xregs[rs1 as usize];
                let value = self.bus.read(addr, Sizes::Word)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::ScW {
                rd,
                rs1,
                rs2,
                rl,
                aq,
            } => {
                debug!("SC.W: rd: {rd}, rs1: {rs1}, rs2: {rs2}, rl: {rl}, aq: {aq}");
                let addr = self.xregs[rs1 as usize];
                let value = self.xregs[rs2 as usize];
                let reserved = self.bus.read(addr, Sizes::Word)?;
                if reserved == 1 {
                    self.bus.write(addr, value, Sizes::Word)?;
                    self.xregs[rd as usize] = 0;
                } else {
                    self.xregs[rd as usize] = 1;
                }
            }
            InstructionDecoded::CNop => (),
            InstructionDecoded::CAddi4Spn { rd, nzuimm } => {
                debug!("C.ADDI4SPN: rd: {rd}, nzuimm: {nzuimm}");
                // imm * 4 + SP
                self.xregs[rd as usize] = self.xregs[2] + (nzuimm as u32) * 4;
            }
            InstructionDecoded::CSlli { rd, rs1, shamt } => {
                debug!("C.SLLI: rd: {rd}, rs1: {rs1}, shamt: {shamt}");
                self.xregs[rd as usize] = self.xregs[rs1 as usize] << shamt;
            }
        }

        Ok(())
    }

    pub fn dump_registers(&mut self) {
        const RVABI: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];
        info!("{:-^80}", "registers");
        info!("{:3}({:^4}) = {:<#18x}", "pc", "pc", self.pc);
        self.xregs[0] = 0;
        for i in (0..32).step_by(4) {
            let (i0, i1, i2, i3) = (
                format!("x{}", i + 0),
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
            info!("{line}");
        }
    }

    #[inline]
    pub fn finished(&self) -> bool {
        self.pc >= (DRAM_BASE + DRAM_SIZE) as XRegisterSize
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let inst = self.fetch()?;
        self.execute(inst)?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Exception> {
        while !self.finished() {
            let inst = self.fetch()?;
            self.pc += 4;
            self.execute(inst)?;
        }
        Ok(())
    }

    pub fn load_program_raw(&mut self, program: &[u8]) -> Result<(), Exception> {
        let program = convert_memory(program);
        let mut addr = DRAM_BASE as MemorySize;
        for word in program {
            self.bus.write(addr, word, Sizes::Word)?;
            addr += 4;
        }
        Ok(())
    }
}
