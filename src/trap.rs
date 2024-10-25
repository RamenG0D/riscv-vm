//! The exception module contains all the exception kinds and the function to handle exceptions.

use std::fmt;

use thiserror::Error;

use crate::{
    cpu::{Cpu, Mode},
    csr::*, bit_ops::*,
};

/// All the exception kinds.
#[derive(Error, PartialEq)]
pub enum Exception {
    /// With the addition of the C extension, no instructions can raise
    /// instruction-address-misaligned exceptions.
    InstructionAddressMisaligned,
    InstructionAccessFault,
    IllegalInstruction(u32),
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode,
    // Stores a trap value (the faulting address) for page fault exceptions.
    InstructionPageFault(u32),
    LoadPageFault(u32),
    StorePageFault(u32),
}

impl fmt::Debug for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exception::InstructionAddressMisaligned => write!(f, "Instruction address misaligned"),
            Exception::InstructionAccessFault => write!(f, "Instruction access fault"),
            Exception::IllegalInstruction(val) => write!(f, "Illegal instruction({:#010x})", val),
            Exception::Breakpoint => write!(f, "Breakpoint"),
            Exception::LoadAddressMisaligned => write!(f, "Load address misaligned"),
            Exception::LoadAccessFault => write!(f, "Load access fault"),
            Exception::StoreAddressMisaligned => write!(f, "Store address misaligned"),
            Exception::StoreAccessFault => write!(f, "Store access fault"),
            Exception::EnvironmentCallFromUMode => write!(f, "Environment call from U-mode"),
            Exception::EnvironmentCallFromSMode => write!(f, "Environment call from S-mode"),
            Exception::EnvironmentCallFromMMode => write!(f, "Environment call from M-mode"),
            Exception::InstructionPageFault(val) => write!(f, "InstructionPageFault({:#010x})", val),
            Exception::LoadPageFault(val) => write!(f, "LoadPageFault({:#010x})", val),
            Exception::StorePageFault(val) => write!(f, "StorePageFault({:#010x})", val),
        }
    }
}

impl fmt::Display for Exception {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Exception::InstructionAddressMisaligned => write!(f, "Instruction address misaligned"),
            Exception::InstructionAccessFault => write!(f, "Instruction access fault"),
            Exception::IllegalInstruction(val) => write!(f, "Illegal instruction: {:#010x}", val),
            Exception::Breakpoint => write!(f, "Breakpoint"),
            Exception::LoadAddressMisaligned => write!(f, "Load address misaligned"),
            Exception::LoadAccessFault => write!(f, "Load access fault"),
            Exception::StoreAddressMisaligned => write!(f, "Store address misaligned"),
            Exception::StoreAccessFault => write!(f, "Store access fault"),
            Exception::EnvironmentCallFromUMode => write!(f, "Environment call from U-mode"),
            Exception::EnvironmentCallFromSMode => write!(f, "Environment call from S-mode"),
            Exception::EnvironmentCallFromMMode => write!(f, "Environment call from M-mode"),
            Exception::InstructionPageFault(val) => write!(f, "Instruction page fault: {:#010x}", val),
            Exception::LoadPageFault(val) => write!(f, "Load page fault: {:#010x}", val),
            Exception::StorePageFault(val) => write!(f, "Store page fault: {:#010x}", val),
        }
    }
}

/// All the trap kinds.
#[derive(Debug)]
pub enum Trap {
    /// The trap is visible to, and handled by, software running inside the execution
    /// environment.
    Contained,
    /// The trap is a synchronous exception that is an explicit call to the execution
    /// environment requesting an action on behalf of software inside the execution environment.
    Requested,
    /// The trap is handled transparently by the execution environment and execution
    /// resumes normally after the trap is handled.
    Invisible,
    /// The trap represents a fatal failure and causes the execution environment to terminate
    /// execution.
    Fatal,
}

impl Exception {
    fn exception_code(&self) -> u32 {
        match self {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction(_) => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAddressMisaligned => 6,
            Exception::StoreAccessFault => 7,
            Exception::EnvironmentCallFromUMode => 8,
            Exception::EnvironmentCallFromSMode => 9,
            Exception::EnvironmentCallFromMMode => 11,
            Exception::InstructionPageFault(_) => 12,
            Exception::LoadPageFault(_) => 13,
            Exception::StorePageFault(_) => 15,
        }
    }

