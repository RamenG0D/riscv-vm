use crate::instruction_sets::rv32i::{Instruction, InstructionDecoded};

#[test]
pub fn kernel_test() {
    use std::io::Read;
    let mut cpu = Cpu::new();

    let mut file = std::fs::File::open("Image").unwrap();

    let mut fbytes = Vec::new();
    let _ = file.read_to_end(&mut fbytes).unwrap();

    let mut pbytes = Vec::new();
    for byte in fbytes.chunks_exact(4) {
        let byte = byte.iter().map(|b| b.to_le()).collect::<Vec<u8>>();
        let byte = u32::from_le_bytes(byte.try_into().unwrap());
        pbytes.push(byte);
    }

    cpu.load_program_raw(&pbytes).unwrap();

    let mut regs_changed = false;
    while cpu.pc < cpu.heap_memory.len() as RegisterSize && !regs_changed {
        cpu.execute().expect("Failed to execute inst");
        cpu.pc += 1;
        regs_changed = cpu.registers.iter().any(|&r| r != 0);
    }
}

pub type RegisterSize = u32;
pub type MemorySize = u32;

// used to ensure stored memory is always little endian when accessed and when modified
pub struct Memory<const LENGTH: usize> {
    array: [MemorySize; LENGTH],
}

impl<const L: usize> Memory<L> {
    pub fn new() -> Self {
        Self { array: [0; L] }
    }

    // may be needed for larger memory sizes, to avoid stack overflow
    pub fn new_boxed() -> Box<Self> {
        Box::new(Self { array: [0; L] })
    }

