extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate shellexpand;
extern crate tempdir;

mod app;
mod cmd;
mod config;
mod err;
mod img;
mod intent;
mod yaml_helper;

use std::env;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};

use clap::{Arg, ArgMatches, App};
use tempdir::TempDir;

use config::Config;
use err::{Error, Result};
use img::img_proc::{Blur, blur, ImgProc};
use intent::Intent;

/// Main application entry point.
fn main() {
    // Create a configuration instance
    let mut config = Config::default();

    // List of paths to check for dotfiles at
    let paths = vec![
        "~/.config/i3lock-slick/config.yml",
        "~/.i3lock-slick.yml",
    ];

    // Merge existing dotfiles
    for path in paths {
        // Expand the path
        let path = shellexpand::full_with_context_no_errors(
            &path,
            || env::home_dir(),
            |var| env::var(var).ok(),
        ).to_string();

        // Get the path, load dotfile if it exists
        let path = Path::new(&path);
        if path.is_file() {
            println!("Load settings: {:?}", path);
            config.merge_file(path).unwrap();
        }
    }

    // Parse arguments
    config.parse_matches(&parse_args()).expect("Failed to parse CLI arguments");

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
        .arg(Arg::with_name(cmd::ARG_DRY)
            .short("d")
            .long(cmd::ARG_DRY)
            .help("Don't invoke i3lock, output the command to stdout instead"))
        .arg(Arg::with_name(cmd::ARG_FILTER)
            .short("f")
            .long(cmd::ARG_FILTER)
            .value_name("FILTER")
            .help("Filter to apply to the image")
            .multiple(true)
            .takes_value(true))
        .get_matches()
}

/// Show the lock screen
///
/// If `matches` are given, all parameters will be parsed accordingly.
fn lock<'a>(config: &'a mut Config) -> Result<'a, ()> {
    // Create a program intent
    let mut intent = Intent::from(config);

    // Create a temporary directory
    let temp = TempDir::new(app::NAME)
        .expect("Failed to create temporary directory");

    // Create a screenshot
    let screenshot = screenshot(&temp);

    // Configure to use the screenshot as lock image
    match screenshot {
        Ok(file) => {
            intent.push_arg("--image".into());
            intent.push_arg(file.to_str().unwrap().into());
        },
        _ => {}
    }

    // Invoke i3lock, or output it's command
    if !config.get_bool(cmd::ARG_DRY).unwrap_or(false) {
        intent.run().unwrap();
    } else {
        // TODO: Escape arguments with spaces and other weird characters?
        println!("{}", intent.command());
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
    blur.set_property(blur::PROP_SIGMA, "7").unwrap();
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
