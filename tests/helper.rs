use riscv_vm::{
    cpu::{Cpu, POINTER_TO_DTB},
    memory::dram::{DRAM_BASE, DRAM_SIZE},
    registers::REGISTERS_COUNT,
};

pub const DEFAULT_SP: u32 = DRAM_BASE + DRAM_SIZE;

/// Create registers for x0-x31 with expected values.
pub fn expected_xregs(non_zero_regs: Vec<(usize, u32)>) -> [u32; REGISTERS_COUNT] {
    let mut xregs = [0; REGISTERS_COUNT];

    // Based on XRegisters::new().
    xregs[2] = DEFAULT_SP;
    xregs[11] = POINTER_TO_DTB;

    for pair in non_zero_regs.iter() {
        xregs[pair.0] = pair.1;
    }
    xregs
}

/// Create registers for f0-f31 with expected values.
pub fn expected_fregs(non_zero_regs: Vec<(usize, f32)>) -> [f32; REGISTERS_COUNT] {
    let mut fregs = [0.0; REGISTERS_COUNT];

    for pair in non_zero_regs.iter() {
        fregs[pair.0] = pair.1;
    }
    fregs
}

/// Start a test and check if the registers are expected.
pub fn run(emu: &mut Cpu, data: Vec<u8>, expected_xregs: &[u32; 32], expected_fregs: &[f32; 32]) {
    let len = data.len() as u32;

    // emu.is_debug = true;

    emu.initialize_dram(&data).expect("failed to initialize dram");
    emu.set_pc(DRAM_BASE);

    emu.test_start(DRAM_BASE, DRAM_BASE + len);

    for (i, e) in expected_xregs.iter().enumerate() {
        assert_eq!(*e, emu.xregs.get(i), "fails at {i}");
    }
    for (i, e) in expected_fregs.iter().enumerate() {
        assert_eq!(
            (*e).to_bits(),
            emu.fregs.get(i).to_bits(),
            "fails at {} expected {} but got {} ",
            i,
            *e,
            emu.fregs.get(i)
        );
    }
}
