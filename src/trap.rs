use crate::{
    cpu::Cpu,
    csr::{
        Mode, MCAUSE, MEDELEG, MEPC, MSTATUS, MTVAL, MTVEC, SCAUSE, SEPC, SSTATUS, STVAL, STVEC,
    },
    memory::virtual_memory::MemorySize,
};

/// All kinds of exceptions, an unusual condition occurring at run
/// time associated with an instruction in the current hardware thread.
#[derive(Debug, PartialEq, Eq)]
pub enum Exception {
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
}

/// The transfer of control to a trap handler caused by either an
/// exception or an interrupt.
pub trait Trap {
    /// Returns an exception code that identifys the last exception.
    fn exception_code(&self) -> MemorySize;
    /// Helper method for a trap handler.
    fn take_trap(&self, cpu: &mut Cpu) {
        let exception_pc = cpu.get_pc().wrapping_sub(4);
        let previous_mode = cpu.get_mode();

        let cause = self.exception_code();
        if (previous_mode <= Mode::Supervisor)
            && ((cpu.read_csr(MEDELEG).wrapping_shr(cause)) & 1 != 0)
        {
            // Handle the trap in S-mode.
            cpu.set_mode(Mode::Supervisor);

            // Set the program counter to the supervisor trap-handler base address (stvec).
            cpu.set_pc(cpu.read_csr(STVEC) & !1);

            // 4.1.9 Supervisor Exception Program Counter (sepc)
            // "The low bit of sepc (sepc[0]) is always zero."
            // "When a trap is taken into S-mode, sepc is written with the virtual address of
            // the instruction that was interrupted or that encountered the exception.
            // Otherwise, sepc is never written by the implementation, though it may be
            // explicitly written by software."
            cpu.write_csr(SEPC, exception_pc & !1);

            // 4.1.10 Supervisor Cause Register (scause)
            // "When a trap is taken into S-mode, scause is written with a code indicating
            // the event that caused the trap.  Otherwise, scause is never written by the
            // implementation, though it may be explicitly written by software."
            cpu.write_csr(SCAUSE, cause);

            // 4.1.11 Supervisor Trap Value (stval) Register
            // "When a trap is taken into S-mode, stval is written with exception-specific
            // information to assist software in handling the trap. Otherwise, stval is never
            // written by the implementation, though it may be explicitly written by software."
            // "When a hardware breakpoint is triggered, or an instruction-fetch, load, or
            // store address-misaligned, access, or page-fault exception occurs, stval is
            // written with the faulting virtual address. On an illegal instruction trap,
            // stval may be written with the first XLEN or ILEN bits of the faulting
            // instruction as described below. For other exceptions, stval is set to zero."
            cpu.write_csr(STVAL, 0);

            // Set a previous interrupt-enable bit for supervisor mode (SPIE, 5) to the value
            // of a global interrupt-enable bit for supervisor mode (SIE, 1).
            cpu.write_csr(
                SSTATUS,
                if ((cpu.read_csr(SSTATUS) >> 1) & 1) == 1 {
                    cpu.read_csr(SSTATUS) | (1 << 5)
                } else {
                    cpu.read_csr(SSTATUS) & !(1 << 5)
                },
            );
            // Set a global interrupt-enable bit for supervisor mode (SIE, 1) to 0.
            cpu.write_csr(SSTATUS, cpu.read_csr(SSTATUS) & !(1 << 1));
            // 4.1.1 Supervisor Status Register (sstatus)
            // "When a trap is taken, SPP is set to 0 if the trap originated from user mode, or
            // 1 otherwise."
            match previous_mode {
                Mode::User => cpu.write_csr(SSTATUS, cpu.read_csr(SSTATUS) & !(1 << 8)),
                _ => cpu.write_csr(SSTATUS, cpu.read_csr(SSTATUS) | (1 << 8)),
            }
        } else {
            // Handle the trap in M-mode.
            cpu.set_mode(Mode::Machine);

            // Set the program counter to the machine trap-handler base address (mtvec).
            cpu.set_pc(cpu.read_csr(MTVEC) & !1);

            // 3.1.15 Machine Exception Program Counter (mepc)
            // "The low bit of mepc (mepc[0]) is always zero."
            // "When a trap is taken into M-mode, mepc is written with the virtual address of
            // the instruction that was interrupted or that encountered the exception.
            // Otherwise, mepc is never written by the implementation, though it may be
            // explicitly written by software."
            cpu.write_csr(MEPC, exception_pc & !1);

            // 3.1.16 Machine Cause Register (mcause)
            // "When a trap is taken into M-mode, mcause is written with a code indicating
            // the event that caused the trap. Otherwise, mcause is never written by the
            // implementation, though it may be explicitly written by software."
            cpu.write_csr(MCAUSE, cause);

            // 3.1.17 Machine Trap Value (mtval) Register
            // "When a trap is taken into M-mode, mtval is either set to zero or written with
            // exception-specific information to assist software in handling the trap.
            // Otherwise, mtval is never written by the implementation, though it may be
            // explicitly written by software."
            // "When a hardware breakpoint is triggered, or an instruction-fetch, load, or
            // store address-misaligned, access, or page-fault exception occurs, mtval is
            // written with the faulting virtual address. On an illegal instruction trap,
            // mtval may be written with the first XLEN or ILEN bits of the faulting
            // instruction as described below. For other traps, mtval is set to zero."
            cpu.write_csr(MTVAL, 0);

            // Set a previous interrupt-enable bit for supervisor mode (MPIE, 7) to the value
            // of a global interrupt-enable bit for supervisor mode (MIE, 3).
            cpu.write_csr(
                MSTATUS,
                if ((cpu.read_csr(MSTATUS) >> 3) & 1) == 1 {
                    cpu.read_csr(MSTATUS) | (1 << 7)
                } else {
                    cpu.read_csr(MSTATUS) & !(1 << 7)
                },
            );
            // Set a global interrupt-enable bit for supervisor mode (MIE, 3) to 0.
            cpu.write_csr(MSTATUS, cpu.read_csr(MSTATUS) & !(1 << 3));
            // Set a previous privilege mode for supervisor mode (MPP, 11..13) to 0.
            cpu.write_csr(MSTATUS, cpu.read_csr(MSTATUS) & !(0b11 << 11));
        }
    }
}

impl Trap for Exception {
    fn exception_code(&self) -> MemorySize {
        match self {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAddressMisaligned => 6,
            Exception::StoreAccessFault => 7,
            Exception::EnvironmentCallFromUMode => 8,
            Exception::EnvironmentCallFromSMode => 9,
            Exception::EnvironmentCallFromMMode => 11,
            Exception::InstructionPageFault => 12,
            Exception::LoadPageFault => 13,
            Exception::StorePageFault => 15,
        }
    }
}

impl Exception {
    pub fn is_fatal(&self) -> bool {
        match self {
            Exception::InstructionAddressMisaligned
            | Exception::InstructionAccessFault
            | Exception::LoadAccessFault
            | Exception::StoreAddressMisaligned
            | Exception::StoreAccessFault => true,
            _ => false,
        }
    }
}
