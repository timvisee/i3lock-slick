extern crate clap;

use std::process::Command;

use clap::{Arg, App};

// Get application constants from Cargo
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

/// Main application entrypoint.
fn main() {
    parse_args();

    // Show the lock screen
    lock();
}

/// Parse all given arguments.
fn parse_args() {
    let _ = App::new(APP_NAME)
        .version(APP_VERSION)
        .about(APP_DESCRIPTION)
        .author(APP_AUTHOR)
        .get_matches();
}

/// Show the lock screen
fn lock() {
    // Invoke i3lock
    Command::new("i3lock")
        .arg("--clock")
        .spawn()
        .expect("Failed to invoke i3lock");
}
