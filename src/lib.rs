pub mod bus;
pub mod cpu;
pub mod csr;
pub mod instruction_sets;
pub mod logging;
pub mod memory;
pub mod registers;
pub mod trap;
pub mod devices;

// internal export
pub mod bit_ops {
    pub use bit_ops::bitops_u32::*;

    pub fn zero_extend(value: u32) -> u32 {
        clear_bit(value, 31)
    }
}
