use log::{debug, error, info};

use crate::{
    bus::Bus,
    csr::{Mode, State},
    instruction_sets::{instructions::InstructionDecoded, rv32i::Instruction},
    memory::{
        dram::{Sizes, DRAM_BASE, DRAM_SIZE},
        virtual_memory::MemorySize,
    },
    registers::{FRegisters, XRegisterSize, XRegisters}, trap::Exception,
};

// #[test]
// pub fn fib_test() {
//     let mut cpu = Cpu::new();

//     println!("Loading program...");
//     cpu.load_program_raw(include_bytes!("../c_test/fib.bin"))
//         .expect("Failed to load program");
//     println!("Program LOADED");

//     while cpu.pc < (DRAM_BASE + DRAM_SIZE) as XRegisterSize {
//         match cpu.execute() {
//             Ok(_) => {
//                 println!("{}", cpu.to_string());
//             }
//             Err(e) => {
//                 eprintln!("Error: {e}");
//             }
//         }
//         cpu.pc += 4;
//     }

//     println!("{}", cpu.to_string());
// }

/*#[test]
pub fn program_test() {
    // crate::logging::init_logging(); // enable for extra debug output
    let mut cpu = Cpu::new();

    debug!("Loading program...");
    cpu.load_program_raw(include_bytes!("../c_test/test.bin"))
        .expect("Failed to load program");
    debug!("Program LOADED");

    while !cpu.finished() {
        match cpu.step() {
            Ok(_) => (),
            Err(e) => {
                e.take_trap(&mut cpu);
                if e.is_fatal() {
                    error!("Fatal trap: {:#X}", e.exception_code());
                    break;
                }
            }
        }
    }

    debug!("{}", cpu.to_string());

    debug!("Program executed");

    assert_eq!(*cpu.get_register(10).unwrap(), 31);
    assert_eq!(*cpu.get_register(15).unwrap(), 0x1F);
}*/

// 32 bit RiscV CPU architecture
pub struct Cpu {
    xregs: XRegisters,
    fregs: FRegisters,

    // program counter
    pc: XRegisterSize,

    /// The current privilege mode.
    mode: Mode,

    // little endian memory / stack array
    bus: Bus,

    state: State,
}

impl Cpu {
    pub const ZERO: usize = 0;
    pub const RA: usize = 1;
    pub const SP: usize = 2;
    pub const GP: usize = 3;
    pub const TP: usize = 4;
    pub const PC: usize = 32;

