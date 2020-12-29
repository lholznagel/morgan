use log::{Level, Log, Metadata, Record};
use std::time::Instant;

/// The logger instance
/// To configure the logger, use the `LogBuilder` struct
pub struct Morgan {
    time: Instant,
    color: bool,
    exclude: Vec<String>
}

impl Morgan {
    /// Creates a new logger using the default values
    ///
    /// # Defaults
    /// - `level` -> The default level is `Info`
    /// - `exclude` -> No targets are excluded
    ///
    /// # Example
    /// ```
    /// #[macro_use]
    /// extern crate log;
    /// extern crate morgan;
    ///
    /// use morgan::Morgan;
    ///
    /// fn main() {
    ///     Morgan::init(Vec::new()).unwrap();
    ///
    ///     log::error!("My error message");
    ///     log::warn!("My warn message");
    ///     log::info!("My info message");
    ///     log::debug!("Will not be shown");
    ///     log::trace!("Will not be shown");
    /// }
    /// ```
    pub fn init(exclude: Vec<String>) {
        let morgan = Self {
            time: Instant::now(),
            color: std::env::var("MORGAN_COLOR")
                .map(|x| x.parse().unwrap_or(true))
                .unwrap_or(true),
            exclude
        };
        if let Err(e) = log::set_boxed_logger(Box::new(morgan)) {
            println!("Error setting logger {:?}", e);
        }

        let log_level = std::env::var("MORGAN_LEVEL")
            .map(|x| match x.to_lowercase().as_ref() {
                "debug" => Level::Debug,
                "error" => Level::Error,
                _ => Level::Info,
            })
            .unwrap_or(Level::Info);
        log::set_max_level(log_level.to_level_filter());
    }
}

impl Log for Morgan {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let base = metadata.target().split(':').next().unwrap_or_default();
        !self.exclude.contains(&base.into())
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let lvl_msg = if self.color {
            match record.level() {
                Level::Error => "\x1B[1;31mError \x1B",
                Level::Warn => "\x1B[1;93mWarn  \x1B",
                Level::Info => "\x1B[1;34mInfo  \x1B",
                Level::Debug => "\x1B[1;35mDebug \x1B",
                Level::Trace => "\x1B[1;36mTrace \x1B",
            }
        } else {
            match record.level() {
                Level::Error => "Error ",
                Level::Warn => "Warn  ",
                Level::Info => "Info  ",
                Level::Debug => "Debug ",
                Level::Trace => "Trace ",
            }
        };

        if self.color {
            println!(
                "\x1B[1;90m[{:10.3?}] > \x1B {}[1;90m>\x1B[1;39m {}",
                self.time.elapsed().as_secs_f64(),
                lvl_msg,
                record.args()
            );
        } else {
            println!(
                "[{:10.3?}] > {} > {}",
                self.time.elapsed().as_secs_f64(),
                lvl_msg,
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
