
pub mod instruction_sets;
pub mod virtual_memory;
pub mod cpu;

#[test]
pub fn run_cpu() {
    use cpu::Cpu;
    use instruction_sets::rv32i::instructions::*;
    let mut cpu = Cpu::<4096, 8192>::new();
    let inst = ADDI.encode(|b| b.rd(1).rs1(1).imm1(30));
    println!("ADDI: 0x{:08X}", inst.to_inner());
    cpu.load_program(&[inst]).expect("Failed to load program");

    // should set x1 to 30
    cpu.execute().expect("Failed to run CPU");

    assert_eq!(*cpu.get_register(1).unwrap(), 30);
}
