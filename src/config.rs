extern crate clap;
extern crate config;
extern crate yaml_rust;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use self::clap::ArgMatches;
use self::yaml_rust::{Yaml, YamlLoader};

use cmd;
use err::{Error, Result};
use yaml_helper::YamlHelper;

/// App configuration structure.
pub struct Config {
    data: Option<Yaml>,
}

impl Config {
    /// Constructor, with configuration defaults.
    pub fn default() -> Self {
        Config {
            data: None,
        }
    }

//    /// Load a YAML configuration from a `file`, and merge it with the default configuration.
//    ///
//    /// Returns an error if loading or parsing failed.
//    pub fn from(file: &Path) -> Result<Self> {
//        // Get the default config
//        let mut config = Config::default();
//
//        // Merge the file to load from
//        config.merge_file(file)?;
//
//        Ok(config)
//    }

    /// Merge the current configuration with the `other` given Yaml.
    ///
    /// `other` overrides the current configuration.
    fn merge(&mut self, other: Yaml) -> Result<()> {
        // Set the data if it's none
        if self.data.is_none() {
            self.data = Some(other);
            return Ok(());
        }

        // Get the other hash
        match other {
            Yaml::Hash(other_root) =>
                // Get the mutable data hash
                // TODO: Don't unwrap, unsafe
                match *self.data.as_mut().unwrap() {
                    Yaml::Hash(ref mut root) => {
                        // Merge the hashes
                        // TODO: Refactor this into a functional for_each
                        for (key, value) in other_root.into_iter() {
                            root.insert(key, value);
                        }

                        Ok(())
                    },

                    _ => Err(Error::new("Failed to access configuration root")),
                },

            _ => Err(Error::new("Failed to access other configuration root"))
        }
    }

    /// Merge the file at the given `path` into the configuration.
    ///
    /// Any load or parse errors are returned if merging failed.
    pub fn merge_file<'a>(&'a mut self, path: &Path) -> Result<'a, ()> {
        // Open the file
        let mut file = File::open(path)?;

        // Read the file contents
        let mut source = String::new();
        file.read_to_string(&mut source)?;

        // Load the first YAML document, and merge it
        match YamlLoader::load_from_str(&source)?.into_iter().next() {
            Some(doc) => {
                self.merge(doc)?;
                Ok(())
            },
            None => Ok(()),
        }
    }

    /// Get the Yaml property at the given `node`.
    ///
    /// `None` is returned if the node doesn't exist.
    pub fn get<'a: 'b, 'b>(&'a self, node: &'b str) -> Option<&'b Yaml> {
        // Return if no data is loaded
        if self.data.is_none() {
            return None;
        }

        self.data.as_ref().unwrap().property(node)
    }

    /// Set the Yaml property at the given `node`.
    pub fn set(&mut self, node: &str, value: Yaml) -> Result<()> {
        // Initialize the configuration
        if self.data.is_none() {
            self.data = Some(Yaml::Hash(BTreeMap::new()));
        }

        self.data.as_mut().unwrap().set_property(node, value)
    }

    /// Check whether a given property is true or false, by it's `node`.
    ///
    /// None is returned if the property doesn't exist, or if it isn't a boolean.
    pub fn get_bool(&self, node: &str) -> Option<bool> {
        match self.get(node) {
            Some(property) =>
                property.as_bool(),
            None => None,
        }
    }

    /// Get the list of Yaml values from the given `node`.
    ///
    /// If the property does not exist, `def` is returned.
    pub fn get_list(&self, node: &str, def: Vec<Yaml>) -> Vec<Yaml> {
        match self.get(node) {
            Some(property) => {
                // Get the vector
                let vec: Option<&Vec<Yaml>> = property.as_vec();
                if vec.is_none() {
                    return def;
                }

                // Clone it into a new list
                vec.unwrap().clone().into_iter().collect()
            },
            None => def,
        }
    }

    /// Set a list of Yaml values in the configuration.
    pub fn set_list(&mut self, node: &str, list: Vec<Yaml>) -> Result<()> {
        self.set(node, Yaml::Array(list))
    }

    /// Get a key value dictionary as `(String, String)` in a `HashMap` at the given `node`.
    ///
    /// The `def` value is returned if the given property was not found.
    ///
    /// Errors are returned if parsing the dictionary resulted in a problem.
    pub fn get_dict<'a>(&self, node: &str, def: BTreeMap<String, String>) -> Result<'a, BTreeMap<String, String>> {
        // The data must be available
        match self.data.as_ref() {
            Some(data) =>
                // The given node must be available
                match data.property(node) {
                    Some(object) => {
                        // Parse the object as dictionary
                        match object.as_hash() {
                            Some(map) => {
                                // Map the Yaml objects into string results
                                // TODO: Somehow collect errors here
                                Ok(
                                    map.into_iter()
                                        .map(|(key, val)| (
                                            // Map the Yaml node and value into owned strings
                                            key.as_str().unwrap().into(),
                                            val.as_str().unwrap().into(),
                                        ))
                                        .collect()
                                )
                            },
                            None =>
                                Err(Error::new("The property is not in Hash format, unable to parse it as dictionary"))
                        }
                    },
                    None => Ok(def),
                },
            None => Ok(def),
        }
    }

    /// Set a (String, String) dictionary at the given `node` in the configuration.
    pub fn set_dict(&mut self, node: &str, dict: BTreeMap<String, String>) -> Result<()> {
        self.set(
            node,
            Yaml::Hash(
                dict.into_iter()
                    .map(|(ref key, ref val)| (
                        Yaml::from_str(key),
                        Yaml::from_str(val),
                    ))
                    .collect()
            )
        )
    }

    /// Parse a set of command line argument matches.
    pub fn parse_matches(&mut self, matches: &ArgMatches) -> Result<()> {
        // TODO: Don't unwrap, but try! the result!
        self.parse_i3_params(matches).unwrap();
        self.parse_filters(matches).unwrap();

        // Dry run
        if matches.is_present(cmd::ARG_DRY) {
            self.set(cmd::ARG_DRY, Yaml::Boolean(true))?;
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
        let params = matches.values_of(cmd::ARG_PARAM);
        if params.is_none() {
            return Ok(());
        }

        // Get the current list of arguments or create a fresh one if non-existent
        let cfg_params = self.get_dict(cmd::ARG_PARAM, BTreeMap::new());
        if cfg_params.is_err() {
            return Err(cfg_params.unwrap_err());
        }
        let mut cfg_params = cfg_params.unwrap();

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
        self.set_dict(cmd::ARG_PARAM, cfg_params)
    }

    /// Parse filters from the command line.
    ///
    /// The configuration is modified directly with the parsed arguments,
    /// and nothing is returned on success.
    ///
    /// Any errors may be returned if parsing failed.
    fn parse_filters(&mut self, matches: &ArgMatches) -> Result<()> {
        // Return early if there are no arguments to parse
        let filters = matches.values_of(cmd::ARG_FILTER);
        if filters.is_none() {
            return Ok(());
        }

        // Get the current list of arguments or create a fresh one if non-existent
        let mut cfg_filters = self.get_list(cmd::ARG_FILTER, vec![]);

        // Put all filters in the list
        filters.unwrap().for_each(|filter| cfg_filters.push(
            Yaml::String(filter.to_string())
        ));

        // Set the properties in the configuration
        self.set_list(cmd::ARG_FILTER, cfg_filters)
    }
}
