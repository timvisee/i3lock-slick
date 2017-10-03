extern crate clap;
extern crate tempdir;

mod app;
mod cmd;
mod config;
mod err;
mod img;

use std::path::PathBuf;
use std::process::{Command, exit};

use clap::{Arg, ArgMatches, App};
use config::Config;
use err::{Error, Result};
use img::img_proc::{Blur, blur, ImgProc};
use tempdir::TempDir;

/// Main application entry point.
fn main() {
    // Create a configuration instance
    let mut config = Config::new();

    // Parse arguments
    config.parse_matches(&parse_args());

    // Show the lock screen
    let result = lock(&mut config);
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
fn lock<'a>(config: &mut Config) -> Result<'a, ()> {
    // Create a temporary directory
    let temp = TempDir::new(app::NAME)
        .expect("Failed to create temporary directory");

    // Create a screenshot
    let screenshot = screenshot(&temp);

    // Configure to use the screenshot as lock image
    match screenshot {
        Ok(file) => {
            config.cmd.push("--image".into());
            config.cmd.push(file.to_str().unwrap().into());
        },
        _ => {}
    }

    // Invoke i3lock
    if !config.fake {
        println!("Starting i3lock...");

        // Invoke i3lock
        let mut args_iter = config.cmd.iter();
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
        println!("{}", config.cmd.join(" "));
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
    let mut edit = img.edit().unwrap();

    println!("Bluring image...");
    let mut blur = Blur::new();
    blur.set_property(blur::PROP_SIGMA, "3".into());
    edit = blur.process_safe(edit).unwrap();

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
