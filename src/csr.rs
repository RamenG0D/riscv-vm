// Machine-level CSRs.
/// Hardware thread ID.
pub const MHARTID: usize = 0xf14;
/// Machine status register.
pub const MSTATUS: usize = 0x300;
/// Machine exception delefation register.
pub const MEDELEG: usize = 0x302;
/// Machine interrupt delefation register.
pub const MIDELEG: usize = 0x303;
/// Machine interrupt-enable register.
pub const MIE: usize = 0x304;
/// Machine trap-handler base address.
pub const MTVEC: usize = 0x305;
/// Machine counter enable.
pub const MCOUNTEREN: usize = 0x306;
/// Scratch register for machine trap handlers.
pub const MSCRATCH: usize = 0x340;
/// Machine exception program counter.
pub const MEPC: usize = 0x341;
/// Machine trap cause.
pub const MCAUSE: usize = 0x342;
/// Machine bad address or instruction.
pub const MTVAL: usize = 0x343;
/// Machine interrupt pending.
pub const MIP: usize = 0x344;
// Machine trap setup.
/// ISA and extensions.
const MISA: usize = 0x301;

// Supervisor-level CSRs.
/// Supervisor status register.
pub const SSTATUS: usize = 0x100;
/// Supervisor interrupt-enable register.
pub const SIE: usize = 0x104;
/// Supervisor trap handler base address.
pub const STVEC: usize = 0x105;
/// Scratch register for supervisor trap handlers.
pub const SSCRATCH: usize = 0x140;
/// Supervisor exception program counter.
pub const SEPC: usize = 0x141;
/// Supervisor trap cause.
pub const SCAUSE: usize = 0x142;
/// Supervisor bad address or instruction.
pub const STVAL: usize = 0x143;
/// Supervisor interrupt pending.
pub const SIP: usize = 0x144;
/// Supervisor address translation and protection.
pub const SATP: usize = 0x180;

/// The privileged mode.
#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone)]
pub enum Mode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
}

pub struct Csr {
    csr: [u32; 4096],
}

impl Csr {
    pub fn new() -> Self {
        let mut csr = [0; 4096];

        bitfield::bitfield! {
            struct MisaFlags(u32);
            impl new;
            u32;
            // whether or not xlen=32
            xlen_val, xlen: 8, 8;
            m, m_ext: 12, 12;
            a, a_ext: 0, 0;
            f, f_ext: 5, 5;
            d, d_ext: 3, 3;
            q, q_ext: 16, 16;
            c, c_ext: 2, 2;
            l, l_ext: 11, 11;
            b, b_ext: 1, 1;
            j, j_ext: 9, 9;
            p, p_ext: 15, 15;
            v, v_ext: 21, 21;
            n, n_ext: 13, 13;
            g, g_ext: 6, 6;
            h, h_ext: 7, 7;
            x, x_ext: 23, 23;
            supervisor_val, supervisor: 18, 18;
            user_val, user: 20, 20;
        }
        impl MisaFlags {
            pub fn inner(&self) -> u32 {
                self.0
            }
        }

        let misa = MisaFlags::new(1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1);

        csr[MISA] = misa.inner();

        Self { csr }
    }

    pub fn write_csr(&mut self, addr: usize, value: u32) -> Option<()> {
        match addr {
            SIE => {
                self.csr[MIE] = (self.csr[MIE] & !self.csr[MIDELEG]) | (value & self.csr[MIDELEG])
            }
            _ => *self.csr.get_mut(addr)? = value,
        }
        Some(())
    }

    pub fn read_csr(&self, addr: usize) -> Option<u32> {
        match addr {
            SIE => Some(self.csr[MIE] & self.csr[MIDELEG]),
            _ => self.csr.get(addr).map(|&v| v),
        }
    }
}
