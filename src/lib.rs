pub mod bus;
pub mod cpu;
pub mod csr;
pub mod devices;
pub mod interrupt;
pub mod memory;
pub mod registers;
pub mod rom;
pub mod trap;

#[cfg(feature = "logging")]
pub mod logging {
    pub use colored;
    pub use fern;
    pub use log;
    use log::LevelFilter;

    #[macro_export]
    macro_rules! log_trace {
        ($($arg:tt)+) => {
            log::trace!($($arg)+)
        };
    }

    #[macro_export]
    macro_rules! log_debug {
        ($($arg:tt)+) => {
            log::debug!($($arg)+)
        };
    }

    #[macro_export]
    macro_rules! log_info {
        ($($arg:tt)+) => {
            log::info!($($arg)+)
        };
    }

    #[macro_export]
    macro_rules! log_warn {
        ($($arg:tt)*) => {
            log::warn!($($arg)*)
        };
    }

    #[macro_export]
    macro_rules! log_error {
        ($($arg:tt)*) => {
            log::error!($($arg)*)
        };
    }

    pub fn init_logging(level: LevelFilter) {
        use crate::logging::colored::{Color, Colorize};
        use crate::logging::fern::{colors::ColoredLevelConfig, Dispatch};

        let colors = ColoredLevelConfig::new()
            .info(fern::colors::Color::Green)
            .debug(fern::colors::Color::Cyan)
            .error(fern::colors::Color::Red);
        Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "[{level}][{target}][{time}] {message}",
                    time = chrono::Local::now()
                        .format("%H:%M:%S")
                        .to_string()
                        .color(Color::BrightBlue),
                    target = record.target().color(Color::Magenta),
                    level = colors.color(record.level()),
                    message = message,
                ))
            })
            .level(level)
            .chain(std::io::stdout())
            .apply()
            .unwrap();
    }
}

#[inline]
/// Used to convert a slice of bytes to a slice of u32 and ensure that the ouput slice is little endian
pub fn convert_memory(data: &[u8]) -> Vec<u32> {
    let mut program = Vec::new();
    for bytes in data.chunks_exact(4) {
        let word = {
            let word = u32::from_ne_bytes(bytes.try_into().unwrap());
            word.to_le()
        };
        program.push(word);
    }
    program
}

pub fn disassemble(program: &[u32], file: &str) {
    use riscv_decoder::decoder::*;
    use std::{fs::File, io::Write};

    let mut file = File::create(file).expect("Failed to create file");

    let mut pc = 0_usize;
    while pc < program.len() {
        // debug_assert!(pc % 4 != 0, "Pc must be aligned to 4 bytes {{ PC: {pc:#X} }}");
        match program.get(pc) {
            Some(&inst) => {
                let dinst = try_decode(inst); /*if is_compressed(inst) {
                                                  pc += 2; try_decode_compressed(inst)
                                              } else {
                                                  pc += 4; try_decode(inst)
                                              };*/
                pc += 4;
                writeln!(
                    file,
                    "{:#X}: {}",
                    pc + memory::dram::DRAM_BASE as usize,
                    match dinst {
                        Ok(inst) => format!("{inst}"),
                        Err(e) => format!("Error => {e} {{ instruction: {inst:#X} }}"),
                    }
                )
                .expect("Failed to write to file");
            }
            None => {
                writeln!(file, "{pc:#010x}: EOF / End of indexs").expect("Failed to write to file");
                break;
            }
        }
    }
}

// internal export
pub mod bit_ops {
    pub use bit_ops::bitops_u32::*;

    pub fn zero_extend(value: u32) -> u32 {
        clear_bit(value, 31)
    }
}
