//! The csr module contains all the control and status registers.

use std::ops::{Bound, Range, RangeBounds, RangeInclusive};

use bit_ops::BitOps;
use log::{info, trace};

pub type CsrAddress = u16;
pub type CsrFieldRange = RangeInclusive<u32>;

pub trait Csr {
    /// Read the val from the CSR.
    fn read(&self, addr: CsrAddress) -> u32;
    /// Write the val to the CSR.
    fn write(&mut self, addr: CsrAddress, val: u32);

    /// Read a bit from the CSR.
    fn read_bit(&self, addr: CsrAddress, bit: u32) -> u32 {
        if self.read(addr).get_bit(bit) != 0 {
            1
        } else {
            0
        }
    }

    /// Read a arbitrary length of bits from the CSR.
    fn read_bits<T: RangeBounds<u32>>(&self, addr: CsrAddress, range: T) -> u32 {
        let r = to_range(&range, MXLEN);
        self.read(addr).get_bits(r.end - r.start, r.start)
    }

    /// Write a bit to the CSR.
    fn write_bit(&mut self, addr: CsrAddress, bit: u32, val: u32) {
        let mut csr_val = self.read(addr);
        if val == 0 {
            csr_val = csr_val.clear_bit(bit);
        } else {
            csr_val = csr_val.set_bit(bit);
        }
        self.write(addr, csr_val);
    }

    /// Write an arbitrary length of bits to the CSR.
    fn write_bits<T: RangeBounds<u32>>(&mut self, addr: CsrAddress, range: T, val: u32) {
        let range = to_range(&range, MXLEN);
        let mut csr_val = self.read(addr);
        csr_val = csr_val.set_bits(val, range.end - range.start, range.start);
        self.write(addr, csr_val);
    }

    /// Read bit(s) from a given field in the SSTATUS register.
    fn read_sstatus(&self, range: CsrFieldRange) -> u32 {
        self.read_bits(SSTATUS, range)
    }

    /// Read bit(s) from a given field in the MSTATUS register.
    fn read_mstatus(&self, range: CsrFieldRange) -> u32 {
        self.read_bits(MSTATUS, range)
    }

    /// Write bit(s) to a given field in the SSTATUS register.
    fn write_sstatus(&mut self, range: CsrFieldRange, val: u32) {
        self.write_bits(SSTATUS, range, val);
    }

    /// Write bit(s) to a given field in the MSTATUS register.
    fn write_mstatus(&mut self, range: CsrFieldRange, val: u32) {
        self.write_bits(MSTATUS, range, val);
    }

    /// Reset all the CSRs.
    fn reset(&mut self);
}

const MXLEN: u32 = 32;
/// The number of CSRs. The field is 12 bits so the maximum kind of CSRs is 4096 (2^12).
pub const CSR_SIZE: usize = 4096;

//////////////////////////////
// User-level CSR addresses //
//////////////////////////////
// User trap setup.
/// User status register.
const USTATUS: CsrAddress = 0x000;
/// User trap handler base address.
const UTVEC: CsrAddress = 0x005;

// User trap handling.
/// User exception program counter.
const UEPC: CsrAddress = 0x041;
/// User trap cause.
const UCAUSE: CsrAddress = 0x042;
/// User bad address or instruction.
const _UTVAL: CsrAddress = 0x043;

// User floating-point CSRs.
/// Flating-point accrued exceptions.
const _FFLAGS: CsrAddress = 0x001;
/// Floating-point dynamic rounding mode.
const _FRB: CsrAddress = 0x002;
/// Floating-point control and status register (frm + fflags).
pub const FCSR: CsrAddress = 0x003;

// User Counter/Timers.
/// Timer for RDTIME instruction.
pub const TIME: CsrAddress = 0xc01;

/////////////////////////////////////
// Supervisor-level CSR addresses //
////////////////////////////////////
// Supervisor trap setup.
/// Supervisor status register.
pub const SSTATUS: CsrAddress = 0x100;
/// Supervisor exception delegation register.
const SEDELEG: CsrAddress = 0x102;
/// Supervisor interrupt delegation register.
const SIDELEG: CsrAddress = 0x103;
/// Supervisor interrupt-enable register.
pub const SIE: CsrAddress = 0x104;
/// Supervisor trap handler base address.
pub const STVEC: CsrAddress = 0x105;

