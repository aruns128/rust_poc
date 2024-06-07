use chrono::Local;
use log4rs::{
    append::{
        rolling_file::{
            policy::compound::{
                roll::fixed_window::FixedWindowRoller, 
                trigger::size::SizeTrigger, 
                CompoundPolicy,
            },
            RollingFileAppender,
        },
        console::ConsoleAppender,
    },
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};
use std::env;
pub fn init_log() {
    dotenv::dotenv().ok();
    let log_level_str = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_level = match log_level_str.to_lowercase().as_str() {
        "error" => log::LevelFilter::Error,
        "warn" => log::LevelFilter::Warn,
        "info" => log::LevelFilter::Info,
        "debug" => log::LevelFilter::Debug,
        "trace" => log::LevelFilter::Trace,
        _ => log::LevelFilter::Info,
    };
    let datetime = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let current_file_path = std::file!(); // Gets the path of the current file
    let log_file_name = format!("{}/log_files/{}_application.log", std::path::Path::new(&current_file_path).parent().unwrap().display(), datetime);
    let pattern = "{d} - {l} - {m}\n";
    let size_trigger = SizeTrigger::new(100 * 1024); // For example, 100 KB; adjust based on your average line length
    let window_roller = FixedWindowRoller::builder().build("log_files/{}_application.{}.log", 10).unwrap();
    let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(window_roller));
    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build(log_file_name, Box::new(compound_policy))
        .expect("Failed to create RollingFileAppender.");
    let console = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(Appender::builder().build("console", Box::new(console)))
        .logger(Logger::builder().appender("console").build("console", log::LevelFilter::Info))
        .build(Root::builder().appender("logfile").appender("console").build(log_level))
        .expect("Failed to build logger configuration.");
    if let Err(e) = log4rs::init_config(config) {
        println!("Logger initialization failed: {}", e);
    }
}
 