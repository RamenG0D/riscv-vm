use super::rv32i::InstructionSize;

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
}