// Supervisor trap handling.
/// Scratch register for supervisor trap handlers.
const _SSCRATCH: CsrAddress = 0x140;
/// Supervisor exception program counter.
pub const SEPC: CsrAddress = 0x141;
/// Supervisor trap cause.
pub const SCAUSE: CsrAddress = 0x142;
/// Supervisor bad address or instruction.
pub const STVAL: CsrAddress = 0x143;
/// Supervisor interrupt pending.
pub const SIP: CsrAddress = 0x144;

// Supervisor protection and translation.
/// Supervisor address translation and protection.
pub const SATP: CsrAddress = 0x180;

// SSTATUS fields.
const SSTATUS_SIE_MASK: u32 = 0x2; // sstatus[1]
const SSTATUS_SPIE_MASK: u32 = 0x20; // sstatus[5]
const SSTATUS_UBE_MASK: u32 = 0x40; // sstatus[6]
const SSTATUS_SPP_MASK: u32 = 0x100; // sstatus[8]
const SSTATUS_FS_MASK: u32 = 0x6000; // sstatus[14:13]
const SSTATUS_XS_MASK: u32 = 0x18000; // sstatus[16:15]
const SSTATUS_SUM_MASK: u32 = 0x40000; // sstatus[18]
const SSTATUS_MXR_MASK: u32 = 0x80000; // sstatus[19]
                                       // const SSTATUS_UXL_MASK: u32 = 0x3_00000000; // sstatus[33:32]
                                       // const SSTATUS_SD_MASK: u32 = 0x80000000_00000000; // sstatus[63]
const SSTATUS_MASK: u32 = SSTATUS_SIE_MASK
    | SSTATUS_SPIE_MASK
    | SSTATUS_UBE_MASK
    | SSTATUS_SPP_MASK
    | SSTATUS_FS_MASK
    | SSTATUS_XS_MASK
    | SSTATUS_SUM_MASK
    | SSTATUS_MXR_MASK;
// | SSTATUS_UXL_MASK
// | SSTATUS_SD_MASK
/// Global interrupt-enable bit for supervisor mode.
pub const XSTATUS_SIE: CsrFieldRange = 1..=1;
/// Previous interrupt-enable bit for supervisor mode.
pub const XSTATUS_SPIE: CsrFieldRange = 5..=5;
/// Previous privilege mode for supervisor mode.
pub const XSTATUS_SPP: CsrFieldRange = 8..=8;

/////////////////////////////////
// Machine-level CSR addresses //
/////////////////////////////////
// Machine information registers.
/// Vendor ID.
const MVENDORID: CsrAddress = 0xf11;
/// Architecture ID.
const MARCHID: CsrAddress = 0xf12;
/// Implementation ID.
const MIMPID: CsrAddress = 0xf13;
/// Hardware thread ID.
const MHARTID: CsrAddress = 0xf14;

// Machine trap setup.
/// Machine status register.
pub const MSTATUS: CsrAddress = 0x300;
/// Machine status register high.
/// This register is used to store the upper 32 bits of the MSTATUS register.
pub const MSTATUSH: CsrAddress = 0x310;
/// ISA and extensions.
const MISA: CsrAddress = 0x301;
/// Machine exception delefation register.
pub const MEDELEG: CsrAddress = 0x302;
/// Machine interrupt delefation register.
pub const MIDELEG: CsrAddress = 0x303;
/// Machine interrupt-enable register.
pub const MIE: CsrAddress = 0x304;
/// Machine trap-handler base address.
pub const MTVEC: CsrAddress = 0x305;
/// Machine counter enable.
const _MCOUNTEREN: CsrAddress = 0x306;

