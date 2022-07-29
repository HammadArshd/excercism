// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message)
    }
}
pub fn info(message: &str) -> String {
    let mut s  = String::from("[INFO]: ");
    s.push_str(message);
    s
}
pub fn warn(message: &str) -> String {
    let mut s  = String::from("[WARNING]: ");
    s.push_str(message);
    s
}
pub fn error(message: &str) -> String {
    let mut s  = String::from("[ERROR]: ");
    s.push_str(message);
    s
}