    fn epc(&self, pc: u32) -> u32 {
        // 3.2.1 Environment Call and Breakpoint
        // "ECALL and EBREAK cause the receiving privilege mode’s epc register to be set to the
        // address of the ECALL or EBREAK instruction itself, not the address of the following
        // instruction."
        match self {
            Exception::Breakpoint
            | Exception::EnvironmentCallFromUMode
            | Exception::EnvironmentCallFromSMode
            | Exception::EnvironmentCallFromMMode
            // TODO: why page fault needs this?
            | Exception::InstructionPageFault(_)
            | Exception::LoadPageFault(_)
            | Exception::StorePageFault(_) => pc.wrapping_sub(4),
            _ => pc,
        }
    }

    fn trap_value(&self, pc: u32) -> u32 {
        // 3.1.17 Machine Trap Value Register (mtval)
        // 4.1.9 Supervisor Trap Value Register (stval)
        // "When a hardware breakpoint is triggered, or an address-misaligned, access-fault, or
        // page-fault exception occurs on an instruction fetch, load, or store, mtval (stval) is
        // written with the faulting virtual address. On an illegal instruction trap, mtval (stval)
        // may be written with the first XLEN or ILEN bits of the faulting instruction as described
        // below. For other traps, mtval (stval) is set to zero, but a future standard may redefine
        // mtval's (stval's) setting for other traps."
        match self {
            Exception::InstructionAddressMisaligned
            | Exception::InstructionAccessFault
            | Exception::Breakpoint
            | Exception::LoadAddressMisaligned
            | Exception::LoadAccessFault
            | Exception::StoreAddressMisaligned
            | Exception::StoreAccessFault => pc,
            Exception::InstructionPageFault(val)
            | Exception::LoadPageFault(val)
            | Exception::StorePageFault(val) => *val,
            Exception::IllegalInstruction(val) => *val,
            _ => 0,
        }
    }

