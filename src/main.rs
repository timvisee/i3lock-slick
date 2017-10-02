extern crate clap;
extern crate tempdir;

mod app;
mod cmd;
mod err;
mod img;

use std::cmp;
use std::path::PathBuf;
use std::process::{Command, exit};

use clap::{Arg, ArgMatches, App};
use err::{Error, Result};
use img::Img;
use tempdir::TempDir;

/// Main application entry point.
fn main() {
    // Parse arguments
    let matches = parse_args();

    // Create a temporary directory
    let temp = TempDir::new(app::NAME)
        .expect("Failed to create temporary directory");

    let screenshot = screenshot(&temp).unwrap();

    // Show the lock screen
    let result = lock(Some(&matches), Some(screenshot));
    if result.is_err() {
        eprintln!("{}\n{} will now quit", result.unwrap_err(), app::NAME);
        exit(1);
    }
}

/// Parse all given arguments.
///
/// A `ArgMatches` struct is returned holding all matches.
fn parse_args<'a>() -> ArgMatches<'a> {
    App::new(app::NAME)
        .version(app::VERSION)
        .about(app::DESCRIPTION)
        .author(app::AUTHOR)
        .arg(Arg::with_name(cmd::ARG_PARAMS)
            .short("p")
            .long(cmd::ARG_PARAMS)
            .value_name("ARGUMENT | ARGUMENT=VALUE")
            .help("Pass an argument to i3lock")
            .multiple(true)
            .takes_value(true))
        .arg(Arg::with_name(cmd::ARG_FAKE)
            .short("f")
            .long(cmd::ARG_FAKE)
            .help("Don't invoke i3lock, print the command to stdout instead"))
        .get_matches()
}

/// Show the lock screen
///
/// If `matches` are given, all parameters will be parsed accordingly.
fn lock<'a>(matches: Option<&ArgMatches>, screenshot: Option<PathBuf>) -> Result<'a, ()> {
    // Build the command and it's arguments
    let mut cmd: Vec<String> = vec![String::from("i3lock")];
    let mut fake = false;

    // Parse parameters
    if let Some(matches) = matches {
        cmd.append(&mut parse_i3_params(matches));

        // Fake running i3lock
        if matches.is_present(cmd::ARG_FAKE) {
            fake = true;
        }
    }

    // Configure to use the screenshot as lock image
    match screenshot {
        Some(file) => {
            cmd.push("--image".into());
            cmd.push(file.to_str().unwrap().into());
        },
        _ => {}
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

/// Take a screenshot and save in the given temporary directory.
///
/// Returns a `Path` which references the saved screenshot.
fn screenshot<'a>(tempdir: &TempDir) -> Result<'a, PathBuf> {
    // Determine the file path for the screenshot
    let file = tempdir.path().join("i3lock-image.png");

    // Invoke i3lock
    println!("Taking screenshot...");
    let out = Command::new("scrot")
        .arg("-z")
        .arg(file.to_str().unwrap())
        .output()
        .expect("Failed to invoke i3lock");

    // Process the image
    let img = img::Img::new(&file);
    let mut edit = Img::new(&file).edit().unwrap();

    println!("Bluring image...");
    edit.blur();

    println!("Saving edited image...");
    if let Err(_) = edit.save(&img) {
        return Err(Error::new("Failed to save image"));
    }

    // Wait for i3lock to complete, handle non-zero status codes
    if !out.status.success() {
        println!(
            "Failed to take screenshot (scrot status code: {})",
            out.status.code().unwrap()
        );

        if !out.stderr.is_empty() {
            println!("\nscrot stderr:");
            println!("==========");
            println!("{}", String::from_utf8_lossy(&out.stderr));
            println!("==========");
        }

        return Err(Error::new("Failed to take screenshot"));
    }

    Ok(file)
}

/// Parse parameters that should be passed to i3lock if any matched.
///
/// Returns a vector of strings with arguments to use when invoking i3lock.
fn parse_i3_params(matches: &ArgMatches) -> Vec<String> {
    // Get all parameters
    let params = matches.values_of(cmd::ARG_PARAMS);
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
