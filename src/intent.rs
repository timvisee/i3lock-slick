extern crate config;

use std::cmp;
use std::collections::BTreeMap;
use std::process::Command;

use cmd;
use config::Config;
use err::{Error, Result};

/// Application intent, defining what this application instance is used for.
///
/// For example; this struct keeps track of the command and it's arguments to run when invoking the
/// lock screen.
///
/// This struct is usually built while processing configurations and arguments, and is used as
/// reference when finishing.
pub struct Intent {
    /// A list of commands to run.
    cmd: Vec<String>,
}

impl Intent {
    /// Construct the intent with the default properties.
    pub fn new() -> Self {
        Intent {
            cmd: vec!["i3lock".into()],
        }
    }

    /// Construct the intent based on a given configuration.
    pub fn from(config: &Config) -> Self {
        let mut intent = Self::new();

        // Get the current list of arguments or create a fresh one if non-existent
        let params = config.get_dict(cmd::ARG_PARAM, BTreeMap::new());

        // Process all i3 parameters
        for (mut arg, base_val) in params.unwrap() {
            // Prepend 1 or 2 argument hyphens if missing
            if !arg.starts_with("-") {
                for _ in 0..cmp::min(arg.len(), 2) {
                    arg.insert(0, '-');
                }
            }

            // Define an optional value variable
            let mut val: Option<String> = None;

            // Parse argument values if set
            if !base_val.is_empty() {
                // Determine whether to attach the argument and value with an equals sign,
                // or whether to separate them with a space.
                if arg.len() <= 2 {
                    val = Some(base_val);
                } else {
                    arg.push('=');
                    arg.push_str(&base_val);
                }
            }

            // Push the arguments to the intent
            intent.push_arg(arg);
            if val.is_some() {
                intent.push_arg(val.unwrap());
            }
        }

        intent
    }

    /// Put an additional argument into the intent.
    pub fn push_arg(&mut self, arg: String) {
        self.cmd.push(arg);
    }

    /// Invoke i3lock with this intent.
    pub fn run(&self) -> Result<()> {
        println!("Starting i3lock...");

        // Invoke i3lock
        let mut args_iter = self.cmd.iter();
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

        Ok(())
    }

    /// Build the command that is run when i3lock is normally invoked and return it.
    ///
    /// This function does not invoke i3lock.
    pub fn command(&self) -> String {
        self.cmd.join(" ")
    }
}