    pub fn new() -> Self {
        info!("Initializing CPU...");
        let mut registers = XRegisters::new();
        registers[2] = (DRAM_BASE + DRAM_SIZE) as u32;
        let cpu = Self {
            xregs: registers,
            fregs: FRegisters::new(),

            pc: DRAM_BASE as u32,

            mode: Mode::Machine,

            bus: Bus::new(),

            state: State::new(),
        };
        info!("CPU initialized");
        cpu
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
        self.state.write_csr(addr, value).expect("Failed to write CSR");
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

    pub fn fetch(&self) -> Result<Instruction, Exception> {
        let inst = self.bus.read(self.pc, Sizes::Word)?;
        Instruction::try_from(inst).map_err(|e| {
            error!("Failed to decode instruction: {e}");
            Exception::IllegalInstruction
        })
    }

    pub fn execute(&mut self, inst: Instruction) -> Result<(), Exception> {
        let decoded = inst.decode().map_err(|e| {
            error!("Failed to decode instruction: {e}");
            Exception::IllegalInstruction
        })?;

        match decoded {
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
                let imm = imm as i32;
                self.xregs[rd as usize] = rs1.wrapping_add(imm) as XRegisterSize;
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
                debug!("LUI: rd: {rd}, imm: {}", imm << 12);
                self.xregs[rd as usize] = imm << 12;
            }
            InstructionDecoded::AuiPc { rd, imm } => {
                debug!("AUIPC: rd: {rd}, imm: {imm}");
                self.xregs[rd as usize] = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            InstructionDecoded::Jal { rd, imm } => {
                debug!("JAL: rd: {rd}, imm: {imm}");
                self.xregs[rd as usize] = self.pc; // save pc without +4 because its already moved

                self.pc = self.pc.wrapping_add(imm).wrapping_sub(4);
            }
            InstructionDecoded::Jalr { rd, rs1, imm } => {
                debug!("JALR: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                self.xregs[rd as usize] = self.pc;

                self.pc = rs1.wrapping_add(imm).wrapping_sub(4);
            }
            InstructionDecoded::Beq { rs1, rs2, imm } => {
                debug!("BEQ: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                if rs1 == rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bne { rs1, rs2, imm } => {
                debug!("BNE: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                if rs1 != rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Blt { rs1, rs2, imm } => {
                debug!("BLT: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                if rs1 < rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bge { rs1, rs2, imm } => {
                debug!("BGE: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize] as i32;
                let rs2 = self.xregs[rs2 as usize] as i32;
                if rs1 >= rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bltu { rs1, rs2, imm } => {
                debug!("BLTU: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                if rs1 < rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bgeu { rs1, rs2, imm } => {
                debug!("BGEU: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let rs1 = self.xregs[rs1 as usize];
                let rs2 = self.xregs[rs2 as usize];
                if rs1 >= rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Lb { rd, rs1, imm } => {
                debug!("LB: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                let value = self.bus.read(addr as MemorySize, Sizes::Byte)?;
                self.xregs[rd as usize] = value as i8 as i32 as u32;
            }
            InstructionDecoded::Lh { rd, rs1, imm } => {
                debug!("LH: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                let value = self.bus.read(addr, Sizes::HalfWord)?;
                self.xregs[rd as usize] = value as i16 as i32 as u32;
            }
            InstructionDecoded::Lw { rd, rs1, imm } => {
                debug!("LW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_sub(imm);
                let value = self.bus.read(addr, Sizes::Word)?;
                self.xregs[rd as usize] = value as i32 as u32;
            }
            InstructionDecoded::Lbu { rd, rs1, imm } => {
                debug!("LBU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                let value = self.bus.read(addr, Sizes::Byte)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lhu { rd, rs1, imm } => {
                debug!("LHU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                let value = self.bus.read(addr, Sizes::HalfWord)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Lwu { rd, rs1, imm } => {
                debug!("LWU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                let value = self.bus.read(addr, Sizes::Word)?;
                self.xregs[rd as usize] = value;
            }
            InstructionDecoded::Sb { rs1, rs2, imm } => {
                debug!("SB: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                self.bus.write(addr, self.xregs[rs2 as usize], Sizes::Byte)?;
            }
            InstructionDecoded::Sh { rs1, rs2, imm } => {
                debug!("SH: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                self.bus.write(addr, self.xregs[rs2 as usize], Sizes::HalfWord)?;
            }
            InstructionDecoded::Sw { rs1, rs2, imm } => {
                debug!("SW: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let addr = self.xregs[rs1 as usize].wrapping_add(imm);
                self.bus.write(addr, self.xregs[rs2 as usize], Sizes::Word)?;
            }
            InstructionDecoded::ECall => {
                debug!("ECALL");
                match self.mode {
                    Mode::User => {
                        debug!("ECall => User mode");
                        return Err(Exception::EnvironmentCallFromUMode);
                    }
                    Mode::Supervisor => {
                        debug!("ECall => Supervisor mode");
                        return Err(Exception::EnvironmentCallFromSMode);
                    }
                    Mode::Machine => {
                        debug!("ECall => Machine mode");
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
            InstructionDecoded::CsrRw { rd, rs1, imm } => {
                debug!("CSRRW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as usize);
                self.write_csr(imm as usize, self.xregs.get(rs1 as usize));
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::CsrRs { rd, rs1, imm } => {
                debug!("CSRRS: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as usize);
                self.write_csr(imm as usize, t | self.xregs.get(rs1 as usize));
                self.xregs[rd as usize] = t;
            }
            InstructionDecoded::CsrRc { rd, rs1, imm } => {
                debug!("CSRRC: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let t = self.read_csr(imm as usize);
                self.write_csr(imm as usize, t & (!self.xregs.get(rs1 as usize)));
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

            // RV32M
            InstructionDecoded::Mul { rd, rs1, rs2 } => {
                debug!("MUL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
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
            _ => todo!(),
        }

        Ok(())
    }

    #[inline]
    pub fn finished(&self) -> bool {
        self.pc >= (DRAM_BASE + DRAM_SIZE) as XRegisterSize
    }

    pub fn step(&mut self) -> Result<(), Exception> {
        let inst = self.fetch()?;
        self.pc += 4;
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
        let mut addr = DRAM_BASE as MemorySize;
        for i in 0..program.len() {
            self.bus.write(addr, program[i] as XRegisterSize, Sizes::Byte)?;
            addr += 1;
        }
        Ok(())
    }

    pub fn load_program<T>(&mut self, program: &[T]) -> Result<(), Exception>
    where
        T: Into<Instruction> + Copy,
    {
        let program = program
            .iter()
            .map(|&inst| inst.into().to_inner())
            .collect::<Vec<_>>();
        let program = unsafe {
            std::slice::from_raw_parts(program.as_ptr() as *const u8, program.len() * 4)
        };
        self.load_program_raw(program)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..32 {
            s.push_str(&format!("x{:<2}: {:#010x}\n", i, self.xregs[i]));
        }
        s.push_str(&format!("pc: {:#010x}\n", self.pc));
        s
    }
}
