use std::io::Read;

use riscv_vm::{log::LevelFilter, bus::VirtualDevice, cpu::{Cpu, Privilege, Riscv32Cpu}, memory::dram::{Dram, DRAM_BASE, DRAM_SIZE}};

macro_rules! add_test {
	($name:ident) => {
		#[test]
		fn $name() -> ::std::io::Result<()> {
			riscv_vm::init_logging(LevelFilter::Debug);

			let data = include_bytes!(concat!("/opt/riscv/target/share/riscv-tests/isa/", stringify!($name)));
			let mut cpu = Riscv32Cpu::new();
			let dram = {
				let mut tmp = Box::new(Dram::new());
				tmp.initialize(data);
				tmp
			};
			cpu.add_device(VirtualDevice::new(dram, DRAM_BASE, DRAM_SIZE));
			*cpu.get_register_mut(2).unwrap() = DRAM_BASE - 1;

			let _ = cpu.run();
			// if !matches!(cpu.run(), Err(Exception::LoadAccessFault)) {
			// 	panic!("Failed to run test");
			// }

			cpu.dump_registers();
			let a0 = cpu.get_register(10).unwrap();
			assert_eq!(*a0, 0);

			let mode = cpu.get_privilege();
			assert_eq!(mode, Privilege::Machine);

			Ok(())
		}
	};
}

add_test!(rv32ui_p_bge);
