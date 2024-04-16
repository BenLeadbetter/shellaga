#[cfg(target_os = "macos")]
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    oslog::OsLogger::new("com.benleadbetter.shellaga")
        .level_filter(level())
        .init()?;
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    eventlog::init("shellaga", level())?;
    Ok(())
}


#[cfg(target_os = "linux")]
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    let formatter = syslog::Formatter3164 {
        facility: syslog::Facility::LOG_USER,
        hostname: None,
        process: "myprogram".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter)?;
    log::set_boxed_logger(Box::new(syslog::BasicLogger::new(logger)))
        .map(|()| log::set_max_level(level()))?;
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
