use compressed::is_compressed;

use super::instructions::InstructionDecoded;

mod internal {
    pub use bit_ops::bitops_u32::*;
    pub fn zero_extend(value: u32) -> u32 {
        clear_bit(value, 31)
    }
}

pub type InstructionSize = u32;
pub type SignedInstructionSize = i32;

// SHOULD ONLY BE USED TO GENERATE THE INSTRUCTION BASE / MASK (uses lots of const fn's and such to stay at comptime as much as possible)
struct InstructionBuilder {
    inst: InstructionSize,
}

impl InstructionBuilder {
    const fn builder() -> Self {
        Self { inst: 0 }
    }

    const fn build(self) -> InstructionSize {
        self.inst
    }

    const fn opcode(mut self, value: InstructionSize) -> Self {
        self.inst |= value;
        self
    }
    const fn funct3(mut self, value: InstructionSize) -> Self {
        self.inst |= value << 12;
        self
    }
    const fn funct7(mut self, value: InstructionSize) -> Self {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    UnknownInstruction(String),
    UnknownInstructionFormat(String),
    NotCompressedInstruction,
}

pub fn decode(inst: InstructionSize) -> Result<InstructionDecoded, DecodeError> {
    if is_compressed(inst) {
        try_decode_compressed(inst)
    } else {
        try_decode(inst)
    }
}

pub fn try_decode_compressed(inst: InstructionSize) -> Result<InstructionDecoded, DecodeError> {
    todo!()
}

pub fn try_decode(inst: InstructionSize) -> Result<InstructionDecoded, DecodeError> {
    const OPCODE_MASK: InstructionSize = 0b1111111;

    let fmt = match inst & OPCODE_MASK {
        instructions::ARITMETIC_REGISTER_MATCH =>  InstructionFormat::RType,
        instructions::STORE_MATCH =>               InstructionFormat::SType,
        instructions::BRANCH_MATCH =>              InstructionFormat::BType,
        instructions::JAL_MATCH =>                 InstructionFormat::JType,
        instructions::ARITMETIC_IMMEDIATE_MATCH |
        instructions::FENCE_MATCH |
        instructions::LOAD_MATCH |
        instructions::CSR_MATCH |
        instructions::JALR_MATCH =>                InstructionFormat::IType,
        instructions::LUI_MATCH |
        instructions::AUIPC_MATCH =>               InstructionFormat::UType,
        v => Err(DecodeError::UnknownInstructionFormat(format!("Unknown InstructionFormat for instruction: {:#X}({:#X})", inst, v)))?,
    };

    let inst = match fmt {
        InstructionFormat::RType => {
            let rinst = rtype::RType::new(inst);
            match (rinst.funct3(), rinst.funct7()) {
                (instructions::add::FUNCT3, instructions::add::FUNCT7) => InstructionDecoded::Add {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::sub::FUNCT3, instructions::sub::FUNCT7) => InstructionDecoded::Sub {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::sll::FUNCT3, instructions::sll::FUNCT7) => InstructionDecoded::Sll {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::slt::FUNCT3, instructions::slt::FUNCT7) => InstructionDecoded::Slt {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::sltu::FUNCT3, instructions::sltu::FUNCT7) => InstructionDecoded::Sltu {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::xor::FUNCT3, instructions::xor::FUNCT7) => InstructionDecoded::Xor {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::srl::FUNCT3, instructions::srl::FUNCT7) => InstructionDecoded::Srl {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::sra::FUNCT3, instructions::sra::FUNCT7) => InstructionDecoded::Sra {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::or::FUNCT3, instructions::or::FUNCT7) => InstructionDecoded::Or {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                (instructions::and::FUNCT3, instructions::and::FUNCT7) => InstructionDecoded::And {
                    rd: rinst.rd(),
                    rs1: rinst.rs1(),
                    rs2: rinst.rs2(),
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown R-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::IType => {
            let iinst = itype::IType::new(inst);
            match (iinst.opcode(), iinst.funct3(), iinst.imm1() as InstructionSize) {
                (instructions::LOAD_MATCH, instructions::lb::FUNCT3, _) => InstructionDecoded::Lb {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::LOAD_MATCH, instructions::lh::FUNCT3, _) => InstructionDecoded::Lh {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::LOAD_MATCH, instructions::lw::FUNCT3, _) => InstructionDecoded::Lw {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::LOAD_MATCH, instructions::lbu::FUNCT3, _) => InstructionDecoded::Lbu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::LOAD_MATCH, instructions::lhu::FUNCT3, _) => InstructionDecoded::Lhu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::addi::FUNCT3, _) => InstructionDecoded::Addi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::slli::FUNCT3, _) => InstructionDecoded::Slli {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::slti::FUNCT3, _) => InstructionDecoded::Slti {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::sltiu::FUNCT3, _) => InstructionDecoded::Sltiu {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::xori::FUNCT3, _) => InstructionDecoded::Xori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::srli::FUNCT3, 0) => InstructionDecoded::Srli {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::srai::FUNCT3, 32) => InstructionDecoded::Srai {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::ori::FUNCT3, _) => InstructionDecoded::Ori {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::ARITMETIC_IMMEDIATE_MATCH, instructions::andi::FUNCT3, _) => InstructionDecoded::Andi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::JALR_MATCH, 0 /* f3 must be zero */, _) => InstructionDecoded::Jalr {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },

                (instructions::CSR_MATCH, instructions::csrrw::FUNCT3, _) => InstructionDecoded::CsrRw {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::CSR_MATCH, instructions::csrrs::FUNCT3, _) => InstructionDecoded::CsrRs {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::CSR_MATCH, instructions::csrrc::FUNCT3, _) => InstructionDecoded::CsrRc {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::CSR_MATCH, instructions::csrrwi::FUNCT3, _) => InstructionDecoded::CsrRwi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::CSR_MATCH, instructions::csrrsi::FUNCT3, _) => InstructionDecoded::CsrRsi {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },
                (instructions::CSR_MATCH, instructions::csrrci::FUNCT3, _) => InstructionDecoded::CsrRci {
                    rd: iinst.rd(),
                    rs1: iinst.rs1(),
                    imm: iinst.imm1() as InstructionSize,
                },

                (instructions::CSR_MATCH, instructions::ecall::FUNCT3, instructions::ecall::FUNCT7) => InstructionDecoded::ECall,
                (instructions::CSR_MATCH, instructions::ebreak::FUNCT3, instructions::ebreak::FUNCT7) => InstructionDecoded::EBreak,
                (instructions::CSR_MATCH, instructions::sret::FUNCT3, instructions::sret::FUNCT7) => InstructionDecoded::SRet,
                (instructions::CSR_MATCH, instructions::mret::FUNCT3, instructions::mret::FUNCT7) => InstructionDecoded::MRet,
                (instructions::CSR_MATCH, instructions::sfence_vma::FUNCT3, instructions::sfence_vma::FUNCT7) => InstructionDecoded::SFenceVma,
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown I-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::SType => {
            let sinst = stype::SType::new(inst);
            match sinst.funct3() {
                instructions::sb::FUNCT3 => InstructionDecoded::Sb {
                    rs1: sinst.rs1(),
                    rs2: sinst.rs2(),
                    imm: sinst.imm(),
                },
                instructions::sh::FUNCT3 => InstructionDecoded::Sh {
                    rs1: sinst.rs1(),
                    rs2: sinst.rs2(),
                    imm: sinst.imm(),
                },
                instructions::sw::FUNCT3 => InstructionDecoded::Sw {
                    rs1: sinst.rs1(),
                    rs2: sinst.rs2(),
                    imm: sinst.imm(),
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown S-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::UType => {
            let uinst = utype::UType::new(inst);
            match uinst.opcode() {
                instructions::lui::OPCODE => InstructionDecoded::Lui {
                    rd: uinst.rd(),
                    imm: uinst.imm1() as InstructionSize,
                },
                instructions::auipc::OPCODE => InstructionDecoded::AuiPc {
                    rd: uinst.rd(),
                    imm: uinst.imm1() as InstructionSize,
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown U-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::BType => {
            let binst = btype::BType::new(inst);
            match binst.funct3() {
                instructions::beq::FUNCT3 => InstructionDecoded::Beq {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                instructions::bne::FUNCT3 => InstructionDecoded::Bne {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                instructions::blt::FUNCT3 => InstructionDecoded::Blt {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                instructions::bge::FUNCT3 => InstructionDecoded::Bge {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                instructions::bltu::FUNCT3 => InstructionDecoded::Bltu {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                instructions::bgeu::FUNCT3 => InstructionDecoded::Bgeu {
                    rs1: binst.rs1(),
                    rs2: binst.rs2(),
                    imm: binst.imm1() as InstructionSize | binst.imm2() as InstructionSize,
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown B-Type instruction: {:#X}", inst)))?
            }
        }
        InstructionFormat::JType => {
            let jinst = jtype::JType::new(inst);
            match jinst.opcode() {
                instructions::jal::OPCODE => InstructionDecoded::Jal {
                    rd: jinst.rd(),
                    imm: jinst.imm() as InstructionSize,
                },
                _ => Err(DecodeError::UnknownInstruction(format!("Unknown J-Type instruction: {:#X}", inst)))?
            }
        }
    };

    Ok(inst)
}

pub mod compressed {
    use crate::instruction_sets::rv32i::InstructionSize;

    pub type CompressedSize = u16;

    pub fn is_compressed(inst: InstructionSize) -> bool {
        const COMPRESSED_MASK: CompressedSize = 0b11;
        match (inst & 0xFFFF) as u16 & COMPRESSED_MASK {
            0 | 1 | 2 => true,
            _ => false,
        }
    }

    pub mod crtype {
        use super::CompressedSize;
        use bitfield::bitfield;

        bitfield! {
            pub struct CRType(CompressedSize);
            impl Debug;
            pub opcode, _: 1, 0;
            rs2, _: 6, 2; // must be 0
            rs1, _: 11, 7; // rs1 != 0
            pub funct4, _: 15, 12;
        }

        impl CRType {
            pub fn new(inst: CompressedSize) -> Self {
                Self(inst)
            }
        }

        #[test]
        fn crtype() {
            let inst = CRType(0x8602 /* c.jr x12 */);
            assert_eq!(inst.opcode(), 2);
            assert_eq!(inst.funct4(), 8);
            assert_eq!(inst.rs1(), 12);
            assert_eq!(inst.rs2(), 0);
        }
    }

    pub mod csstype {
        // TODO: Implement compressed S-Type
    }

    pub mod cwitype {
        // TODO: Implement compressed W-Type
    }

    pub mod citype {
        // TODO: Implement compressed I-Type
    }

    pub mod cjtype {
        // TODO: Implement compressed J-Type
    }

    pub mod cbtype {
        // TODO: Implement compressed B-Type
    }

    pub mod cltype {
        // TODO: Implement compressed L-Type
    }

    pub mod cstype {
        // TODO: Implement cs-type
    }
}

#[allow(dead_code)]
pub mod instructions {
    use super::{InstructionSize, InstructionBuilder};

    pub const LOAD_MATCH: InstructionSize = 3;
    pub const FENCE_MATCH: InstructionSize = 15;
    pub const ARITMETIC_IMMEDIATE_MATCH: InstructionSize = 19;
    pub const AUIPC_MATCH: InstructionSize = 23;
    pub const LUI_MATCH: InstructionSize = 55;
    pub const STORE_MATCH: InstructionSize = 35;
    pub const ARITMETIC_REGISTER_MATCH: InstructionSize = 51;
    pub const BRANCH_MATCH: InstructionSize = 99;
    pub const CSR_MATCH: InstructionSize = 115;
    pub const JALR_MATCH: InstructionSize = 103;
    pub const JAL_MATCH: InstructionSize = 111;
    pub const ATOMIC_MATCH: InstructionSize = 47;

    macro_rules! instruction {
        ($name:ident => $name_upper:ident($opcode:expr, $f3:expr, $f7:expr)[$ty:expr]) => {
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
    instruction!(lbu => LBU(LOAD_MATCH, 4, 0)[InstructionFormat::IType]);
    instruction!(lhu => LHU(LOAD_MATCH, 5, 0)[InstructionFormat::IType]);
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
    instruction!(jalr => JALR(JALR_MATCH, 0, 0)[InstructionFormat::IType]);
    instruction!(jal => JAL(JAL_MATCH, 0, 0)[InstructionFormat::JType]);

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

pub mod rtype {
    use bitfield::bitfield;
    use super::InstructionSize;

    bitfield! {
        pub struct RType(InstructionSize);
        impl Debug;
        InstructionSize;
        pub opcode, _: 6, 0;
        pub rd, _:     11, 7;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        pub rs2, _:    24, 20;
        pub funct7, _: 31, 25;
    }

    impl RType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }
    }
}

pub mod itype {
    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct IType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub rd, _:     11, 7;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        SignedInstructionSize;
        pub imm1, _:   31, 20;
    }

    impl IType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }
    }

    #[test]
    fn imm_check() {
        let inst = IType(0x06468613 /* addi x12 x13 100 */);
        assert_eq!(inst.rd(), 12);
        assert_eq!(inst.rs1(), 13);
        assert_eq!(inst.imm1(), 100);
    }

    #[test]
    fn instructions() {
        let inst = IType(0x00411573 /* csrrw x10 x2 4 */);
        assert_eq!(inst.rd(), 10);
        assert_eq!(inst.rs1(), 2);
        assert_eq!(inst.imm1(), 4);
        let inst = IType(0x00c12603 /* lw x12, 12(sp) */);
        assert_eq!(inst.rd(), 12);
        assert_eq!(inst.rs1(), 2);
        assert_eq!(inst.imm1(), 12);
        let inst = IType(0x00c080e7 /* jalr x1, 12(ra) */);
        assert_eq!(inst.rd(), 1);
        assert_eq!(inst.rs1(), 1);
        assert_eq!(inst.imm1(), 12);
    }
}

pub mod stype {
    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct SType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub imm1, _:   11, 7;
        InstructionSize;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        pub rs2, _:    24, 20;
        SignedInstructionSize;
        pub imm2, _:   31, 25;
    }

    impl SType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        pub fn imm(&self) -> InstructionSize {
            self.imm1() | (self.imm2() << 5) as InstructionSize
        }
    }

    #[test]
    fn imm_check() {
        let inst = SType(0x00112f23 /* sw ra, 30(sp) */);
        assert_eq!(inst.rs1(), 2);
        assert_eq!(inst.rs2(), 1);
        assert_eq!(inst.imm(), 30);
    }
}

pub mod utype {
    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct UType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub rd, _:     11, 7;
        SignedInstructionSize;
        pub imm1, _:   31, 12;
    }

    impl UType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }
    }
}

// aims to mimic `mm[12|10:5] rs2 rs1 funct3 imm[4:1|11] opcode B-type` in the RISC-V spec
pub mod btype {
    use super::{InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct BType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub imm1, _:   11, 7;
        InstructionSize;
        pub funct3, _: 14, 12;
        pub rs1, _:    19, 15;
        pub rs2, _:    24, 20;
        SignedInstructionSize;
        pub imm2, _:   31, 25;
    }

    impl BType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        pub fn imm(&self) -> InstructionSize {
            self.imm1() | (self.imm2() << 5) as InstructionSize
        }
    }

    #[test]
    fn imm_check() {
        let inst = BType(0x50A60463 /* beq x12 x10 1288 */);
        assert_eq!(inst.rs1(), 12);
        assert_eq!(inst.rs2(), 10);
        assert_eq!(inst.imm(), 1288);
        let inst = BType(0x00409663 /* bne x1 x4 12 */);
        assert_eq!(inst.rs1(), 1);
        assert_eq!(inst.rs2(), 4);
        assert_eq!(inst.imm(), 12);
    }
}

pub mod jtype {
    use super::{internal, InstructionSize, SignedInstructionSize};
    use bitfield::bitfield;

    bitfield! {
        pub struct JType(InstructionSize);
        impl Debug;
        pub opcode, _: 6, 0;
        pub rd, _: 11, 7;
    }

    impl JType {
        pub fn new(inst: InstructionSize) -> Self {
            Self(inst)
        }

        fn imm1(&self) -> InstructionSize {
            let imm = (internal::get_bit(self.0, 31) << 31) as SignedInstructionSize;
            (imm >> 11) as InstructionSize
        }

        fn imm2(&self) -> InstructionSize {
            internal::get_bits(self.0, 8, 12) << 12
        }

        fn imm3(&self) -> InstructionSize {
            let imm = self.0 >> 9 /* now get bit 11 */;
            internal::get_bit(imm, 11) << 11
        }

        fn imm4(&self) -> InstructionSize {
            let imm = self.0 >> 20 /* now get bits 10:1 */;
            internal::get_bits(imm, 10, 1) << 1
        }

        pub fn imm(&self) -> InstructionSize {
            let (imm1, imm2, imm3, imm4) = (
                self.imm1(), // imm[20]
                self.imm2(), // imm[19:12]
                self.imm3(), // imm[11]
                self.imm4() // imm[10:1]
            );
            imm1 | imm2 | imm3 | imm4
        }
    }

    #[test]
    fn imm_check() {
        let inst = JType(0x0100006f /* jal x0 16 */);
        assert_eq!(inst.rd(), 0);
        assert_eq!(inst.imm(), 16);
        let inst = JType(0x84000EF /* JAL ra 132 (0b00001000010000000000000011101111) */);
        assert_eq!(inst.rd(), 1);
        assert_eq!(inst.imm(), 132);
        let inst = JType(0xfb9ff0ef /* jal ra, -72 */);
        assert_eq!(inst.rd(), 1);
        assert_eq!(inst.imm() as SignedInstructionSize, -72);
    }
}