// Machine trap handling.
/// Scratch register for machine trap handlers.
const _MSCRATCH: CsrAddress = 0x340;
/// Machine exception program counter.
pub const MEPC: CsrAddress = 0x341;
/// Machine trap cause.
pub const MCAUSE: CsrAddress = 0x342;
/// Machine bad address or instruction.
pub const MTVAL: CsrAddress = 0x343;
/// Machine interrupt pending.
pub const MIP: CsrAddress = 0x344;

// Machine memory protection.
/// Physical memory protection configuration.
const _PMPCFG0: CsrAddress = 0x3a0;
/// Physical memory protection address register.
const _PMPADDR0: CsrAddress = 0x3b0;

// MSTATUS fields.
/// Global interrupt-enable bit for machine mode.
pub const MSTATUS_MIE: CsrFieldRange = 3..=3;
/// Previous interrupt-enable bit for machine mode.
pub const MSTATUS_MPIE: CsrFieldRange = 7..=7;
/// Previous privilege mode for machine mode.
pub const MSTATUS_MPP: CsrFieldRange = 11..=12;
/// Modify privilege bit.
pub const MSTATUS_MPRV: CsrFieldRange = 17..=17;

// MIP fields.
/// Supervisor software interrupt.
pub const SSIP_BIT: u32 = 1 << 1;
/// Machine software interrupt.
pub const MSIP_BIT: u32 = 1 << 3;
/// Supervisor timer interrupt.
pub const STIP_BIT: u32 = 1 << 5;
/// Machine timer interrupt.
pub const MTIP_BIT: u32 = 1 << 7;
/// Supervisor external interrupt.
pub const SEIP_BIT: u32 = 1 << 9;
/// Machine external interrupt.
pub const MEIP_BIT: u32 = 1 << 11;

/// The state to contains all the CSRs.
pub struct CpuCsr {
    csrs: [u32; CSR_SIZE],
}

impl Default for CpuCsr {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuCsr {
    /// Create a new `state` object.
    pub fn new() -> Self {
        let mut csrs = [0; CSR_SIZE];
        bitfield::bitfield! {
            struct MisaFlags(u32);
            _, a_ext: 0; // standard A extension for atomic instructions is supported
            _, b_ext: 1; // Tentatively reserved for Bit-Manipulation extension
            _, c_ext: 2; // standard C extension for 16-bit compressed instructions is supported
            _, d_ext: 3; // standard D extension for double-precision floating-point is supported
            _, e_ext: 4; // RV32E base ISA
            _, f_ext: 5; // standard F extension for single-precision floating-point is supported
            _, g_ext: 6; // reserved
            _, h_ext: 7; // Hypervisor extension
            _, i_ext: 8; // RV32I/64I/128I base ISA
            _, j_ext: 9; // Tentatively reserved for Dynamically Translated Languages extension
            _, k_ext: 10; // reserved
            _, l_ext: 11; // reserved
            _, m_ext: 12; // Integer Multiply/Divide extension
            _, n_ext: 13; // Tentatively reserved for User-Level Interrupts extension
            _, o_ext: 14; // reserved
            _, p_ext: 15; // Tentatively reserved for Packed-SIMD extension
            _, q_ext: 16; // Quad-precision floating-point extension
            _, r_ext: 17; // reserved
            _, supervisor: 18; // Supervisor mode implemented
            _, t_ext: 19; // reserved
            _, user: 20; // User mode implemented
            _, v_ext: 21; // Tentatively reserved for Vector extension
            _, w_ext: 22; // reserved
            _, x_ext: 23; // non-standard extensions
            _, y_ext: 24; // reserved
            _, z_ext: 25; // reserved
        }
        impl MisaFlags {
            pub fn inner(&self) -> u32 {
                self.0
            }
        }
        let mut misa = MisaFlags(0);
        misa.user(true);
        misa.supervisor(true);
        misa.m_ext(true);
        misa.a_ext(true);
        // misa.f_ext(true);
        misa.i_ext(true);
        // misa.c_ext(true);
        // misa.d_ext(true);

        csrs[MISA as usize] = misa.inner();

        Self { csrs }
    }

