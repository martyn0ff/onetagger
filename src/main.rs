#![windows_subsystem = "windows"]

#[macro_use] extern crate slog;
#[macro_use] extern crate slog_scope;

use std::env;
use std::panic;
use std::fs::OpenOptions;
use std::sync::Mutex;
use backtrace::Backtrace;
use slog::{Drain, Duplicate, Logger, FnValue};

// Get timestamp macro
macro_rules! timestamp {
    () => {
        std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis()
    };
}

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

mod ui;
mod cli;
mod tag;
mod tagger;
mod playlist;


fn main() {
    // Logging setup
    let drain1 = slog_term::term_full();
    let log = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(ui::Settings::get_folder().unwrap().join("onetagger.log")) {
            // Log file
            Ok(file) => {
                let drain2 = slog_term::FullFormat::new(slog_term::PlainDecorator::new(file)).build().fuse();
                let both = Mutex::new(Duplicate::new(drain1, drain2)).fuse();
                Logger::root(both, o!("module" => FnValue(move |info| {
                    format!("{}", info.module())
                })))
            },
            // Only terminal
            Err(_) => {
                Logger::root(Mutex::new(drain1).fuse(), o!())
            }
        };
    let _guard = slog_scope::set_global_logger(log);
    // Panic hook
    panic::set_hook(Box::new(|p| {
        let bt = Backtrace::new();
        error!("PANIC: {}", p);
        if let Some(location) = p.location() {
            error!("LOCATION: File: {}, Line: {}", location.file(), location.line());
        }
        // Show backtrace
        if env::var_os("RUST_BACKTRACE").is_some() {
            debug!("BACKTRACE:\n{:?}", bt);
        }
    }));

    cli::parse_args();
}