    /// Update CSRs and the program counter depending on an exception.
    pub fn take_trap(&self, cpu: &mut Cpu) -> Trap {
        // 1.2 Privilege Levels
        // "Traps that increase privilege level are termed vertical traps, while traps that remain
        // at the same privilege level are termed horizontal traps."

        let exception_pc = self.epc(cpu.get_pc());
        let previous_mode = cpu.get_mode();
        let cause = self.exception_code();

        // 3.1.8 Machine Trap Delegation Registers (medeleg and mideleg)
        // "By default, all traps at any privilege level are handled in machine mode"
        // "To increase performance, implementations can provide individual read/write bits within
        // medeleg and mideleg to indicate that certain exceptions and interrupts should be
        // processed directly by a lower privilege level."
        //
        // "medeleg has a bit position allocated for every synchronous exception shown in Table 3.6
        // on page 37, with the index of the bit position equal to the value returned in the mcause
        // register (i.e., setting bit 8 allows user-mode environment calls to be delegated to a
        // lower-privilege trap handler)."
        if previous_mode <= Mode::Supervisor && ((cpu.state().read(MEDELEG) >> cause) & 1) == 1 {
            // Handle the trap in S-mode.
            cpu.set_mode(Mode::Supervisor);

            // Set the program counter to the supervisor trap-handler base address (stvec).
            cpu.set_pc(clear_bit(cpu.state().read(STVEC), 0));

            // 4.1.9 Supervisor Exception Program Counter (sepc)
            // "The low bit of sepc (sepc[0]) is always zero."
            // "When a trap is taken into S-mode, sepc is written with the virtual address of
            // the instruction that was interrupted or that encountered the exception.
            // Otherwise, sepc is never written by the implementation, though it may be
            // explicitly written by software."
            cpu.state_mut().write(SEPC, clear_bit(exception_pc, 0));

            // 4.1.10 Supervisor Cause Register (scause)
            // "When a trap is taken into S-mode, scause is written with a code indicating
            // the event that caused the trap.  Otherwise, scause is never written by the
            // implementation, though it may be explicitly written by software."
            cpu.state_mut().write(SCAUSE, cause);

            // 4.1.11 Supervisor Trap Value (stval) Register
            // "When a trap is taken into S-mode, stval is written with exception-specific
            // information to assist software in handling the trap. Otherwise, stval is never
            // written by the implementation, though it may be explicitly written by software."
            cpu.state_mut().write(STVAL, self.trap_value(exception_pc));

            // Set a previous interrupt-enable bit for supervisor mode (SPIE, 5) to the value
            // of a global interrupt-enable bit for supervisor mode (SIE, 1).
            let value = cpu.state().read_sstatus(XSTATUS_SIE);
            cpu.state_mut().write_sstatus(XSTATUS_SPIE, value);
            // Set a global interrupt-enable bit for supervisor mode (SIE, 1) to 0.
            cpu.state_mut().write_sstatus(XSTATUS_SIE, 0);
            // 4.1.1 Supervisor Status Register (sstatus)
            // "When a trap is taken, SPP is set to 0 if the trap originated from user mode, or
            // 1 otherwise."
            match previous_mode {
                Mode::User => cpu.state_mut().write_sstatus(XSTATUS_SPP, 0),
                _ => cpu.state_mut().write_sstatus(XSTATUS_SPP, 1),
            }
        } else {
            // Handle the trap in M-mode.
            cpu.set_mode(Mode::Machine);

            // Set the program counter to the machine trap-handler base address (mtvec).
            cpu.set_pc(clear_bit(cpu.state().read(MTVEC), 0));

            // 3.1.15 Machine Exception Program Counter (mepc)
            // "The low bit of mepc (mepc[0]) is always zero."
            // "When a trap is taken into M-mode, mepc is written with the virtual address of
            // the instruction that was interrupted or that encountered the exception.
            // Otherwise, mepc is never written by the implementation, though it may be
            // explicitly written by software."
            cpu.state_mut().write(MEPC, clear_bit(exception_pc, 0));

            // 3.1.16 Machine Cause Register (mcause)
            // "When a trap is taken into M-mode, mcause is written with a code indicating
            // the event that caused the trap. Otherwise, mcause is never written by the
            // implementation, though it may be explicitly written by software."
            cpu.state_mut().write(MCAUSE, cause);

            // 3.1.17 Machine Trap Value (mtval) Register
            // "When a trap is taken into M-mode, mtval is either set to zero or written with
            // exception-specific information to assist software in handling the trap.
            // Otherwise, mtval is never written by the implementation, though it may be
            // explicitly written by software."
            cpu.state_mut().write(MTVAL, self.trap_value(exception_pc));

            // Set a previous interrupt-enable bit for machine mode (MPIE, 7) to the value
            // of a global interrupt-enable bit for machine mode (MIE, 3).
            let value = cpu.state().read_mstatus(MSTATUS_MIE);
            cpu.state_mut().write_mstatus(MSTATUS_MPIE, value);
            // Set a global interrupt-enable bit for machine mode (MIE, 3) to 0.
            cpu.state_mut().write_mstatus(MSTATUS_MIE, 0);
            // When a trap is taken from privilege mode y into privilege mode x, xPIE is set
            // to the value of x IE; x IE is set to 0; and xPP is set to y.
            match previous_mode {
                Mode::User => cpu.state_mut().write_mstatus(MSTATUS_MPP, Mode::User as u32),
                Mode::Supervisor => cpu.state_mut().write_mstatus(MSTATUS_MPP, Mode::Supervisor as u32),
                Mode::Machine => cpu.state_mut().write_mstatus(MSTATUS_MPP, Mode::Machine as u32),
            }
        }

        match self {
            Exception::InstructionAddressMisaligned | Exception::InstructionAccessFault => {
                Trap::Fatal
            }
            Exception::IllegalInstruction(_) => Trap::Invisible,
            Exception::Breakpoint => Trap::Requested,
            Exception::LoadAddressMisaligned
            | Exception::LoadAccessFault
            | Exception::StoreAddressMisaligned
            | Exception::StoreAccessFault => Trap::Fatal,
            Exception::EnvironmentCallFromUMode
            | Exception::EnvironmentCallFromSMode
            | Exception::EnvironmentCallFromMMode => Trap::Requested,
            Exception::InstructionPageFault(_)
            | Exception::LoadPageFault(_)
            | Exception::StorePageFault(_) => Trap::Invisible,
        }
    }
}