    pub fn dump(&self) {
        const CSR_NAMES: [&str; 16] = [
            "mstatus", "mtvec", "mepc", "mcause", "medeleg", "mideleg", "sstatus", "stvec", "sepc",
            "scause", "sedeleg", "sideleg", "ustatus", "utvec", "uepc", "ucause",
        ];
        const CSR_INDEXS: [CsrAddress; 16] = [
            MSTATUS, MTVEC, MEPC, MCAUSE, MEDELEG, MIDELEG, SSTATUS, STVEC, SEPC, SCAUSE, SEDELEG,
            SIDELEG, USTATUS, UTVEC, UEPC, UCAUSE,
        ];

        info!("{:-^80}", "csr");

        for (name, &index) in CSR_NAMES.iter().zip(CSR_INDEXS.iter()) {
            info!("{:8} = {:#010x}", name, self.read(index));
        }

        info!("{:-^80}", "");
    }

    /// Increment the value in the TIME register.
    pub fn increment_time(&mut self) {
        self.csrs[TIME as usize] = self.csrs[TIME as usize].wrapping_add(1);
    }
}

impl Csr for CpuCsr {
    /// Read the val from the CSR.
    fn read(&self, addr: CsrAddress) -> u32 {
        // 4.1 Supervisor CSRs
        // "The supervisor should only view CSR state that should be visible to a supervisor-level
        // operating system. In particular, there is no information about the existence (or
        // non-existence) of higher privilege levels (machine level or other) visible in the CSRs
        // accessible by the supervisor.  Many supervisor CSRs are a subset of the equivalent
        // machine-mode CSR, and the machinemode chapter should be read first to help understand
        // the supervisor-level CSR descriptions."
        trace!("Reading CSR: {:#x}", addr);
        match addr {
            SSTATUS => self.csrs[MSTATUS as usize] & SSTATUS_MASK,
            SIE => self.csrs[MIE as usize] & self.csrs[MIDELEG as usize],
            SIP => self.csrs[MIP as usize] & self.csrs[MIDELEG as usize],
            _ => self.csrs[addr as usize],
        }
    }

    /// Write the val to the CSR.
    fn write(&mut self, addr: CsrAddress, val: u32) {
        // 4.1 Supervisor CSRs
        // "The supervisor should only view CSR state that should be visible to a supervisor-level
        // operating system. In particular, there is no information about the existence (or
        // non-existence) of higher privilege levels (machine level or other) visible in the CSRs
        // accessible by the supervisor.  Many supervisor CSRs are a subset of the equivalent
        // machine-mode CSR, and the machinemode chapter should be read first to help understand
        // the supervisor-level CSR descriptions."
        trace!("Writing CSR: {:#x} with value: {:#x}", addr, val);
        match addr {
            MVENDORID => (),
            MARCHID => (),
            MIMPID => (),
            MHARTID => (),
            SSTATUS => {
                self.csrs[MSTATUS as usize] =
                    (self.csrs[MSTATUS as usize] & !SSTATUS_MASK) | (val & SSTATUS_MASK);
            }
            SIE => {
                self.csrs[MIE as usize] = (self.csrs[MIE as usize] & !self.csrs[MIDELEG as usize])
                    | (val & self.csrs[MIDELEG as usize]);
            }
            SIP => {
                let mask = SSIP_BIT & self.csrs[MIDELEG as usize];
                self.csrs[MIP as usize] = (self.csrs[MIP as usize] & !mask) | (val & mask);
            }
            _ => self.csrs[addr as usize] = val,
        }
    }

    fn reset(&mut self) {
        *self = Self::new();
    }
}

/// Convert the val implement `RangeBounds` to the `Range` struct.
fn to_range<T: RangeBounds<u32>>(generic_range: &T, bit_length: u32) -> Range<u32> {
    let start = match generic_range.start_bound() {
        Bound::Excluded(&val) => val + 1,
        Bound::Included(&val) => val,
        Bound::Unbounded => 0,
    };
    let end = match generic_range.end_bound() {
        Bound::Excluded(&val) => val,
        Bound::Included(&val) => val + 1,
        Bound::Unbounded => bit_length,
    };

    start..end
}
