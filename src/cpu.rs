use log::debug;

use crate::{
    bus::Bus,
    csr::State,
    instruction_sets::rv32i::{Instruction, InstructionDecoded},
    memory::{
        dram::{Sizes, DRAM_BASE, DRAM_SIZE},
        virtual_memory::MemorySize,
    },
};

#[test]
pub fn kernel_test() {
    //
}

#[test]
pub fn program_test() {
    let mut cpu = Cpu::new();

    println!("Loading program...");
    cpu.load_program_raw(include_bytes!("../c_test/test.bin"))
        .expect("Failed to load program");
    println!("Program LOADED");

    while cpu.pc < (DRAM_BASE + DRAM_SIZE) as RegisterSize {
        match cpu.execute() {
            Ok(_) => {
                println!("{}", cpu.to_string());
            }
            Err(e) => {
                eprintln!("Error: {e}");
            }
        }
        cpu.pc += 4;
    }

    println!("{}", cpu.to_string());
}

pub type RegisterSize = u32;

// 32 bit RiscV CPU architecture
pub struct Cpu {
    // From https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf#page=22&zoom=auto,-95,583
    registers: [RegisterSize; 32],

    // program counter
    pc: RegisterSize,

    // little endian memory / stack array
    bus: Bus,

    state: State,
}

impl Cpu {
    pub const ZERO: RegisterSize = 0;
    pub const RA: RegisterSize = 1;
    pub const SP: RegisterSize = 2;
    pub const GP: RegisterSize = 3;
    pub const TP: RegisterSize = 4;
    pub const PC: RegisterSize = 32;

    pub fn new() -> Self {
        let mut registers = [0; 32];
        registers[2] = (DRAM_BASE + DRAM_SIZE) as RegisterSize;
        Self {
            registers,
            pc: DRAM_BASE as RegisterSize,

            bus: Bus::new(),

            state: State::new(),
        }
    }

    pub fn get_pc(&self) -> RegisterSize {
        self.pc
    }

    pub fn set_pc(&mut self, pc: RegisterSize) {
        self.pc = pc;
    }

    pub fn get_register(&self, register: RegisterSize) -> Result<&RegisterSize, String> {
        match register {
            0..=31 => Ok(&self.registers[register as usize]),
            32 => Ok(&self.pc),
            _ => Err(format!(
                "The register '{register}' is not an addressable register?"
            )),
        }
    }

    pub fn get_register_mut(
        &mut self,
        register: RegisterSize,
    ) -> Result<&mut RegisterSize, String> {
        match register {
            0..=31 => Ok(&mut self.registers[register as usize]),
            32 => Ok(&mut self.pc),
            _ => Err(format!(
                "The register '{register}' is not an addressable register?"
            )),
        }
    }

