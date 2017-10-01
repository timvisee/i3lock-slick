extern crate clap;

use std::cmp;
use std::process::Command;

use clap::{Arg, ArgMatches, App};

// Get application constants from Cargo
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

/// Main application entrypoint.
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
        .arg(Arg::with_name("parameter")
            .short("p")
            .long("parameter")
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
        .values_of("parameter")
        .unwrap();

    let mut arg_params: Vec<String> = Vec::new();

    // Process all i3 parameters
    for param in params {
        // Split the parameter in parts
        let mut parts = param.splitn(2, '=');

        // Get the parameter part
        let param_param = parts.next().unwrap();
        let mut arg_parsed = String::new();
        let mut arg_parsed_arg = String::new();

        // Put one or two hyphens in front, if it doesn't have them yet
        if !param_param.starts_with("-") {
            for _ in 0..cmp::min(param_param.len(), 2) {
                arg_parsed.push('-');
            }
        }

        // Add the actual parameter 
        arg_parsed.push_str(param_param);

        // Parse the parameter argument if set
        if let Some(param_arg_arg) = parts.next() {
            // Determine whether to attach the argument with an equals sign,
            // or whether to supply it detached value
            if arg_parsed.len() <= 2 {
                arg_parsed_arg.push_str(param_arg_arg);
            } else {
                arg_parsed.push('=');
                arg_parsed.push_str(param_arg_arg);
            }
        }

        // Add the arguments to the command list
        arg_params.push(arg_parsed);
        if !arg_parsed_arg.is_empty() {
            arg_params.push(arg_parsed_arg);
        }
    }

    arg_params
}
