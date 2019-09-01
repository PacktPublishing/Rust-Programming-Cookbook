use log::{debug, error, info, trace, warn};

mod custom {

    pub use log::Level;
    use log::{Metadata, Record};

    pub struct EmojiLogger {
        pub level: Level,
    }

    impl log::Log for EmojiLogger {

        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= self.level
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let level = match record.level() {
                    Level::Warn => "⚠️",
                    Level::Info => "ℹ️",
                    Level::Debug => "🐛",
                    Level::Trace => "💡",
                    Level::Error => "☢️",
                };

                let utc = time::now_utc();
                println!("{} | [{}] | {:<5}", utc.rfc3339(), record.target(), level);
                println!("{:21} {}", "", record.args());
            }
        }

        fn flush(&self) {}
    }
}

static LOGGER: custom::EmojiLogger = custom::EmojiLogger {
    level: log::Level::Trace,
};

fn log_some_stuff() {
    let a = 100;

    trace!("TRACE: Called log_some_stuff()");
    debug!("DEBUG: a = {} ", a);
    info!("INFO: The weather is fine");
    warn!("WARNING, stuff is breaking down");
    warn!(target: "special-target", "WARNING, stuff is breaking down");
    error!("ERROR: stopping ...");
}

const USE_CUSTOM: bool = true;

fn main() {
    if USE_CUSTOM { 
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Trace))
        .unwrap();
    } else {
        log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    }
    log_some_stuff();
}
