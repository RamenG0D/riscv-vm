pub mod bus;
pub mod cpu;
pub mod csr;
pub mod interrupt;
pub mod memory;
pub mod registers;
pub mod rom;
pub mod trap;

use std::io::IsTerminal;

pub use log;

pub fn init_logging(level: log::LevelFilter) {
    use fern::{colors::ColoredLevelConfig, Dispatch};

    // check if our env supports colors
    let colors = if std::io::stdin().is_terminal() {
        ColoredLevelConfig::new()
            .info(fern::colors::Color::Green)
            .debug(fern::colors::Color::Cyan)
            .error(fern::colors::Color::Red)
    } else {
        ColoredLevelConfig::new()
    };

    // terminal output
    let terminal = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{level}][{target}][{time}] {message}",
                time = chrono::Local::now().format("%H:%M:%S"),
                target = record.target(),
                level = colors.color(record.level()),
            ))
        })
        .filter(|meta| !matches!(meta.target(), "init" | "execution"))
        .chain(std::io::stdout());

    // log file output
    let log_file = fern::log_file("riscv_vm.log").expect("Failed to create log file");
    let file = Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{level}][{target}][{time}] {message}",
                time = chrono::Local::now().format("%H:%M:%S"),
                target = record.target(),
                level = record.level(),
            ))
        })
        .chain(log_file);

    let err = Dispatch::new()
        .level(level)
        .chain(terminal)
        .chain(file)
        .apply();

    match err {
        Ok(_) => {
            log::info!(target: "init", "Logging initialized with level: {level}");
            // command to get distribution info
            let mut dist = std::process::Command::new("uname");
            let output = dist
                .arg("-o")
                .arg("-r")
                .arg("-s")
                .output()
                .expect("Failed to get distribution info");
            let mut output = output.stdout.iter().map(|x| *x as char).collect::<String>();
            // remove any newlines
            output.retain(|c| c != '\n' && c != '\r');

            log::info!(target: "init", "Distribution: {}", output);
            log::info!(target: "init", "Arch: {}", std::env::consts::ARCH);

            // log::info!("Build: {}", std::env!("CARGO_BUILD_TARGET"));
            log::info!(target: "init", "Commit: {}", option_env!("GIT_COMMIT").unwrap_or("None"));
            log::info!(target: "init", "Branch: {}", option_env!("GIT_BRANCH").unwrap_or("None"));
            log::info!(target: "init", "Tag: {}", option_env!("GIT_TAG").unwrap_or("None"));
            log::info!(target: "init", "Rust version: {}", env!("RUST_VERSION"));
            log::info!(target: "init", "Version: {}", env!("CARGO_PKG_VERSION"));
            log::info!(target: "init", "Repository: {}", env!("CARGO_PKG_REPOSITORY"));
        }
        Err(e) => eprintln!("Failed to initialize logging: {e}"),
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
