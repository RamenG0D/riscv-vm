use colored::{Color, Colorize};
use fern::{colors::ColoredLevelConfig, Dispatch};

pub fn init_logging() {
    let colors = ColoredLevelConfig::new()
        .info(fern::colors::Color::Green)
        .debug(fern::colors::Color::Cyan)
        .error(fern::colors::Color::Red);
    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{level}][{target}][{date}][{time}] {message}",
                date = chrono::Local::now()
                    .format("%d-%m-%Y")
                    .to_string()
                    .color(Color::Green),
                time = chrono::Local::now()
                    .format("%H:%M:%S")
                    .to_string()
                    .color(Color::BrightBlue),
                target = record.target().color(Color::Magenta),
                level = colors.color(record.level()),
                message = message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
