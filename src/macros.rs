use colored::{ColoredString, Colorize};
use std::process::exit;
pub fn err_out(message: &str, status_code: i32) {
    msg("Error".red(), message);
    exit(status_code)
}

pub fn msg(preceding_str: ColoredString, message: &str) {
    println!("{}: {}", preceding_str, message);
}

#[macro_export]
macro_rules! err {
    ($message: expr) => {
        err_out($message, 1)
    };
    ($message: expr, $err_code: expr) => {
        err_out($message, $err_code)
    };
}
#[macro_export]
macro_rules! debug {
    ($message: expr) => {
        msg("Debug".yellow(), $message)
    };
}
#[macro_export]
macro_rules! info {
    ($message: expr) => {
        msg("Info".green(), $message)
    };
}
