use super::instructions::InstructionDecoded;

mod internal {
    pub use bit_ops::bitops_u32::create_mask;
}

pub type InstructionSize = u32;

pub struct InstructionBuilder {
    inst: InstructionSize,
}

impl InstructionBuilder {
    pub const fn builder() -> Self {
        Self { inst: 0 }
    }

    pub const fn build(self) -> InstructionSize {
        self.inst
    }

    pub const fn opcode(mut self, value: InstructionSize) -> Self {
        self.inst |= value;
        self
    }
    pub const fn rd(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 7;
        self
    }
    pub const fn rs1(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 15;
        self
    }
    pub const fn rs2(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 20;
        self
    }
    pub const fn funct3(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 12;
        self
    }
    pub const fn funct7(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 25;
        self
    }
    pub const fn imm1(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 20;
        self
    }
    pub const fn imm2(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 25;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstructionFormat {
    RType,
    IType,
    SType,
    UType,
    BType,
    JType,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    inst: InstructionSize,
    format: InstructionFormat,
}

impl Instruction {
    pub const fn make(inst: InstructionSize, format: InstructionFormat) -> Self {
        Self { inst, format }
    }

    // used to set certain bits in the instruction (args of the instruction)
    pub fn encode(&self, value: &[InstructionSize]) -> Instruction {
        let mut inst = self.inner();
        for (i, v) in value.iter().enumerate() {
            inst |= v << i;
        }
        Self::from(inst)
    }

    pub fn from(value: InstructionSize) -> Self {
        match Instruction::try_from(value) {
            Ok(v) => v,
            Err(e) => {
                panic!("Failed to create Instruction: {e}");
            }
        }
    }

    pub fn decode(self) -> Result<InstructionDecoded, String> {
        match self.format {
            InstructionFormat::IType => {
                let rd = self.rd().unwrap();
                let rs1 = self.rs1().unwrap();
                let imm = self.immediate1().unwrap();

                match self.opcode() {
                    instructions::LOAD_MATCH => {
                        match self.funct3().ok_or(format!("could not get funct3"))? {
                            instructions::lb::FUNCT3 => Ok(InstructionDecoded::Lb { rd, rs1, imm }),
                            instructions::lh::FUNCT3 => Ok(InstructionDecoded::Lh { rd, rs1, imm }),
                            instructions::lw::FUNCT3 => Ok(InstructionDecoded::Lw { rd, rs1, imm }),
                            instructions::ld::FUNCT3 => Err(format!("Unsuppored LD instruction (64 bit ONLY)")),
                            instructions::lbu::FUNCT3 => {
                                Ok(InstructionDecoded::Lbu { rd, rs1, imm })
                            }
                            instructions::lhu::FUNCT3 => {
                                Ok(InstructionDecoded::Lhu { rd, rs1, imm })
                            }
                            instructions::lwu::FUNCT3 => {
                                Ok(InstructionDecoded::Lwu { rd, rs1, imm })
                            }
                            _ => Err(format!("Unknown funct3 value for IType instruction => {:#X}( 0b{:032b} )",self.inner(), self.inner())),
                        }
                    }
                    instructions::ARITMETIC_IMMEDIATE_MATCH => {
                        match self.funct3().ok_or(format!("couldnt get funct3"))? {
                            instructions::addi::FUNCT3 =>  Ok(InstructionDecoded::Addi { rd, rs1,  imm }),
                            instructions::slli::FUNCT3 =>  Ok(InstructionDecoded::Slli { rd, rs1,  imm }),
                            instructions::slti::FUNCT3 =>  Ok(InstructionDecoded::Slti { rd, rs1,  imm }),
                            instructions::sltiu::FUNCT3 => Ok(InstructionDecoded::Sltiu { rd, rs1, imm }),
                            instructions::xori::FUNCT3 =>  Ok(InstructionDecoded::Xori { rd, rs1,  imm }),
                            instructions::srli::FUNCT3 => match self.funct7().ok_or(format!("couldnt get funct7"))? {
                                instructions::srli::FUNCT7 => Ok(InstructionDecoded::Srli { rd, rs1, imm }),
                                instructions::srai::FUNCT7 => Ok(InstructionDecoded::Srai { rd, rs1, imm }),
                                _ => Err(format!("Unknown funct7 value for IType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                            },
                            instructions::ori::FUNCT3 =>  Ok(InstructionDecoded::Ori { rd, rs1, imm }),
                            instructions::andi::FUNCT3 => Ok(InstructionDecoded::Andi { rd, rs1, imm }),
                            _ => Err(format!("Unknown funct3 value for IType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                        }
                    }
                    instructions::AUIPC_MATCH => Ok(InstructionDecoded::AuiPc { rd, imm }),
                    instructions::JALR_MATCH => Ok(InstructionDecoded::Jalr { rd, rs1, imm }),
                    instructions::FENCE_MATCH => {
                        let fm = bit_ops::bitops_u32::get_bits(imm, 4, 27);
                        let pred = bit_ops::bitops_u32::get_bits(imm, 4, 23);
                        let succ = bit_ops::bitops_u32::get_bits(imm, 4, 19);
                        match self.funct3().ok_or(format!("could not get funct3"))? {
                            instructions::fence::FUNCT3 => Ok(InstructionDecoded::Fence {
                                rd,
                                rs1,
                                fm,
                                pred,
                                succ,
                            }),
                            instructions::fence_i::FUNCT3 => Ok(InstructionDecoded::FenceI {
                                rd,
                                rs1,
                                fm,
                                pred,
                                succ,
                            }),
                            _ => Err(format!("Unknown funct3 value for IType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                        }
                    }

                    instructions::CSR_MATCH => {
                        let csr = (self.inner() & 0xfff00000) as i32 as u32 >> 20;
                        let (rs2, funct7) = (
                            bit_ops::bitops_u32::get_bits(imm, 4, 20),
                            bit_ops::bitops_u32::get_bits(imm, 6, 25)
                        );
                        match self.funct3().ok_or(format!("could not get funct3"))? {
                            instructions::csrrw::FUNCT3 => Ok(InstructionDecoded::CsrRw { rd, rs1, imm: csr }),
                            instructions::csrrs::FUNCT3 => Ok(InstructionDecoded::CsrRs { rd, rs1, imm: csr }),
                            instructions::csrrc::FUNCT3 => Ok(InstructionDecoded::CsrRc { rd, rs1, imm: csr }),
                            instructions::csrrwi::FUNCT3 => Ok(InstructionDecoded::CsrRwi { rd, rs1, imm: csr }),
                            instructions::csrrsi::FUNCT3 => Ok(InstructionDecoded::CsrRsi { rd, rs1, imm: csr }),
                            instructions::csrrci::FUNCT3 => Ok(InstructionDecoded::CsrRci { rd, rs1, imm: csr }),

                            instructions::ecall::FUNCT3 => match (rs2, funct7) {
                                (instructions::ecall::FUNCT7, 0) => Ok(InstructionDecoded::ECall),
                                (instructions::ebreak::FUNCT7, 0) => Ok(InstructionDecoded::EBreak),
                                (instructions::sret::FUNCT7, 8) => Ok(InstructionDecoded::SRet),
                                (instructions::mret::FUNCT7, 24) => Ok(InstructionDecoded::MRet),
                                (instructions::sfence_vma::FUNCT7, 9) => Ok(InstructionDecoded::SFenceVma),
                                _ => Err(format!("Unknown funct7 value for IType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                            },

                            _ => Err(format!("Unknown funct3 value for IType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                        }
                    }
                    _ => Err(format!("Unknown opcode for IType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                }
            }
            InstructionFormat::RType => {
                let rd = self.rd().unwrap();
                let rs1 = self.rs1().unwrap();
                let rs2 = self.rs2().unwrap();
                match self.opcode() {
                    instructions::ARITMETIC_REGISTER_MATCH => {
                        let (f3, f7) = (
                            self.funct3().ok_or(format!("failed to get funct3"))?,
                            self.funct7().ok_or(format!("failed to get funct7"))?,
                        );
                        match (f3, f7) {
                            (instructions::add::FUNCT3, instructions::add::FUNCT7) => Ok(InstructionDecoded::Add { rd, rs1, rs2 }),
                            (instructions::sub::FUNCT3, instructions::sub::FUNCT7) => Ok(InstructionDecoded::Sub { rd, rs1, rs2 }),
                            (instructions::sll::FUNCT3, 0) =>  Ok(InstructionDecoded::Sll { rd, rs1, rs2 }),
                            (instructions::slt::FUNCT3, 0) =>  Ok(InstructionDecoded::Slt { rd, rs1, rs2 }),
                            (instructions::sltu::FUNCT3, 0) => Ok(InstructionDecoded::Sltu { rd, rs1, rs2 }),
                            (instructions::xor::FUNCT3, 0) =>  Ok(InstructionDecoded::Xor { rd, rs1, rs2 }),
                            (instructions::srl::FUNCT3, instructions::srl::FUNCT7) => Ok(InstructionDecoded::Srl { rd, rs1, rs2 }),
                            (instructions::sra::FUNCT3, instructions::sra::FUNCT7) => Ok(InstructionDecoded::Sra { rd, rs1, rs2 }),
                            (instructions::or::FUNCT3, 0) =>  Ok(InstructionDecoded::Or { rd, rs1, rs2 }),
                            (instructions::and::FUNCT3, 0) => Ok(InstructionDecoded::And { rd, rs1, rs2 }),
                            // M Extension
                            (instructions::mul::FUNCT3, instructions::mul::FUNCT7) => Ok(InstructionDecoded::Mul { rd, rs1, rs2 }),
                            (instructions::mulh::FUNCT3, instructions::mulh::FUNCT7) => Ok(InstructionDecoded::Mulh { rd, rs1, rs2 }),
                            (instructions::mulsu::FUNCT3, instructions::mulsu::FUNCT7) => Ok(InstructionDecoded::Mulsu { rd, rs1, rs2 }),
                            (instructions::mulu::FUNCT3, instructions::mulu::FUNCT7) => Ok(InstructionDecoded::Mulu { rd, rs1, rs2 }),
                            (instructions::div::FUNCT3, instructions::div::FUNCT7) => Ok(InstructionDecoded::Div { rd, rs1, rs2 }),
                            (instructions::divu::FUNCT3, instructions::divu::FUNCT7) => Ok(InstructionDecoded::Divu { rd, rs1, rs2 }),
                            (instructions::rem::FUNCT3, instructions::rem::FUNCT7) => Ok(InstructionDecoded::Rem { rd, rs1, rs2 }),
                            (instructions::remu::FUNCT3, instructions::remu::FUNCT7) => Ok(InstructionDecoded::Remu { rd, rs1, rs2 }),
                            // A Extension
                            (instructions::lrw::FUNCT3, _) if f7 >> 2 == instructions::lrw::FUNCT5 => Ok(InstructionDecoded::LrW { rd, rs1, rs2, aq: 0, rl: 0 }),
                            (instructions::scw::FUNCT3, _) if f7 >> 2 == instructions::scw::FUNCT5 => Ok(InstructionDecoded::ScW { rd, rs1, rs2, aq: 0, rl: 0 }),

                            (instructions::amoswapw::FUNCT3, _) if f7 >> 2 == instructions::amoswapw::FUNCT5 => Ok(InstructionDecoded::AmoswapW { rd, rs1, rs2, rl: self.rl().unwrap(), aq: self.aq().unwrap() }),

                            _ => Err(format!("Unknown funct3/funct7 value for RType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                        }
                    }
                    _ => Err(format!("Unknown opcode for RType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                }
            }
            InstructionFormat::SType => {
                let rs1 = self.rs1().unwrap();
                let rs2 = self.rs2().unwrap();
                let imm = self.immediate1().unwrap();

                match self.opcode() {
                    instructions::STORE_MATCH => match self.funct3().ok_or(format!("could not get funct3"))? {
                        instructions::sb::FUNCT3 => Ok(InstructionDecoded::Sb { rs1, rs2, imm }),
                        instructions::sh::FUNCT3 => Ok(InstructionDecoded::Sh { rs1, rs2, imm }),
                        instructions::sw::FUNCT3 => Ok(InstructionDecoded::Sw { rs1, rs2, imm }),
                        instructions::sd::FUNCT3 => Err(format!("Unsuppored SD instruction (64 bit ONLY)")),
                        _ => Err(format!("Unknown funct3 value for SType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                    },
                    _ => Err(format!("Unknown opcode for SType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                }
            }
            InstructionFormat::UType => {
                let rd = self.rd().unwrap();
                let imm = self.immediate1().unwrap();
                match self.opcode() {
                    instructions::LUI_MATCH => Ok(InstructionDecoded::Lui { rd, imm }),
                    instructions::AUIPC_MATCH => Ok(InstructionDecoded::AuiPc { rd, imm }),
                    _ => Err(format!("Unknown opcode for UType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                }
            }
            InstructionFormat::BType => {
                let rs1 = self.rs1().unwrap();
                let rs2 = self.rs2().unwrap();
                let imm = self.immediate1().unwrap();
                match self.opcode() {
                    instructions::BRANCH_MATCH => match self.funct3().ok_or(format!("could not get funct3"))? {
                        instructions::beq::FUNCT3 => Ok(InstructionDecoded::Beq { rs1, rs2, imm }),
                        instructions::bne::FUNCT3 => Ok(InstructionDecoded::Bne { rs1, rs2, imm }),
                        instructions::blt::FUNCT3 => Ok(InstructionDecoded::Blt { rs1, rs2, imm }),
                        instructions::bge::FUNCT3 => Ok(InstructionDecoded::Bge { rs1, rs2, imm }),
                        instructions::bltu::FUNCT3 => {
                            Ok(InstructionDecoded::Bltu { rs1, rs2, imm })
                        }
                        instructions::bgeu::FUNCT3 => {
                            Ok(InstructionDecoded::Bgeu { rs1, rs2, imm })
                        }
                        _ => Err(format!("Unknown funct3 value for BType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                    },
                    _ => Err(format!("Unknown opcode for BType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                }
            }
            InstructionFormat::JType => {
                let rd = self.rd().unwrap();
                let imm = self.immediate1().unwrap()
                    | self.immediate2().unwrap()
                    | self.immediate3().unwrap()
                    | self.immediate4().unwrap();
                match self.opcode() {
                    instructions::JUMP_MATCH => Ok(InstructionDecoded::Jal { rd, imm }),
                    _ => Err(format!("Unknown opcode for JType instruction => {:#X}( 0b{:032b} )", self.inner(), self.inner())),
                }
            }
        }
    }
}

impl TryFrom<InstructionSize> for Instruction {
    type Error = String;
    fn try_from(value: InstructionSize) -> Result<Self, Self::Error> {
        let format = InstructionFormat::try_from(value)?;
        Ok(Self::make(value, format))
    }
}

impl TryFrom<InstructionSize> for InstructionFormat {
    type Error = String;
    fn try_from(value: InstructionSize) -> Result<Self, Self::Error> {
        match value & OPCODE_MASK {
            instructions::ARITMETIC_IMMEDIATE_MATCH => Ok(InstructionFormat::IType),
            instructions::ARITMETIC_REGISTER_MATCH => Ok(InstructionFormat::RType),
            instructions::STORE_MATCH => Ok(InstructionFormat::SType),
            instructions::LUI_MATCH => Ok(InstructionFormat::UType),
            instructions::AUIPC_MATCH => Ok(InstructionFormat::UType),
            instructions::LOAD_MATCH => Ok(InstructionFormat::IType),
            instructions::FENCE_MATCH => Ok(InstructionFormat::IType),
            instructions::BRANCH_MATCH => Ok(InstructionFormat::BType),
            instructions::JUMP_MATCH => Ok(InstructionFormat::JType),
            instructions::CSR_MATCH => Ok(InstructionFormat::IType),
            v => Err(format!(
                "Unknown InstructionFormat for opcode {:#X} (value = {:#X})",
                v, value
            )),
        }
    }
}

#[allow(dead_code)]
pub mod instructions {
    use super::{Instruction, InstructionBuilder, InstructionFormat, InstructionSize};

    pub const LOAD_MATCH: InstructionSize = 3;
    pub const FENCE_MATCH: InstructionSize = 15;
    pub const ARITMETIC_IMMEDIATE_MATCH: InstructionSize = 19;
    pub const AUIPC_MATCH: InstructionSize = 23;
    pub const LUI_MATCH: InstructionSize = 55;
    pub const STORE_MATCH: InstructionSize = 35;
    pub const ARITMETIC_REGISTER_MATCH: InstructionSize = 51;
    pub const BRANCH_MATCH: InstructionSize = 99;
    pub const JUMP_MATCH: InstructionSize = 103;
    pub const CSR_MATCH: InstructionSize = 115;
    pub const JALR_MATCH: InstructionSize = 103;

    #[derive(Debug)]
    pub struct ConstInstruction<
        const OPCODE: InstructionSize,
        const FUNCT3: InstructionSize,
        const FUNCT7: InstructionSize,
    >(InstructionFormat);

    impl<const O: InstructionSize, const F3: InstructionSize, const F7: InstructionSize>
        ConstInstruction<O, F3, F7>
    {
        pub const fn new(type_: InstructionFormat) -> Self {
            Self(type_)
        }

        pub fn encode<FN: FnOnce(InstructionBuilder) -> InstructionBuilder>(
            self,
            build: FN,
        ) -> Instruction {
            let builder = InstructionBuilder::builder()
                .opcode(O)
                .funct3(F3)
                .funct7(F7);
            let inst = build(builder).build();

            Instruction::make(inst, self.0)
        }

        pub fn to_inst(self) -> Instruction {
            let inst = InstructionBuilder::builder()
                .opcode(O)
                .funct3(F3)
                .funct7(F7)
                .build();
            Instruction::make(inst, self.0)
        }

        pub const fn to_inner(self) -> InstructionSize {
            let inst = InstructionBuilder::builder()
                .opcode(O)
                .funct3(F3)
                .funct7(F7)
                .build();
            Instruction::make(inst, self.0).to_inner()
        }

        pub fn opcode(self) -> InstructionSize {
            O
        }
        pub fn funct3(self) -> InstructionSize {
            F3
        }
        pub fn funct7(self) -> InstructionSize {
            F7
        }
    }

    macro_rules! instruction {
        ($name:ident => $name_upper:ident($opcode:expr, $f3:expr, $f7:expr)[$ty:expr]) => {
            pub const $name_upper: ConstInstruction<$opcode, $f3, $f7> = ConstInstruction::new($ty);
            pub mod $name {
                use super::*;
                pub const INST_BASE: InstructionSize = InstructionBuilder::builder()
                    .opcode(OPCODE)
                    .funct3(FUNCT3)
                    .funct7(FUNCT7)
                    .build();
                pub const OPCODE: InstructionSize = $opcode;
                pub const FUNCT3: InstructionSize = $f3;
                pub const FUNCT7: InstructionSize = $f7;
            }
        };
        ($name:ident => $name_upper:ident($opcode:expr, $f3:expr, $f7:expr)[$ty:expr] { $($b:tt)* }) => {
            pub const $name_upper: ConstInstruction<$opcode, $f3, $f7> = ConstInstruction::new($ty);
            pub mod $name {
                use super::*;
                pub const INST_BASE: InstructionSize = InstructionBuilder::builder()
                    .opcode(OPCODE)
                    .funct3(FUNCT3)
                    .funct7(FUNCT7)
                    .build();
                pub const OPCODE: InstructionSize = $opcode;
                pub const FUNCT3: InstructionSize = $f3;
                pub const FUNCT7: InstructionSize = $f7;

                $($b)*
            }
        };
    }

    instruction!(lb => LB(LOAD_MATCH, 0, 0)[InstructionFormat::IType]);
    instruction!(lh => LH(LOAD_MATCH, 1, 0)[InstructionFormat::IType]);
    instruction!(lw => LW(LOAD_MATCH, 2, 0)[InstructionFormat::IType]);
    instruction!(ld => LD(LOAD_MATCH, 3, 0)[InstructionFormat::IType]);
    instruction!(lbu => LBU(LOAD_MATCH, 4, 0)[InstructionFormat::IType]);
    instruction!(lhu => LHU(LOAD_MATCH, 5, 0)[InstructionFormat::IType]);
    instruction!(lwu => LWU(LOAD_MATCH, 6, 0)[InstructionFormat::IType]);
    instruction!(addi => ADDI(ARITMETIC_IMMEDIATE_MATCH, 0, 0)[InstructionFormat::IType]);
    instruction!(slli => SLLI(ARITMETIC_IMMEDIATE_MATCH, 1, 0)[InstructionFormat::IType]);
    instruction!(slti => SLTI(ARITMETIC_IMMEDIATE_MATCH, 2, 0)[InstructionFormat::IType]);
    instruction!(sltiu => SLTIU(ARITMETIC_IMMEDIATE_MATCH, 3, 0)[InstructionFormat::IType]);
    instruction!(xori => XORI(ARITMETIC_IMMEDIATE_MATCH, 4, 0)[InstructionFormat::IType]);
    instruction!(srli => SRLI(ARITMETIC_IMMEDIATE_MATCH, 5, 0)[InstructionFormat::IType]);
    instruction!(srai => SRAI(ARITMETIC_IMMEDIATE_MATCH, 5, 32)[InstructionFormat::IType]);
    instruction!(ori => ORI(ARITMETIC_IMMEDIATE_MATCH, 6, 0)[InstructionFormat::IType]);
    instruction!(andi => ANDI(ARITMETIC_IMMEDIATE_MATCH, 7, 0)[InstructionFormat::IType]);
    instruction!(auipc => AUIPC(AUIPC_MATCH, 0, 0)[InstructionFormat::UType]);
    instruction!(sb => SB(STORE_MATCH, 0, 0)[InstructionFormat::SType]);
    instruction!(sh => SH(STORE_MATCH, 1, 0)[InstructionFormat::SType]);
    instruction!(sw => SW(STORE_MATCH, 2, 0)[InstructionFormat::SType]);
    instruction!(sd => SD(STORE_MATCH, 3, 0)[InstructionFormat::SType]);
    instruction!(add => ADD(ARITMETIC_REGISTER_MATCH, 0, 0)[InstructionFormat::RType]);
    instruction!(sub => SUB(ARITMETIC_REGISTER_MATCH, 0, 32)[InstructionFormat::RType]);
    instruction!(sll => SLL(ARITMETIC_REGISTER_MATCH, 1, 0)[InstructionFormat::RType]);
    instruction!(slt => SLT(ARITMETIC_REGISTER_MATCH, 2, 0)[InstructionFormat::RType]);
    instruction!(sltu => SLTU(ARITMETIC_REGISTER_MATCH, 3, 0)[InstructionFormat::RType]);
    instruction!(xor => XOR(ARITMETIC_REGISTER_MATCH, 4, 0)[InstructionFormat::RType]);
    instruction!(srl => SRL(ARITMETIC_REGISTER_MATCH, 5, 0)[InstructionFormat::RType]);
    instruction!(sra => SRA(ARITMETIC_REGISTER_MATCH, 5, 32)[InstructionFormat::RType]);
    instruction!(or => OR(ARITMETIC_REGISTER_MATCH, 6, 0)[InstructionFormat::RType]);
    instruction!(and => AND(ARITMETIC_REGISTER_MATCH, 7, 0)[InstructionFormat::RType]);
    instruction!(lui => LUI(LUI_MATCH, 0, 0)[InstructionFormat::UType]);
    instruction!(addw => ADDW(ARITMETIC_REGISTER_MATCH, 0, 0)[InstructionFormat::RType]);
    instruction!(subw => SUBW(ARITMETIC_REGISTER_MATCH, 0, 32)[InstructionFormat::RType]);
    instruction!(sllw => SLLW(ARITMETIC_REGISTER_MATCH, 1, 0)[InstructionFormat::RType]);
    instruction!(srlw => SRLW(ARITMETIC_REGISTER_MATCH, 5, 0)[InstructionFormat::RType]);
    instruction!(sraw => SRAW(ARITMETIC_REGISTER_MATCH, 5, 32)[InstructionFormat::RType]);
    instruction!(beq => BEQ(BRANCH_MATCH, 0, 0)[InstructionFormat::SType]);
    instruction!(bne => BNE(BRANCH_MATCH, 1, 0)[InstructionFormat::SType]);
    instruction!(blt => BLT(BRANCH_MATCH, 4, 0)[InstructionFormat::SType]);
    instruction!(bge => BGE(BRANCH_MATCH, 5, 0)[InstructionFormat::SType]);
    instruction!(bltu => BLTU(BRANCH_MATCH, 6, 0)[InstructionFormat::SType]);
    instruction!(bgeu => BGEU(BRANCH_MATCH, 7, 0)[InstructionFormat::SType]);
    instruction!(jalr => JALR(JUMP_MATCH, 0, 0)[InstructionFormat::IType]);
    instruction!(jal => JAL(JUMP_MATCH, 0, 0)[InstructionFormat::JType]);

    instruction!(ecall => ECALL(CSR_MATCH, 0, 0)[InstructionFormat::IType]);
    instruction!(ebreak => EBREAK(CSR_MATCH, 0, 1)[InstructionFormat::IType]);
    /* Why couldnt i find this in the RiscV ISA? ID```F*CKIN```K! */
    instruction!(sret => SRET(CSR_MATCH, 0, 2)[InstructionFormat::IType]);
    instruction!(mret => MRET(CSR_MATCH, 0, 3)[InstructionFormat::IType]);
    instruction!(sfence_vma => SFENCE_VMA(CSR_MATCH, 0, 9)[InstructionFormat::IType]);

    instruction!(csrrw => CSRRW(CSR_MATCH, 1, 0)[InstructionFormat::IType]);
    instruction!(csrrs => CSRRS(CSR_MATCH, 2, 0)[InstructionFormat::IType]);
    instruction!(csrrc => CSRRC(CSR_MATCH, 3, 0)[InstructionFormat::IType]);
    instruction!(csrrwi => CSRRWI(CSR_MATCH, 5, 0)[InstructionFormat::IType]);
    instruction!(csrrsi => CSRRSI(CSR_MATCH, 6, 0)[InstructionFormat::IType]);
    instruction!(csrrci => CSRRCI(CSR_MATCH, 7, 0)[InstructionFormat::IType]);

    instruction!(fence => FENCE(FENCE_MATCH, 0, 0)[InstructionFormat::IType]);
    instruction!(fence_i => FENCE_I(FENCE_MATCH, 1, 0)[InstructionFormat::IType]);

    // D Extension
    instruction!(flw => FLW(LOAD_MATCH, 2, 2)[InstructionFormat::IType]);
    instruction!(fld => FLD(LOAD_MATCH, 3, 3)[InstructionFormat::IType]);
    instruction!(fsw => FSW(STORE_MATCH, 2, 2)[InstructionFormat::SType]);
    instruction!(fsd => FSD(STORE_MATCH, 3, 3)[InstructionFormat::SType]);
    instruction!(fmadd_s => FMADD_S(ARITMETIC_REGISTER_MATCH, 0, 0)[InstructionFormat::RType]);
    instruction!(fmsub_s => FMSUB_S(ARITMETIC_REGISTER_MATCH, 0, 1)[InstructionFormat::RType]);
    instruction!(fnmsub_s => FNMSUB_S(ARITMETIC_REGISTER_MATCH, 0, 2)[InstructionFormat::RType]);
    instruction!(fnmadd_s => FNMADD_S(ARITMETIC_REGISTER_MATCH, 0, 3)[InstructionFormat::RType]);
    instruction!(fadd_s => FADD_S(ARITMETIC_REGISTER_MATCH, 0, 0)[InstructionFormat::RType]);
    instruction!(fsub_s => FSUB_S(ARITMETIC_REGISTER_MATCH, 0, 1)[InstructionFormat::RType]);
    instruction!(fmul_s => FMUL_S(ARITMETIC_REGISTER_MATCH, 0, 2)[InstructionFormat::RType]);
    instruction!(fdiv_s => FDIV_S(ARITMETIC_REGISTER_MATCH, 0, 3)[InstructionFormat::RType]);
    instruction!(fsqrt_s => FSQRT_S(ARITMETIC_REGISTER_MATCH, 0, 4)[InstructionFormat::RType]);
    instruction!(fsgnj_s => FSGNJ_S(ARITMETIC_REGISTER_MATCH, 0, 5)[InstructionFormat::RType]);
    instruction!(fsgnjn_s => FSGNJN_S(ARITMETIC_REGISTER_MATCH, 0, 6)[InstructionFormat::RType]);
    instruction!(fsgnjx_s => FSGNJX_S(ARITMETIC_REGISTER_MATCH, 0, 7)[InstructionFormat::RType]);
    instruction!(fmin_s => FMIN_S(ARITMETIC_REGISTER_MATCH, 0, 8)[InstructionFormat::RType]);
    instruction!(fmax_s => FMAX_S(ARITMETIC_REGISTER_MATCH, 0, 9)[InstructionFormat::RType]);
    instruction!(fcvt_w_s => FCVT_W_S(ARITMETIC_REGISTER_MATCH, 0, 10)[InstructionFormat::RType]);
    instruction!(fcvt_wu_s => FCVT_WU_S(ARITMETIC_REGISTER_MATCH, 0, 11)[InstructionFormat::RType]);
    instruction!(fmv_x_w => FMV_X_W(ARITMETIC_REGISTER_MATCH, 0, 12)[InstructionFormat::RType]);
    instruction!(feq_s => FEQ_S(ARITMETIC_REGISTER_MATCH, 0, 13)[InstructionFormat::RType]);
    instruction!(flt_s => FLT_S(ARITMETIC_REGISTER_MATCH, 0, 14)[InstructionFormat::RType]);
    instruction!(fle_s => FLE_S(ARITMETIC_REGISTER_MATCH, 0, 15)[InstructionFormat::RType]);
    instruction!(fclass_s => FCLASS(ARITMETIC_REGISTER_MATCH, 0, 16)[InstructionFormat::RType]);

    // M Extension
    instruction!(mul => MUL(ARITMETIC_REGISTER_MATCH, 0, 1)[InstructionFormat::RType]);
    instruction!(mulh => MULH(ARITMETIC_REGISTER_MATCH, 1, 1)[InstructionFormat::RType]);
    instruction!(mulsu => MULSU(ARITMETIC_REGISTER_MATCH, 2, 1)[InstructionFormat::RType]);
    instruction!(mulu => MULU(ARITMETIC_REGISTER_MATCH, 3, 1)[InstructionFormat::RType]);
    instruction!(div => DIV(ARITMETIC_REGISTER_MATCH, 4, 1)[InstructionFormat::RType]);
    instruction!(divu => DIVU(ARITMETIC_REGISTER_MATCH, 5, 1)[InstructionFormat::RType]);
    instruction!(rem => REM(ARITMETIC_REGISTER_MATCH, 6, 1)[InstructionFormat::RType]);
    instruction!(remu => REMU(ARITMETIC_REGISTER_MATCH, 7, 1)[InstructionFormat::RType]);

    // A Extension
    instruction!(lrw => LRW(ARITMETIC_REGISTER_MATCH, 2, 8 /* its 8(0b0001000) b/c its the funct7 value rsht by 2 (first 2 bits are the rl, and aq) */)[InstructionFormat::RType] {
        pub const FUNCT5: InstructionSize = FUNCT7 >> 2;
    });
    instruction!(scw => SCW(ARITMETIC_REGISTER_MATCH, 2, 12)[InstructionFormat::RType] {
        pub const FUNCT5: InstructionSize = FUNCT7 >> 2;
    });
    instruction!(amoswapw => AMOSWAPW(ARITMETIC_REGISTER_MATCH, 2, 4)[InstructionFormat::RType] {
        pub const FUNCT5: InstructionSize = FUNCT7 >> 2;
    });
    instruction!(amoaddw => AMOADDW(ARITMETIC_REGISTER_MATCH, 2, 0)[InstructionFormat::RType] {
        pub const FUNCT5: InstructionSize = FUNCT7 >> 2;
    });
    instruction!(amoxorw => AMOXORW(ARITMETIC_REGISTER_MATCH, 2, 6)[InstructionFormat::RType] {
        pub const FUNCT5: InstructionSize = FUNCT7 >> 2;
    });
}

impl Instruction {
    pub const fn to_inner(self) -> InstructionSize {
        self.inst
    }

    pub fn inner(&self) -> InstructionSize {
        self.inst
    }

    pub fn opcode(&self) -> InstructionSize {
        (self.inst & OPCODE_MASK) >> 0
    }

    pub fn rd(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::RType => Some((self.inst & rtype::RD_MASK) >> 7),
            InstructionFormat::IType => Some((self.inst & itype::RD_MASK) >> 7),
            InstructionFormat::UType => Some((self.inst & utype::RD_MASK) >> 7),
            InstructionFormat::JType => Some((self.inst & jtype::RD_MASK) >> 7),
            _ => None,
        }
    }

    pub fn funct3(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::RType => Some((self.inst & rtype::FUNCT3_MASK) >> 12),
            InstructionFormat::IType => Some((self.inst & itype::FUNCT3_MASK) >> 12),
            InstructionFormat::SType => Some((self.inst & stype::FUNCT3_MASK) >> 12),
            InstructionFormat::BType => Some((self.inst & btype::FUNCT3_MASK) >> 12),
            _ => None,
        }
    }

    pub fn funct7(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::RType => Some((self.inst & rtype::FUNCT7_MASK) >> 25),
            _ => None,
        }
    }

    pub fn rs1(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::RType => Some((self.inst & rtype::RS1_MASK) >> 15),
            InstructionFormat::IType => Some((self.inst & itype::RS1_MASK) >> 15),
            InstructionFormat::SType => Some((self.inst & stype::RS1_MASK) >> 15),
            InstructionFormat::BType => Some((self.inst & btype::RS1_MASK) >> 15),
            _ => None,
        }
    }

    pub fn rs2(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::RType => Some((self.inst & rtype::RS2_MASK) >> 20),
            InstructionFormat::SType => Some((self.inst & stype::RS2_MASK) >> 20),
            InstructionFormat::BType => Some((self.inst & btype::RS2_MASK) >> 20),
            _ => None,
        }
    }

    pub fn aq(&self) -> Option<InstructionSize> {
        Some((self.funct7().unwrap() & 0b0000010) >> 1)
    }
    pub fn rl(&self) -> Option<InstructionSize> {
        Some(self.funct7().unwrap() & 0b0000001)
    }

    pub fn immediate1(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::IType => {
                Some(((self.inst as i32 & itype::IMM1 as i32) >> 20) as InstructionSize)
            }
            InstructionFormat::SType => {
                Some(((self.inst as i32 & stype::IMM1 as i32) >> 7) as InstructionSize)
            }
            InstructionFormat::UType => {
                Some(((self.inst as i32 & utype::IMM1 as i32) >> 12) as InstructionSize)
            }
            InstructionFormat::JType => {
                Some(((self.inst as i32 & jtype::IMM1 as i32) >> 12) as InstructionSize)
            }
            InstructionFormat::BType => {
                Some(((self.inst as i32 & btype::IMM1 as i32) >> 7) as InstructionSize)
            }
            _ => None,
        }
    }

    pub fn immediate2(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::SType => {
                Some(((self.inst as i32 & stype::IMM2 as i32) >> 25) as InstructionSize)
            }
            InstructionFormat::BType => {
                Some(((self.inst as i32 & btype::IMM2 as i32) >> 25) as InstructionSize)
            }
            InstructionFormat::JType => {
                Some(((self.inst as i32 & jtype::IMM2 as i32) >> 20) as InstructionSize)
            }
            _ => None,
        }
    }

    pub fn immediate3(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::JType => {
                Some(((self.inst as i32 & jtype::IMM3 as i32) >> 21) as InstructionSize)
            }
            _ => None,
        }
    }

    pub fn immediate4(&self) -> Option<InstructionSize> {
        match self.format {
            InstructionFormat::JType => {
                Some(((self.inst as i32 & jtype::IMM4 as i32) >> 31) as InstructionSize)
            }
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Instruction {{ format: {:#?}, inst: {{\n\tdec: {inst}\n\thex: 0x{inst:X}\n\tbin: 0b{inst:032b}\n}}\n}}",
            self.format, inst = self.inst
        )
    }
}

#[test]
fn addi_instruction() {
    let instruction = Instruction::try_from(0x128293 /* addi t0, t0, 1 */).unwrap();
    match instruction.format {
        InstructionFormat::IType => (),
        _ => panic!("Instruction SHOULD BE AN ITYPE!"),
    }
    for i in instruction.to_string().lines() {
        println!("{i}");
    }
    assert_eq!(instruction.opcode(), 19);
    assert_eq!(instruction.rd().unwrap(), 5);
    assert_eq!(instruction.funct3().unwrap(), 0);
    assert_eq!(instruction.rs1().unwrap(), 5);
    assert_eq!(instruction.immediate1().unwrap(), 1);
}

#[test]
fn write_value_into_x1() {
    let instruction = Instruction::from(0x900093 /* addi x1, x0, 9 */);
    match instruction.format {
        InstructionFormat::IType => (),
        _ => panic!("Instruction SHOULD BE AN ITYPE!"),
    }
    for i in instruction.to_string().lines() {
        println!("{i}");
    }
    assert_eq!(instruction.opcode(), 19);
    assert_eq!(instruction.rd().unwrap(), 1);
    assert_eq!(instruction.funct3().unwrap(), 0);
    assert_eq!(instruction.rs1().unwrap(), 0);
    assert_eq!(instruction.immediate1().unwrap(), 9);

    use crate::cpu::Cpu;
    let mut cpu = Cpu::new();
    cpu.load_program(&[instruction])
        .expect("Failed to load Program");
    cpu.step().expect("Failed to step through program");

    // check that the register x1 has the value 9
    let x1 = *cpu.get_register(1).unwrap();
    assert_eq!(x1, 9);
}

#[test]
fn add_instruction() {
    let instruction = Instruction::try_from(0x00208233 /* add x4 x1 x2 */).unwrap();
    match instruction.format {
        InstructionFormat::RType => (),
        _ => panic!(
            "Instruction SHOULD BE AN RTYPE!\nInstead got {:?}",
            instruction.format
        ),
    }
    for i in instruction.to_string().lines() {
        println!("{i}");
    }
    assert_eq!(instruction.opcode(), 51);
    assert_eq!(instruction.rd().unwrap(), 4);
    assert_eq!(instruction.funct3().unwrap(), 0);
    assert_eq!(instruction.rs1().unwrap(), 1);
    assert_eq!(instruction.rs2().unwrap(), 2);
}

pub const OPCODE_MASK: InstructionSize = self::internal::create_mask(7);

pub mod rtype {
    use super::InstructionSize;

    pub const RD_MASK: InstructionSize = super::internal::create_mask(5) << 7;
    pub const FUNCT3_MASK: InstructionSize = super::internal::create_mask(3) << 12;
    pub const RS1_MASK: InstructionSize = super::internal::create_mask(5) << 15;
    pub const RS2_MASK: InstructionSize = super::internal::create_mask(5) << 20;
    pub const FUNCT7_MASK: InstructionSize = super::internal::create_mask(7) << 25;

    #[test]
    pub fn bit_masks() {
        use crate::instruction_sets::rv32i::OPCODE_MASK;
        println!("OPCODE_MASK = 0b{:032b}", OPCODE_MASK);
        println!("RD_MASK     = 0b{:032b}", RD_MASK);
        println!("FUNCT3_MASK = 0b{:032b}", FUNCT3_MASK);
        println!("RS1_MASK    = 0b{:032b}", RS1_MASK);
        println!("RS2_MASK    = 0b{:032b}", RS2_MASK);
        println!("FUNCT7_MASK = 0b{:032b}", FUNCT7_MASK);
        assert_eq!(OPCODE_MASK, 0b00000000000000000000000001111111);
        assert_eq!(RD_MASK, 0b00000000000000000000111110000000);
        assert_eq!(FUNCT3_MASK, 0b00000000000000000111000000000000);
        assert_eq!(RS1_MASK, 0b00000000000011111000000000000000);
        assert_eq!(RS2_MASK, 0b00000001111100000000000000000000);
        assert_eq!(FUNCT7_MASK, 0b11111110000000000000000000000000);
    }
}

pub mod itype {
    use super::InstructionSize;

    pub const RD_MASK: InstructionSize = super::internal::create_mask(5) << 7;
    pub const FUNCT3_MASK: InstructionSize = super::internal::create_mask(3) << 12;
    pub const RS1_MASK: InstructionSize = super::internal::create_mask(5) << 15;
    pub const IMM1: InstructionSize = super::internal::create_mask(12) << 20;

    #[test]
    pub fn bit_masks() {
        use crate::instruction_sets::rv32i::OPCODE_MASK;
        println!("OPCODE_MASK = 0b{:034b}", OPCODE_MASK);
        println!("RD_MASK     = 0b{:034b}", RD_MASK);
        println!("FUNCT3_MASK = 0b{:034b}", FUNCT3_MASK);
        println!("RS1_MASK    = 0b{:034b}", RS1_MASK);
        println!("IMM1        = 0b{:034b}", IMM1);
        assert_eq!(OPCODE_MASK, 0b00000000000000000000000001111111);
        assert_eq!(RD_MASK, 0b00000000000000000000111110000000);
        assert_eq!(FUNCT3_MASK, 0b00000000000000000111000000000000);
        assert_eq!(RS1_MASK, 0b00000000000011111000000000000000);
        assert_eq!(IMM1, 0b11111111111100000000000000000000);
    }
}

pub mod stype {
    use super::InstructionSize;

    pub const IMM1: InstructionSize = super::internal::create_mask(5) << 7;
    pub const FUNCT3_MASK: InstructionSize = super::internal::create_mask(3) << 12;
    pub const RS1_MASK: InstructionSize = super::internal::create_mask(5) << 15;
    pub const RS2_MASK: InstructionSize = super::internal::create_mask(5) << 20;
    pub const IMM2: InstructionSize = super::internal::create_mask(7) << 25;

    #[test]
    pub fn bit_masks() {
        use crate::instruction_sets::rv32i::OPCODE_MASK;
        println!("OPCODE_MASK = 0b{:034b}", OPCODE_MASK);
        println!("IMM1        = 0b{:034b}", IMM1);
        println!("FUNCT3_MASK = 0b{:034b}", FUNCT3_MASK);
        println!("RS1_MASK    = 0b{:034b}", RS1_MASK);
        println!("RS2_MASK    = 0b{:034b}", RS2_MASK);
        println!("IMM2        = 0b{:034b}", IMM2);
        assert_eq!(OPCODE_MASK, 0b00000000000000000000000001111111);
        assert_eq!(IMM1, 0b00000000000000000000111110000000);
        assert_eq!(FUNCT3_MASK, 0b00000000000000000111000000000000);
        assert_eq!(RS1_MASK, 0b00000000000011111000000000000000);
        assert_eq!(RS2_MASK, 0b00000001111100000000000000000000);
        assert_eq!(IMM2, 0b11111110000000000000000000000000);
    }
}

pub mod utype {
    use super::InstructionSize;

    pub const RD_MASK: InstructionSize = super::internal::create_mask(5) << 7;
    pub const IMM1: InstructionSize = super::internal::create_mask(20) << 12;

    #[test]
    pub fn bit_masks() {
        use crate::instruction_sets::rv32i::OPCODE_MASK;
        println!("OPCODE_MASK = 0b{:032b}", OPCODE_MASK);
        println!("RD_MASK     = 0b{:032b}", RD_MASK);
        println!("IMM1        = 0b{:032b}", IMM1);
        assert_eq!(OPCODE_MASK, 0b00000000000000000000000001111111);
        assert_eq!(RD_MASK,     0b00000000000000000000111110000000);
        assert_eq!(IMM1,        0b11111111111111111111000000000000);
    }
}

// aims to mimic `mm[12|10:5] rs2 rs1 funct3 imm[4:1|11] opcode B-type` in the RISC-V spec
pub mod btype {
    use super::InstructionSize;

    // should be imm[4:1|11] as it is in the spec
    pub const IMM1: InstructionSize = super::internal::create_mask(1) << 7;
    // the second part of the immediate (just after IMM1 and is the imm[4:1] portion)
    pub const IMM2: InstructionSize = super::internal::create_mask(4) << 8;
    // the funct3 portion of the instruction
    pub const FUNCT3_MASK: InstructionSize = super::internal::create_mask(3) << 12;
    // the first source register
    pub const RS1_MASK: InstructionSize = super::internal::create_mask(5) << 15;
    // the second source register
    pub const RS2_MASK: InstructionSize = super::internal::create_mask(5) << 20;
    // the imm[10:5] portion of the immediate
    pub const IMM3: InstructionSize = super::internal::create_mask(6) << 25;
    // the imm[12] portion of the immediate
    pub const IMM4: InstructionSize = super::internal::create_mask(1) << 31;

    #[test]
    pub fn bit_masks() {
        use crate::instruction_sets::rv32i::OPCODE_MASK;
        println!("OPCODE_MASK = 0b{:032b}", OPCODE_MASK);
        println!("IMM1        = 0b{:032b}", IMM1);
        println!("IMM2        = 0b{:032b}", IMM2);
        println!("FUNCT3_MASK = 0b{:032b}", FUNCT3_MASK);
        println!("RS1_MASK    = 0b{:032b}", RS1_MASK);
        println!("RS2_MASK    = 0b{:032b}", RS2_MASK);
        println!("IMM3        = 0b{:032b}", IMM3);
        println!("IMM4        = 0b{:032b}", IMM4);
        assert_eq!(OPCODE_MASK, 0b00000000000000000000000001111111);
        assert_eq!(IMM1, 0b00000000000000000000000010000000);
        assert_eq!(IMM2, 0b00000000000000000000111100000000);
        assert_eq!(FUNCT3_MASK, 0b00000000000000000111000000000000);
        assert_eq!(RS1_MASK, 0b00000000000011111000000000000000);
        assert_eq!(RS2_MASK, 0b00000001111100000000000000000000);
        assert_eq!(IMM3, 0b01111110000000000000000000000000);
        assert_eq!(IMM4, 0b10000000000000000000000000000000);
    }
}

pub mod jtype {
    use super::InstructionSize;

    pub const RD_MASK: InstructionSize = super::internal::create_mask(5) << 7;
    pub const IMM1: InstructionSize = super::internal::create_mask(7) << 12;
    pub const IMM2: InstructionSize = super::internal::create_mask(1) << 19;
    pub const IMM3: InstructionSize = super::internal::create_mask(11) << 20;
    pub const IMM4: InstructionSize = super::internal::create_mask(1) << 31;

    #[test]
    pub fn bit_masks() {
        use crate::instruction_sets::rv32i::OPCODE_MASK;
        println!("OPCODE_MASK = 0b{:032b}", OPCODE_MASK);
        println!("RD_MASK     = 0b{:032b}", RD_MASK);
        println!("IMM1        = 0b{:032b}", IMM1);
        println!("IMM2        = 0b{:032b}", IMM2);
        println!("IMM3        = 0b{:032b}", IMM3);
        println!("IMM4        = 0b{:032b}", IMM4);
        assert_eq!(OPCODE_MASK, 0b00000000000000000000000001111111);
        assert_eq!(RD_MASK, 0b00000000000000000000111110000000);
        assert_eq!(IMM1, 0b00000000000001111111000000000000);
        assert_eq!(IMM2, 0b00000000000010000000000000000000);
        assert_eq!(IMM3, 0b01111111111100000000000000000000);
        assert_eq!(IMM4, 0b10000000000000000000000000000000)
    }
}
