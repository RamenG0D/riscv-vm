//! The exception module contains all the exception kinds and the function to handle exceptions.

use std::fmt;
use log::info;
use thiserror::Error;

use crate::{cpu::Cpu, csr::{MCAUSE, MEPC, MSTATUS, MTVAL}, memory::virtual_memory::MemorySize};

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

    // Update CSRs and the program counter depending on an exception.
    pub fn take_trap(&self, cpu: &mut impl Cpu) -> Trap {
		info!("Taking a trap: {:?}", self);

		// 3.1.18 Machine Cause Register (mcause)
		// 4.1.10 Supervisor Cause Register (scause)
		// "When a trap is taken into M-mode, S-mode, or U-mode, mcause (scause) is written with a
		// code indicating the event that caused the trap. When a trap is taken into M-mode, the
		// high bit of mcause is set to 1; when taken into S-mode, the high bit of scause is set to
		// 0; when taken into U-mode, the high bit of scause is set to 0. The encoding of the
		// exception codes is the same in all privilege modes."
		let cause = self.exception_code();

		// 3.1.5 Machine Exception Program Counter (mepc)
		// 4.1.7 Supervisor Exception Program Counter (sepc)
		// "When a trap is taken, mepc (sepc) is written with the virtual address of the
		// instruction that encountered the exception."
		let epc = self.epc(cpu.get_pc());

		// 3.1.17 Machine Trap Value Register (mtval)
		// 4.1.9 Supervisor Trap Value Register (stval)
		// "When a hardware breakpoint is triggered, or an address-misaligned, access-fault, or
		// page-fault exception occurs on an instruction fetch, load, or store, mtval (stval) is
		// written with the faulting virtual address. On an illegal instruction trap, mtval (stval)
		// may be written with the first XLEN or ILEN bits of the faulting instruction as described
		// below. For other traps, mtval (stval) is set to zero, but a future standard may redefine
		// mtval's (stval's) setting for other traps."
		let trap_value = self.trap_value(cpu.get_pc());

		// 3.1.6 Machine Trap Delegation Register (medeleg)
		// 4.1.8 Supervisor Trap Delegation Register (sedeleg)
		// "The mtval register is a WARL field that controls whether synchronous exceptions are
		// reported as exceptions or interrupts. When a bit in medeleg (sedeleg) is set, synchronous
		// exceptions corresponding to that bit number are reported as exceptions; when a bit is
		// clear, the corresponding synchronous exceptions are reported as interrupts."
		let interrupt = false;

		cpu.write_csr(MCAUSE, cause);
		cpu.write_csr(MEPC, epc);
		cpu.write_csr(MTVAL, trap_value);
		let mstatus = cpu.read_csr(MSTATUS);
		cpu.write_csr(MSTATUS, mstatus & !(1 << 3) | (interrupt as MemorySize) << 3);

		Trap::Contained
    }
}