    pub fn get(&self, index: MemorySize) -> Option<MemorySize> {
        if index < L as MemorySize {
            Some(self.array[index as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: MemorySize, value: MemorySize) -> Result<(), String> {
        if index < L as MemorySize {
            self.array[index as usize] = value.to_le();
            Ok(())
        } else {
            Err(format!("Index out of bounds"))
        }
    }

    pub fn len(&self) -> usize {
        L
    }

    pub fn as_slice(&self) -> &[MemorySize] {
        &self.array
    }

    pub fn as_mut_slice(&mut self) -> &mut [MemorySize] {
        &mut self.array
    }
}

pub struct HeapMemory<const LENGTH: usize> {
    memory: Box<[MemorySize]>,
    _phantom: std::marker::PhantomData<[u8; LENGTH]>,
}

impl<const L: usize> HeapMemory<L> {
    pub fn new() -> Self {
        Self {
            // the memory is on the heap but never changes size
            memory: vec![0; L].into_boxed_slice(),
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn get(&self, index: MemorySize) -> Option<MemorySize> {
        if index < L as MemorySize {
            Some(self.memory[index as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, index: MemorySize, value: MemorySize) -> Result<(), String> {
        if index < L as MemorySize {
            self.memory[index as usize] = value.to_le();
            Ok(())
        } else {
            Err(format!("Index out of bounds"))
        }
    }

    pub fn len(&self) -> usize {
        L
    }

    pub fn as_slice(&self) -> &[MemorySize] {
        &self.memory
    }

    pub fn as_mut_slice(&mut self) -> &mut [MemorySize] {
        &mut self.memory
    }
}

// 32 bit RiscV CPU architecture
pub struct Cpu {
    // From https://riscv.org/wp-content/uploads/2017/05/riscv-spec-v2.2.pdf#page=22&zoom=auto,-95,583
    registers: [RegisterSize; 32],

    // program counter
    pc: RegisterSize,

    // little endian memory / stack array
    stack_memory: Memory<4096>,
    // little endian memory / heap array
    heap_memory: HeapMemory<16777216>,
}

impl Cpu {
    pub const ZERO: RegisterSize = 0;
    pub const RA: RegisterSize = 1;
    pub const SP: RegisterSize = 2;
    pub const GP: RegisterSize = 3;
    pub const TP: RegisterSize = 4;
    pub const PC: RegisterSize = 32;

    pub fn new() -> Self {
        Self {
            registers: [0; 32],
            stack_memory: Memory::new(),
            heap_memory: HeapMemory::new(),
            pc: 0,
        }
    }

    pub fn get_register(&self, register: RegisterSize) -> Result<&RegisterSize, String> {
        match register {
            0..=31 => Ok(&self.registers[register as usize]),
            32 => Ok(&self.pc),
            _ => Err(format!(
                "The register '{register}' is not an addressable register?"
            )),
        }
    }

    pub fn get_register_mut(
        &mut self,
        register: RegisterSize,
    ) -> Result<&mut RegisterSize, String> {
        match register {
            0..=31 => Ok(&mut self.registers[register as usize]),
            32 => Ok(&mut self.pc),
            _ => Err(format!(
                "The register '{register}' is not an addressable register?"
            )),
        }
    }

    pub fn fetch(&self) -> Result<Instruction, String> {
        let inst = self.heap_memory.get(self.pc).ok_or(format!("Failed to fetch memory at addr {}", self.pc))?;
        Ok(Instruction::from(inst))
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let inst = self.fetch()?;
        println!("Executing instruction: 0x{:08X}", inst.to_inner());

        match inst.decode() {
            InstructionDecoded::Add { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1.wrapping_add(rs2);
            }
            InstructionDecoded::Addi { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1.wrapping_add(imm);
            }
            InstructionDecoded::Sub { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1.wrapping_sub(rs2);
            }
            InstructionDecoded::Addw { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = rs1.wrapping_add(rs2) as u32;
            }
            InstructionDecoded::Subw { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = rs1.wrapping_sub(rs2) as u32;
            }
            InstructionDecoded::Sllw { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = rs1.wrapping_shl(rs2 as u32) as u32;
            }
            InstructionDecoded::Srlw { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = rs1.wrapping_shr(rs2 as u32) as u32;
            }
            InstructionDecoded::Sraw { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = rs1.wrapping_shr(rs2 as u32) as u32;
            }
            InstructionDecoded::And { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 & rs2;
            }
            InstructionDecoded::Andi { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 & imm;
            }
            InstructionDecoded::Or { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 | rs2;
            }
            InstructionDecoded::Ori { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 | imm;
            }
            InstructionDecoded::Xor { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 ^ rs2;
            }
            InstructionDecoded::Xori { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 ^ imm;
            }
            InstructionDecoded::Sll { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 << rs2;
            }
            InstructionDecoded::Slli { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 << imm;
            }
            InstructionDecoded::Srl { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = rs1 >> rs2;
            }
            InstructionDecoded::Srli { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = rs1 >> imm;
            }
            InstructionDecoded::Sra { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = (rs1 >> rs2) as u32;
            }
            InstructionDecoded::Srai { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                self.registers[rd as usize] = (rs1 >> imm) as u32;
            }
            InstructionDecoded::Lui { rd, imm } => {
                self.registers[rd as usize] = imm << 12;
            }
            InstructionDecoded::AuiPc { rd, imm } => {
                self.registers[rd as usize] = self.pc.wrapping_add(imm << 12);
            }
            InstructionDecoded::Jal { rd, imm } => {
                self.registers[rd as usize] = self.pc;
                self.pc = self.pc.wrapping_add(imm);
            }
            InstructionDecoded::Jalr { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = self.pc;
                self.pc = rs1.wrapping_add(imm);
            }
            InstructionDecoded::Beq { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                if rs1 == rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bne { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                if rs1 != rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Blt { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                if rs1 < rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bge { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                if rs1 >= rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bltu { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                if rs1 < rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Bgeu { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                if rs1 >= rs2 {
                    self.pc = self.pc.wrapping_add(imm);
                }
            }
            InstructionDecoded::Lb { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let addr = rs1.wrapping_add(imm);
                let byte = self.heap_memory.get(addr).ok_or(format!("Failed to fetch memory at addr {}", addr))?;
                self.registers[rd as usize] = byte as i8 as i32 as u32;
            }
            InstructionDecoded::Lh { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let addr = rs1.wrapping_add(imm);
                let byte = self.heap_memory.get(addr).ok_or(format!("Failed to fetch memory at addr {}", addr))?;
                self.registers[rd as usize] = byte as i16 as i32 as u32;
            }
            InstructionDecoded::Lw { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let addr = rs1.wrapping_add(imm);
                let byte = self.heap_memory.get(addr).ok_or(format!("Failed to fetch memory at addr {}", addr))?;
                self.registers[rd as usize] = byte;
            }
            InstructionDecoded::Ld { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let addr = rs1.wrapping_add(imm);
                let byte = self.heap_memory.get(addr).ok_or(format!("Failed to fetch memory at addr {}", addr))?;
                self.registers[rd as usize] = byte;
            }
            InstructionDecoded::Lbu { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let addr = rs1.wrapping_add(imm);
                let byte = self.heap_memory.get(addr).ok_or(format!("Failed to fetch memory at addr {}", addr))?;
                self.registers[rd as usize] = byte & 0xFF;
            }
            InstructionDecoded::Lhu { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let addr = rs1.wrapping_add(imm);
                let byte = self.heap_memory.get(addr).ok_or(format!("Failed to fetch memory at addr {}", addr))?;
                self.registers[rd as usize] = byte & 0xFFFF;
            }
            InstructionDecoded::Lwu { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let addr = rs1.wrapping_add(imm);
                let byte = self.heap_memory.get(addr).ok_or(format!("Failed to fetch memory at addr {}", addr))?;
                self.registers[rd as usize] = byte;
            }
            InstructionDecoded::Sb { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                let addr = rs1.wrapping_add(imm);
                self.heap_memory.set(addr, rs2)?;
            }
            InstructionDecoded::Sh { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                let addr = rs1.wrapping_add(imm);
                self.heap_memory.set(addr, rs2)?;
            }
            InstructionDecoded::Sw { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                let addr = rs1.wrapping_add(imm);
                self.heap_memory.set(addr, rs2)?;
            }
            InstructionDecoded::Sd { rs1, rs2, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                let addr = rs1.wrapping_add(imm);
                self.heap_memory.set(addr, rs2)?;
            }
            InstructionDecoded::ECall => {
                println!("ECALL");
            }
            InstructionDecoded::EBreak => {
                println!("EBREAK");
            }
            InstructionDecoded::CsrRw { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let csr = self.registers[imm as usize];
                self.registers[rd as usize] = csr;
                self.registers[csr as usize] = rs1;
            }
            InstructionDecoded::CsrRs { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let csr = self.registers[imm as usize];
                self.registers[rd as usize] = csr;
                self.registers[csr as usize] |= rs1;
            }
            InstructionDecoded::CsrRc { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                let csr = self.registers[imm as usize];
                self.registers[rd as usize] = imm;
                self.registers[csr as usize] &= !rs1;
            }
            InstructionDecoded::CsrRwi { rd, rs1, imm } => {
                let csr = self.registers[imm as usize];
                self.registers[rd as usize] = csr;
                self.registers[csr as usize] = rs1;
            }
            InstructionDecoded::CsrRsi { rd, rs1, imm } => {
                let csr = self.registers[imm as usize];
                self.registers[rd as usize] = csr;
                self.registers[csr as usize] |= rs1;
            }
            InstructionDecoded::Slti { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                self.registers[rd as usize] = if rs1 < imm as i32 { 1 } else { 0 };
            }
            InstructionDecoded::Sltiu { rd, rs1, imm } => {
                let rs1 = self.registers[rs1 as usize];
                self.registers[rd as usize] = if rs1 < imm { 1 } else { 0 };
            }
            InstructionDecoded::Slt { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize] as i32;
                let rs2 = self.registers[rs2 as usize] as i32;
                self.registers[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }
            InstructionDecoded::Sltu { rd, rs1, rs2 } => {
                let rs1 = self.registers[rs1 as usize];
                let rs2 = self.registers[rs2 as usize];
                self.registers[rd as usize] = if rs1 < rs2 { 1 } else { 0 };
            }
            InstructionDecoded::Fence { pred, succ } => {
                println!("FENCE pred: {:#010x} succ: {:#010x}", pred, succ);
            }
            InstructionDecoded::FenceI { pred, succ } => {
                println!("FENCE.I pred: {:#010x} succ: {:#010x}", pred, succ);
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        while self.pc < self.heap_memory.len() as RegisterSize {
            self.execute()?;
            self.pc += 1;
        }
        Ok(())
    }

    pub fn load_program_raw(&mut self, program: &[MemorySize]) -> Result<(), String> {
        if program.len() > self.heap_memory.len() {
            return Err(format!(
                "Program is too large to fit in memory => size: {} bytes | actual size: {} bytes",
                program.len() * 4,
                self.heap_memory.len() * 4
            ));
        }

        println!("Loading program into heap memory");
        for (i, &inst) in program.iter().enumerate() {
            // println!("Loading Inst 0x{inst:08X} into memory");
            self.heap_memory.set(i as MemorySize, inst)?;
        }
        println!("Program loaded into heap memory");

        Ok(())
    }

    pub fn load_program<T>(&mut self, program: &[T]) -> Result<(), String>
    where
        T: Into<Instruction> + Copy,
    {
        let program = program
            .iter()
            .map(|&inst| Into::<Instruction>::into(inst).to_inner())
            .collect::<Vec<MemorySize>>();
        self.load_program_raw(&program)
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..32 {
            s.push_str(&format!("x{:<2}: {:#010x}\n", i, self.registers[i]));
        }
        s.push_str(&format!("pc: {:#010x}\n", self.pc));
        s
    }
}
