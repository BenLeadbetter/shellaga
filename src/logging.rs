pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    oslog::OsLogger::new("com.benleadbetter.shellaga")
        .level_filter(level())
        .init()?;
    Ok(())
}

fn level() -> log::LevelFilter {
    match std::env::var("SHELLAGA_LOG_LEVEL") {
        Ok(value) => match &*value.to_lowercase() {
            "off" => log::LevelFilter::Off,
            "error" => log::LevelFilter::Error,
            "warn" => log::LevelFilter::Warn,
            "info" => log::LevelFilter::Info,
            "debug" => log::LevelFilter::Debug,
            "trace" => log::LevelFilter::Trace,
            _ => log::LevelFilter::Warn,
        },
        Err(_) => log::LevelFilter::Warn,
    }
}