    pub fn fetch(&self) -> Result<Instruction, String> {
        let inst = self
            .bus
            .read(self.pc, Sizes::Word)
            .ok_or(format!("Failed to fetch memory at addr {}", self.pc))?;
        Instruction::try_from(inst)
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let inst = self.fetch()?;

        match inst.decode()? {
            InstructionDecoded::Add { rd, rs1, rs2 } => {
                debug!("ADD: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                debug!("rs1 = {rs1}, rs2 = {rs2}");
                self.registers[rd as usize] = (rs1 + rs2) as RegisterSize;
            }
            InstructionDecoded::Addi { rd, rs1, imm } => {
                debug!("ADDI: rd: {rd}, rs1: {rs1}, imm: {}", imm as i32);
                let rs1 = self.registers[rs1 as usize] as i32;
                self.registers[rd as usize] = (rs1 + imm as i32) as RegisterSize;
            }
            InstructionDecoded::Sub { rd, rs1, rs2 } => {
                debug!("SUB: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = (rs1 - rs2) as RegisterSize;
            }
            InstructionDecoded::And { rd, rs1, rs2 } => {
                debug!("AND: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 & rs2;
            }
            InstructionDecoded::Andi { rd, rs1, imm } => {
                debug!("ANDI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 & imm;
            }
            InstructionDecoded::Or { rd, rs1, rs2 } => {
                debug!("OR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 | rs2;
            }
            InstructionDecoded::Ori { rd, rs1, imm } => {
                debug!("ORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 | imm;
            }
            InstructionDecoded::Xor { rd, rs1, rs2 } => {
                debug!("XOR: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 ^ rs2;
            }
            InstructionDecoded::Xori { rd, rs1, imm } => {
                debug!("XORI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 ^ imm;
            }
            InstructionDecoded::Sll { rd, rs1, rs2 } => {
                debug!("SLL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 << rs2;
            }
            InstructionDecoded::Slli { rd, rs1, imm } => {
                debug!("SLLI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 << imm;
            }
            InstructionDecoded::Srl { rd, rs1, rs2 } => {
                debug!("SRL: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 >> rs2;
            }
            InstructionDecoded::Srli { rd, rs1, imm } => {
                debug!("SRLI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 >> imm;
            }
            InstructionDecoded::Sra { rd, rs1, rs2 } => {
                debug!("SRA: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1.wrapping_shr(rs2);
            }
            InstructionDecoded::Srai { rd, rs1, imm } => {
                debug!("SRAI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1.wrapping_shr(imm);
            }
            InstructionDecoded::Lui { rd, imm } => {
                debug!("LUI: rd: {rd}, imm: {imm}");
                self.registers[rd as usize] = imm << 12;
            }
            InstructionDecoded::AuiPc { rd, imm } => {
                debug!("AUIPC: rd: {rd}, imm: {imm}");
                self.registers[rd as usize] = self.pc.wrapping_add(imm << 12);
            }
            InstructionDecoded::Jal { rd, imm1, imm2, imm3 } => {
                self.registers[rd as usize] = self.pc; // store the return address

                let imm = imm1 << 19 | imm2 << 11 | imm3 << 0;

                debug!("JAL: imm: {imm}");

                self.pc = self.pc.wrapping_add(imm);
            }
            InstructionDecoded::Jalr { .. } => todo!(),
            InstructionDecoded::Beq { .. } => todo!(),
            InstructionDecoded::Bne { .. } => todo!(),
            InstructionDecoded::Blt { .. } => todo!(),
            InstructionDecoded::Bge { .. } => todo!(),
            InstructionDecoded::Bltu { .. } => todo!(),
            InstructionDecoded::Bgeu { .. } => todo!(),
            InstructionDecoded::Lb { rd, rs1, imm } => {
                debug!("LB: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                let value = self.bus.read(addr as MemorySize, Sizes::Byte)
                    .ok_or(format!("Failed to read memory at addr {addr}"))?;
                self.registers[rd as usize] = value as i8 as i32 as u32;
            }
            InstructionDecoded::Lh { rd, rs1, imm } => {
                debug!("LH: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                let value = self
                    .bus
                    .read(addr, Sizes::HalfWord)
                    .ok_or(format!("Failed to read memory at addr {addr}"))?;
                self.registers[rd as usize] = value as i16 as i32 as u32;
            }
            InstructionDecoded::Lw { rd, rs1, imm } => {
                debug!("LW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                let value = self
                    .bus
                    .read(addr, Sizes::Word)
                    .ok_or(format!("Failed to read memory at addr {addr}"))?;
                self.registers[rd as usize] = value as i32 as u32;
            }
            InstructionDecoded::Lbu { rd, rs1, imm } => {
                debug!("LBU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                let value = self
                    .bus
                    .read(addr, Sizes::Byte)
                    .ok_or(format!("Failed to read memory at addr {addr}"))?;
                self.registers[rd as usize] = value;
            }
            InstructionDecoded::Lhu { rd, rs1, imm } => {
                debug!("LHU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                let value = self
                    .bus
                    .read(addr, Sizes::HalfWord)
                    .ok_or(format!("Failed to read memory at addr {addr}"))?;
                self.registers[rd as usize] = value;
            }
            InstructionDecoded::Lwu { rd, rs1, imm } => {
                debug!("LWU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                let value = self
                    .bus
                    .read(addr, Sizes::Word)
                    .ok_or(format!("Failed to read memory at addr {addr}"))?;
                self.registers[rd as usize] = value;
            }
            InstructionDecoded::Sb { rs1, rs2, imm } => {
                debug!("SB: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                self.bus
                    .write(addr, self.registers[rs2 as usize], Sizes::Byte)
                    .ok_or(format!("Failed to write byte to memory at addr {addr}"))?;
            }
            InstructionDecoded::Sh { rs1, rs2, imm } => {
                debug!("SH: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                self.bus
                    .write(addr, self.registers[rs2 as usize], Sizes::HalfWord)
                    .ok_or(format!("Failed to write hword memory at addr {addr}"))?;
            }
            InstructionDecoded::Sw { rs1, rs2, imm } => {
                debug!("SW: rs1: {rs1}, rs2: {rs2}, imm: {imm}");
                let addr = self.registers[rs1 as usize].wrapping_add(imm);
                self.bus
                    .write(addr, self.registers[rs2 as usize], Sizes::Word)
                    .ok_or(format!("Failed to write word memory at addr {addr}"))?;
            }
            InstructionDecoded::ECall => {
                debug!("ECALL");
            }
            InstructionDecoded::EBreak => {
                debug!("EBREAK");
            }
            InstructionDecoded::CsrRw { rd, rs1, imm } => {
                debug!("CSRRW: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let tmp = self.state.read_csr(imm as usize);
                self.state
                    .write_csr(imm as usize, self.registers[rs1 as usize]);
                self.registers[rd as usize] = tmp;
            }
            InstructionDecoded::CsrRs { rd, rs1, imm } => {
                debug!("CSRRS: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let tmp = self.state.read_csr(imm as usize);
                self.state
                    .write_csr(imm as usize, tmp | self.registers[rs1 as usize]);
                self.registers[rd as usize] = tmp;
            }
            InstructionDecoded::CsrRc { rd, rs1, imm } => {
                debug!("CSRRC: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let tmp = self.state.read_csr(imm as usize);
                self.state
                    .write_csr(imm as usize, tmp & !self.registers[rs1 as usize]);
                self.registers[rd as usize] = tmp;
            }
            InstructionDecoded::CsrRwi { rd, rs1, imm } => {
                debug!("CSRRWI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let tmp = self.state.read_csr(imm as usize);
                self.state
                    .write_csr(imm as usize, tmp.wrapping_add(self.registers[rs1 as usize]));
                self.registers[rd as usize] = tmp;
            }
            InstructionDecoded::CsrRsi { rd, rs1, imm } => {
                debug!("CSRRSI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let tmp = self.state.read_csr(imm as usize);
                self.state
                    .write_csr(imm as usize, tmp.wrapping_add(self.registers[rs1 as usize]));
                self.registers[rd as usize] = tmp;
            }
            InstructionDecoded::Fence { .. } => todo!(),
            InstructionDecoded::FenceI { .. } => todo!(),
            InstructionDecoded::Slti { rd, rs1, imm } => {
                debug!("SLTI: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = if rs1 < imm { 1 } else { 0 };
            }
            InstructionDecoded::Sltiu { rd, rs1, imm } => {
                debug!("SLTIU: rd: {rd}, rs1: {rs1}, imm: {imm}");
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = if rs1 < imm { 1 } else { 0 };
            }
            InstructionDecoded::Slt { rd, rs1, rs2 } => {
                debug!("SLT: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }
            InstructionDecoded::Sltu { rd, rs1, rs2 } => {
                debug!("SLTU: rd: {rd}, rs1: {rs1}, rs2: {rs2}");
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < (DRAM_BASE + DRAM_SIZE) as RegisterSize {
            self.execute()?;
            self.pc += 4;
        }
        Ok(())
    }

    pub fn load_program_raw(&mut self, program: &[u8]) -> Result<(), String> {
        let mut addr = DRAM_BASE as MemorySize;
        for i in 0..program.len() {
            self.bus
                .write(addr, program[i] as RegisterSize, Sizes::Byte)
                .ok_or(format!("Failed to write memory at addr {addr}"))?;
            addr += 1;
        }
        Ok(())
    }

    pub fn load_program<T>(&mut self, program: &[T]) -> Result<(), String>
    where
        T: Into<Instruction> + Copy,
    {
        let program = program
            .iter()
            .map(|&inst| inst.into().to_inner())
            .collect::<Vec<_>>();
        let program =
            unsafe { std::slice::from_raw_parts(program.as_ptr() as *const u8, program.len() * 4) };
        self.load_program_raw(program)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..32 {
            s.push_str(&format!("x{:<2}: {:#010x}\n", i, self.registers[i]));
        }
        s.push_str(&format!("pc: {:#010x}\n", self.pc));
        s
    }
}
