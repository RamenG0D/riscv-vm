use std::fmt::Display;

use super::rv32i::InstructionSize;

#[derive(Debug)]
pub enum InstructionDecoded {
    Lb {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lh {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lw {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lbu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lhu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Lwu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Addi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Slli {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Slti {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Sltiu {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Xori {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Srli {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Srai {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Ori {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Andi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    AuiPc {
        rd: InstructionSize,
        imm: InstructionSize,
    },
    Sb {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Sh {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Sw {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Add {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sub {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sll {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Slt {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sltu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Xor {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Srl {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Sra {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Or {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    And {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Lui {
        rd: InstructionSize,
        imm: InstructionSize,
    },
    Beq {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bne {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Blt {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bge {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bltu {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Bgeu {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    Jalr {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Jal {
        rd: InstructionSize,
        imm: InstructionSize,
    },

    ECall,
    EBreak,
    SRet,
    MRet,
    SFenceVma,

    CsrRw {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRs {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRc {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRwi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRsi {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    CsrRci {
        rd: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },

    Fence {
        rd: InstructionSize,
        rs1: InstructionSize,
        fm: InstructionSize,
        pred: InstructionSize,
        succ: InstructionSize,
    },
    FenceI {
        rd: InstructionSize,
        rs1: InstructionSize,
        fm: InstructionSize,
        pred: InstructionSize,
        succ: InstructionSize,
    },

    // D Extension (floats)
    Flw {
        rd: InstructionSize,
        width: InstructionSize,
        rs1: InstructionSize,
        imm: InstructionSize,
    },
    Fsw {
        rs1: InstructionSize,
        rs2: InstructionSize,
        imm: InstructionSize,
    },
    FmaddS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FmsubS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FnmaddS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FnmsubS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rs3: InstructionSize,
    },
    FaddS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsubS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FmulS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FdivS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsqrtS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FsgnjS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsgnjnS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FsgnjxS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FminS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FmaxS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FcvtSW {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtSWU {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtWS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FcvtWUS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FmvXW {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FmvWX {
        rd: InstructionSize,
        rs1: InstructionSize,
    },
    FeqS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FltS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FleS {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    FClassS {
        rd: InstructionSize,
        rs1: InstructionSize,
    },

    // M Extension
    Mul {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Mulh {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Mulsu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Mulu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Div {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Divu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Rem {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },
    Remu {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
    },

    // A Extension
    LrW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    ScW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    AmoswapW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    AmoaddW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    AmoandW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    AmoorW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    AmoxorW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    AmomaxW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },
    AmominW {
        rd: InstructionSize,
        rs1: InstructionSize,
        rs2: InstructionSize,
        rl: InstructionSize,
        aq: InstructionSize,
    },

    // Compressed Instructions
    CAddi4Spn {
        rd: InstructionSize,
        nzuimm: InstructionSize,
    },
    CNop,
    CSlli {
        rd: InstructionSize,
        rs1: InstructionSize,
        shamt: InstructionSize,
    }
}

impl Display for InstructionDecoded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const REG_NAMES: [&str; 32] = [
            "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
            "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
            "t3", "t4", "t5", "t6",
        ];

        match self {
            InstructionDecoded::Lb { rd, rs1, imm } => {
                write!(f, "lb {}, {}({})", REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Lh { rd, rs1, imm } => {
                write!(f, "lh {}, {}({})", REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Lw { rd, rs1, imm } => {
                write!(f, "lw {}, {}({})", REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Lbu { rd, rs1, imm } => {
                write!(f, "lbu {}, {}({})", REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Lhu { rd, rs1, imm } => {
                write!(f, "lhu {}, {}({})", REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Lwu { rd, rs1, imm } => {
                write!(f, "lwu {}, {}({})", REG_NAMES[*rd as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Addi { rd, rs1, imm } => {
                write!(f, "addi {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Slli { rd, rs1, imm } => {
                write!(f, "slli {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Slti { rd, rs1, imm } => {
                write!(f, "slti {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Sltiu { rd, rs1, imm } => {
                write!(f, "sltiu {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Xori { rd, rs1, imm } => {
                write!(f, "xori {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Srli { rd, rs1, imm } => {
                write!(f, "srli {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Srai { rd, rs1, imm } => {
                write!(f, "srai {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Ori { rd, rs1, imm } => {
                write!(f, "ori {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Andi { rd, rs1, imm } => {
                write!(f, "andi {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::AuiPc { rd, imm } => {
                write!(f, "auipc {}, {}", REG_NAMES[*rd as usize], *imm as i32)
            }
            InstructionDecoded::Sb { rs1, rs2, imm } => {
                write!(f, "sb {}, {}({})", REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Sh { rs1, rs2, imm } => {
                write!(f, "sh {}, {}({})", REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Sw { rs1, rs2, imm } => {
                write!(f, "sw {}, {}({})", REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Add { rd, rs1, rs2 } => {
                write!(f, "add {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Sub { rd, rs1, rs2 } => {
                write!(f, "sub {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Sll { rd, rs1, rs2 } => {
                write!(f, "sll {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Slt { rd, rs1, rs2 } => {
                write!(f, "slt {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Sltu { rd, rs1, rs2 } => {
                write!(f, "sltu {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Xor { rd, rs1, rs2 } => {
                write!(f, "xor {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Srl { rd, rs1, rs2 } => {
                write!(f, "srl {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Sra { rd, rs1, rs2 } => {
                write!(f, "sra {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Or { rd, rs1, rs2 } => {
                write!(f, "or {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::And { rd, rs1, rs2 } => {
                write!(f, "and {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Lui { rd, imm } => {
                write!(f, "lui {}, {}", REG_NAMES[*rd as usize], *imm as i32)
            }
            InstructionDecoded::Beq { rs1, rs2, imm } => {
                write!(f, "beq {}, {}, {}", REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32)
            }
            InstructionDecoded::Bne { rs1, rs2, imm } => {
                write!(f, "bne {}, {}, {}", REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32)
            }
            InstructionDecoded::Blt { rs1, rs2, imm } => {
                write!(f, "blt {}, {}, {}", REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32)
            }
            InstructionDecoded::Bge { rs1, rs2, imm } => {
                write!(f, "bge {}, {}, {}", REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32)
            }
            InstructionDecoded::Bltu { rs1, rs2, imm } => {
                write!(f, "bltu {}, {}, {}", REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32)
            }
            InstructionDecoded::Bgeu { rs1, rs2, imm } => {
                write!(f, "bgeu {}, {}, {}", REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *imm as i32)
            }
            InstructionDecoded::Jalr { rd, rs1, imm } => {
                write!(f, "jalr {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Jal { rd, imm } => {
                write!(f, "jal {}, {}", REG_NAMES[*rd as usize], *imm as i32)
            }
            InstructionDecoded::ECall => {
                write!(f, "ecall")
            }
            InstructionDecoded::EBreak => {
                write!(f, "ebreak")
            }
            InstructionDecoded::SRet => {
                write!(f, "sret")
            }
            InstructionDecoded::MRet => {
                write!(f, "mret")
            }
            InstructionDecoded::SFenceVma => {
                write!(f, "sfence.vma")
            }
            InstructionDecoded::CsrRw { rd, rs1, imm } => {
                write!(f, "csrrw {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::CsrRs { rd, rs1, imm } => {
                write!(f, "csrrs {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::CsrRc { rd, rs1, imm } => {
                write!(f, "csrrc {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::CsrRwi { rd, rs1, imm } => {
                write!(f, "csrrwi {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::CsrRsi { rd, rs1, imm } => {
                write!(f, "csrrsi {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::CsrRci { rd, rs1, imm } => {
                write!(f, "csrrci {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Fence { rd, rs1, fm, pred, succ } => {
                write!(f, "fence {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *fm as i32, *pred as i32, *succ as i32)
            }
            InstructionDecoded::FenceI { rd, rs1, fm, pred, succ } => {
                write!(f, "fence.i {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *fm as i32, *pred as i32, *succ as i32)
            }
            InstructionDecoded::Flw { rd, width, rs1, imm } => {
                write!(f, "flw {}, {}, {}({})", REG_NAMES[*rd as usize], *width as i32, REG_NAMES[*rs1 as usize], *imm as i32)
            }
            InstructionDecoded::Fsw { rs1, rs2, imm } => {
                write!(f, "fsw {}, {}({})", REG_NAMES[*rs2 as usize], *imm as i32, REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FmaddS { rd, rs1, rs2, rs3 } => {
                write!(f, "fmadd.s {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], REG_NAMES[*rs3 as usize])
            }
            InstructionDecoded::FmsubS { rd, rs1, rs2, rs3 } => {
                write!(f, "fmsub.s {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], REG_NAMES[*rs3 as usize])
            }
            InstructionDecoded::FnmaddS { rd, rs1, rs2, rs3 } => {
                write!(f, "fnmadd.s {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], REG_NAMES[*rs3 as usize])
            }
            InstructionDecoded::FnmsubS { rd, rs1, rs2, rs3 } => {
                write!(f, "fnmsub.s {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], REG_NAMES[*rs3 as usize])
            }
            InstructionDecoded::FaddS { rd, rs1, rs2 } => {
                write!(f, "fadd.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FsubS { rd, rs1, rs2 } => {
                write!(f, "fsub.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FmulS { rd, rs1, rs2 } => {
                write!(f, "fmul.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FdivS { rd, rs1, rs2 } => {
                write!(f, "fdiv.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FsqrtS { rd, rs1 } => {
                write!(f, "fsqrt.s {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FsgnjS { rd, rs1, rs2 } => {
                write!(f, "fsgnj.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FsgnjnS { rd, rs1, rs2 } => {
                write!(f, "fsgnjn.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FsgnjxS { rd, rs1, rs2 } => {
                write!(f, "fsgnjx.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FminS { rd, rs1, rs2 } => {
                write!(f, "fmin.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FmaxS { rd, rs1, rs2 } => {
                write!(f, "fmax.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FcvtSW { rd, rs1 } => {
                write!(f, "fcvt.s.w {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FcvtSWU { rd, rs1 } => {
                write!(f, "fcvt.s.wu {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FcvtWS { rd, rs1 } => {
                write!(f, "fcvt.w.s {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FcvtWUS { rd, rs1 } => {
                write!(f, "fcvt.wu.s {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FmvXW { rd, rs1 } => {
                write!(f, "fmv.x.w {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FmvWX { rd, rs1 } => {
                write!(f, "fmv.w.x {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::FeqS { rd, rs1, rs2 } => {
                write!(f, "feq.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FltS { rd, rs1, rs2 } => {
                write!(f, "flt.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FleS { rd, rs1, rs2 } => {
                write!(f, "fle.s {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::FClassS { rd, rs1 } => {
                write!(f, "fclass.s {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize])
            }
            InstructionDecoded::Mul { rd, rs1, rs2 } => {
                write!(f, "mul {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Mulh { rd, rs1, rs2 } => {
                write!(f, "mulh {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Mulsu { rd, rs1, rs2 } => {
                write!(f, "mulsu {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Mulu { rd, rs1, rs2 } => {
                write!(f, "mulu {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Div { rd, rs1, rs2 } => {
                write!(f, "div {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Divu { rd, rs1, rs2 } => {
                write!(f, "divu {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Rem { rd, rs1, rs2 } => {
                write!(f, "rem {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::Remu { rd, rs1, rs2 } => {
                write!(f, "remu {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize])
            }
            InstructionDecoded::LrW { rd, rs1, rs2, rl, aq } => {
                write!(f, "lr.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::ScW { rd, rs1, rs2, rl, aq } => {
                write!(f, "sc.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::AmoswapW { rd, rs1, rs2, rl, aq } => {
                write!(f, "amoswap.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::AmoaddW { rd, rs1, rs2, rl, aq } => {
                write!(f, "amoadd.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::AmoandW { rd, rs1, rs2, rl, aq } => {
                write!(f, "amoand.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::AmoorW { rd, rs1, rs2, rl, aq } => {
                write!(f, "amoor.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::AmoxorW { rd, rs1, rs2, rl, aq } => {
                write!(f, "amoxor.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::AmomaxW { rd, rs1, rs2, rl, aq } => {
                write!(f, "amomax.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::AmominW { rd, rs1, rs2, rl, aq } => {
                write!(f, "amomin.w {}, {}, {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], REG_NAMES[*rs2 as usize], *rl as i32, *aq as i32)
            }
            InstructionDecoded::CNop => {
                write!(f, "c.nop")
            }
            InstructionDecoded::CAddi4Spn { rd, nzuimm } => {
                write!(f, "c.addi4spn {}, {}", REG_NAMES[*rd as usize], *nzuimm as i32)
            }
            InstructionDecoded::CSlli { rd, rs1, shamt } => {
                write!(f, "c.slli {}, {}, {}", REG_NAMES[*rd as usize], REG_NAMES[*rs1 as usize], *shamt as i32)
            }
        }
    }
}
