extern crate clap;
extern crate config;
extern crate serde;

use std::collections::HashMap;
use std::sync::RwLock;

use cmd;
use err::{Error, Result};
use self::clap::ArgMatches;
use self::serde::Deserialize;

/// App configuration structure.
pub struct Config {
    cfg: RwLock<self::config::Config>,
}

impl Config {
    /// Constructor, with configuration defaults.
    pub fn new() -> Self {
        Config {
            cfg: RwLock::new(self::config::Config::default()),
        }
    }

    /// Get the given property by it's `key`.
    pub fn get<'de, T: Deserialize<'de>>(&self, key: &'de str) -> Result<T> {
        match self.cfg.read() {
            Ok(property) =>
                match property.get(key) {
                    Ok(value) => Ok(value),
                    Err(_) => Err(Error::new("Failed to parse property.")),
                },
            Err(_) => Err(Error::new("Failed to read property")),
        }
    }

    /// Check whether a given property is true or false, by it's `key`.
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        self.get(key)
    }

    /// Get a key value dictionary as `(String, String)` in a `HashMap` for the given key.
    ///
    /// The `def` value is returned if the given property was not found.
    ///
    /// Errors are returned if parsing the dictionary resulted in a problem.
    pub fn get_dict(&self, key: &str, def: HashMap<String, String>) -> Result<HashMap<String, String>> {
        match self.cfg.read()?.get_table(key) {
            Ok(table) => Ok(
                table
                .into_iter()
                .map(|(key, val)| (key, val.into_str().unwrap()))
                .collect()
            ),
            Err(config::ConfigError::NotFound(_)) => Ok(def),
            Err(err) => return Err(err.into()),
        }
    }

    /// Parse a set of command line argument matches.
    pub fn parse_matches(&mut self, matches: &ArgMatches) -> Result<()> {
        // TODO: Don't unwrap, but try! the result!
        self.parse_i3_params(matches).unwrap();

        // Fake running
        if matches.is_present(cmd::ARG_FAKE) {
            self.cfg.write()?.set(cmd::ARG_FAKE, true)?;
        }

        Ok(())
    }

    /// Parse parameters that should be passed to i3lock if any matched.
    ///
    /// The configuration is modified directly with the parsed arguments,
    /// and nothing is returned on success.
    ///
    /// Any errors may be returned if parsing failed.
    fn parse_i3_params(&mut self, matches: &ArgMatches) -> Result<()> {
        // Return early if there are no arguments to parse
        let params = matches.values_of(cmd::ARG_PARAMS);
        if params.is_none() {
            return Ok(());
        }

        // Get the current list of arguments or create a fresh one if non-existent
        let mut cfg_params = self.get_dict(cmd::ARG_PARAMS, HashMap::new())?;

        // Process all i3 parameters
        for param in params.unwrap() {
            // Split the parameter in parts
            let mut parts = param.splitn(2, '=');

            // Get the argument and value
            let arg = parts.next().unwrap().into();
            let val: Option<String> = parts.next().map(|val| val.into());

            // TODO: Maybe strip prefixed hyphens from arguments? They're automatically added in the Intent.

            // Add the argument to the params
            cfg_params.insert(
                arg,
                val.unwrap_or("".into())
            );
        }

        // Set the properties in the configuration
        self.cfg.write()?.set(cmd::ARG_PARAMS, cfg_params)?;
        Ok(())
    }
}