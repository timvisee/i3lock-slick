extern crate clap;
extern crate config;
extern crate serde;

use std::cmp;
use std::sync::RwLock;

use cmd;
use err::{Error, Result};
use self::clap::ArgMatches;
use self::serde::Deserialize;

/// App configuration structure.
pub struct Config {
    cfg: RwLock<self::config::Config>,

    // TODO: Put this into some action intent (this is parsed data for after the configuration)
    pub cmd: Vec<String>,
}

impl Config {
    /// Constructor, with configuration defaults.
    pub fn new() -> Self {
        Config {
            cfg: RwLock::new(self::config::Config::default()),

            cmd: vec![String::from("i3lock")],
        }
    }

    /// Get the given property by it's `key`.
    pub fn property<'de, T: Deserialize<'de>>(&self, key: &'de str) -> Result<T> {
        match self.cfg.read() {
            Ok(property) =>
                match property.get(key) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(Error::new("Failed to parse property.")),
                },
            Err(_) => Err(Error::new("Failed to read property")),
        }
    }

    /// Parse a set of command line argument matches.
    pub fn parse_matches(&mut self, matches: &ArgMatches) -> Result<()> {
        self.parse_i3_params(matches);

        // Fake running
        if matches.is_present(cmd::ARG_FAKE) {
            self.cfg.write()?.set(cmd::ARG_FAKE, true)?;
        }

        Ok(())
    }

    /// Parse parameters that should be passed to i3lock if any matched.
    ///
    /// Returns a vector of strings with arguments to use when invoking i3lock.
    fn parse_i3_params(&mut self, matches: &ArgMatches) {
        // Get all parameters
        let params = matches.values_of(cmd::ARG_PARAMS);
        if params.is_none() {
            return;
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

        self.cmd.append(&mut args);
    }
}