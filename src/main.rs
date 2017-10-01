extern crate clap;

use std::cmp;
use std::fmt::{Display, Formatter};
use std::process::{Command, exit};

use clap::{Arg, ArgMatches, App};
//use screenshot::get_screenshot;

// Get application constants from Cargo
const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const APP_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

// Command constant
const CMD_ARG_PARAMS: &'static str = "parameter";
const CMD_ARG_FAKE: &'static str = "fake";

// Application result type
type Result<'a, T> = std::result::Result<T, Error<'a>>;

/// Main application entry point.
fn main() {
    // Parse arguments
    let matches = parse_args();

    // Show the lock screen
    let result = lock(Some(&matches));
    if result.is_err() {
        eprintln!("{}\n{} will now quit", result.unwrap_err(), APP_NAME);
        exit(1);
    }
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
        .arg(Arg::with_name(CMD_ARG_FAKE)
            .short("f")
            .long(CMD_ARG_FAKE)
            .help("Don't invoke i3lock, print the command to stdout instead"))
        .get_matches()
}

/// Show the lock screen
///
/// If `matches` are given, all parameters will be parsed accordingly.
fn lock<'a>(matches: Option<&ArgMatches>) -> Result<'a, ()> {
    // Build the command and it's arguments
    let mut cmd: Vec<String> = vec![String::from("i3lock")];
    let mut fake = false;

    // Parse parameters
    if let Some(matches) = matches {
        cmd.append(&mut parse_i3_params(matches));

        // Fake running i3lock
        if matches.is_present(CMD_ARG_FAKE) {
            fake = true;
        }
    }

    // Invoke i3lock
    if !fake {
        println!("Starting i3lock...");

        // Invoke i3lock
        let mut args_iter = cmd.iter();
        let out = Command::new(args_iter.next().unwrap())
            .args(args_iter)
            .output()
            .expect("Failed to invoke i3lock");

        // Wait for i3lock to complete, handle non-zero status codes
        if out.status.success() {
            println!("i3lock exited successfully");
        } else {
            println!(
                "i3lock exited with a non-zero status code (code: {})",
                out.status.code().unwrap()
            );
        }

        // Print stdout and stderr from i3lock if not empty
        if !out.stdout.is_empty() {
            println!("\ni3lock stdout:");
            println!("==========");
            println!("{}", String::from_utf8_lossy(&out.stdout));
            println!("==========");
        }
        if !out.stderr.is_empty() {
            println!("\ni3lock stderr:");
            println!("==========");
            println!("{}", String::from_utf8_lossy(&out.stderr));
            println!("==========");
        }

        // Return errors
        if !out.status.success() {
            return Err(Error::new("i3lock exited with a non-zero status code"));
        }

    } else {
        // Don't invoke i3lock, print the command to stdout instead
        println!("{}", cmd.join(" "));
    }

    Ok(())
}

/// Parse parameters that should be passed to i3lock if any matched.
///
/// Returns a vector of strings with arguments to use when invoking i3lock.
fn parse_i3_params(matches: &ArgMatches) -> Vec<String> {
    // Get all parameters
    let params = matches.values_of(CMD_ARG_PARAMS);
    if params.is_none() {
        return Vec::new();
    }

    // Create a list of arguments to use
    let mut args: Vec<String> = Vec::new();

    // Process all i3 parameters
    for param in params.unwrap() {
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

#[derive(Debug)]
struct Error<'a> {
    description: &'a str,
    cause: Option<&'a std::error::Error>
}

impl<'a> Error<'a> {
    /// New error instance, with the given `description`.
    pub fn new(description: &'a str) -> Self {
        Error {
            description,
            cause: None,
        }
    }

//    /// New error instance, with the given `description` and `cause`.
//    pub fn from(description: &'a str, cause: &'a std::error::Error) -> Self {
//        Error {
//            description,
//            cause: Some(cause),
//        }
//    }
}

impl<'a> std::error::Error for Error<'a> {
    fn description(&self) -> &str {
        self.description
    }

    fn cause(&self) -> Option<&std::error::Error> {
        self.cause
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{} error: {}", APP_NAME, self.description)
    }
}