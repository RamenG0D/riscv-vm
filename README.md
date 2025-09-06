
    $$$$$$$\  $$\                                       $$\    $$\ $$\      $$\ 
    $$  __$$\ \__|                                      $$ |   $$ |$$$\    $$$ |
    $$ |  $$ |$$\  $$$$$$$\  $$$$$$$\ $$\    $$\        $$ |   $$ |$$$$\  $$$$ |
    $$$$$$$  |$$ |$$  _____|$$  _____|\$$\  $$  |$$$$$$\\$$\  $$  |$$\$$\$$ $$ |
    $$  __$$< $$ |\$$$$$$\  $$ /       \$$\$$  / \______|\$$\$$  / $$ \$$$  $$ |
    $$ |  $$ |$$ | \____$$\ $$ |        \$$$  /           \$$$  /  $$ |\$  /$$ |
    $$ |  $$ |$$ |$$$$$$$  |\$$$$$$$\    \$  /             \$  /   $$ | \_/ $$ |
    \__|  \__|\__|\_______/  \_______|    \_/               \_/    \__|     \__|


# A Riscv 32-Bit Emulator

Only supports bare metal currently

Example:
```rust
let data = include_bytes!(concat!(
                concat!(
                    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/rvtests/"),
                    stringify!($name)
                ),
                ".bin"
            ));
let mut cpu = Riscv32Cpu::new();
let dram = {
    let mut tmp = Box::new(Dram::new());
    tmp.initialize(data);
    tmp
};
            cpu.add_device(VirtualDevice::new(dram, DRAM_BASE, DRAM_SIZE));
            cpu.register_syscall(93, |cpu| {
                let exit_code = *cpu.get_register(10 /* register ( a0 ) */).unwrap();
                let gp = *cpu.get_register(3 /* register ( gp ) */).unwrap();
                cpu.set_privilege(Privilege::Machine);

                if exit_code != 0 || gp != 1 {
                    panic!(
                        "exit code: {0:} [{0:#X}], gp: {1:} [{1:#X}], pc: {2:} [{2:#X}]",
                        exit_code,
                        gp,
                        cpu.get_pc()
                    );
                }

                Trap::Fatal
            });

            let _ = cpu.run();

            let a0 = cpu.get_register(10).unwrap();
            assert_eq!(*a0, 0);

            let mode = cpu.get_privilege();
            assert_eq!(mode, Privilege::Machine);
```
