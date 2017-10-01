extern crate clap;

use std::cmp;
use std::process::Command;

use clap::{Arg, ArgMatches, App};

// Get application constants from Cargo
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

// Command constant
const CMD_ARG_PARAMS: &'static str = "parameter";

/// Main application entry point.
fn main() {
    // Parse arguments
    let matches = parse_args();

    // Show the lock screen
    lock(Some(&matches));
}

/// Parse all given arguments.
///
/// A `ArgMatches` struct is returned holding all matches.
fn parse_args<'a>() -> ArgMatches<'a> {
    App::new(APP_NAME)
        .version(APP_VERSION)
        .about(APP_DESCRIPTION)
        .author(APP_AUTHOR)
        .arg(Arg::with_name(CMD_ARG_PARAMS)
            .short("p")
            .long(CMD_ARG_PARAMS)
            .value_name("ARGUMENT | ARGUMENT=VALUE")
            .help("Pass an argument to i3lock")
            .multiple(true)
            .takes_value(true))
        .get_matches()
}

/// Show the lock screen
///
/// If `matches` are given, all parameters will be parsed accordingly.
fn lock(matches: Option<&ArgMatches>) {
    // Create a list of arguments to add
    let mut args: Vec<String> = Vec::new();

    // Parse i3 parameters
    if matches.is_some() {
        args = parse_i3_params(matches.unwrap());
    }

    // Invoke i3lock
    Command::new("i3lock")
        .args(args)
        .spawn()
        .expect("Failed to invoke i3lock");
}

/// Parse parameters that should be passed to i3lock if any matched.
///
/// Returns a vector of strings with arguments to use when invoking i3lock.
fn parse_i3_params(matches: &ArgMatches) -> Vec<String> {
    // Get all parameters
    let params = matches
        .values_of(CMD_ARG_PARAMS)
        .unwrap();

    // Create a list of arguments to use
    let mut args: Vec<String> = Vec::new();

    // Process all i3 parameters
    for param in params {
        // Split the parameter in parts
        let mut parts = param.splitn(2, '=');

        // Get the argument, define variables for the argument and a possible value
        let part_arg = parts.next().unwrap();
        let mut arg = String::new();
        let mut val: Option<String> = None;

        // Prefix 1 or 2 argument hyphens if missing
        if !part_arg.starts_with("-") {
            for _ in 0..cmp::min(part_arg.len(), 2) {
                arg.push('-');
            }
        }

        // Append the actual argument after the hyphens
        arg.push_str(part_arg);

        // Parse argument values if set
        if let Some(part_val) = parts.next() {
            // Determine whether to attach the argument and value with an equals sign,
            // or whether to separate them with a space.
            if arg.len() <= 2 {
                val = Some(part_val.into());
            } else {
                arg.push('=');
                arg.push_str(part_val);
            }
        }

        // Push the arguments to the result
        args.push(arg);
        if val.is_some() {
            args.push(val.unwrap());
        }
    }

    args
}
