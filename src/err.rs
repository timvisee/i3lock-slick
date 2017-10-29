extern crate config;
extern crate yaml_rust;

use std;
use std::convert::From;
use std::fmt::{Display, Formatter};
use std::num::{ParseFloatError, ParseIntError};
use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};

use app;

// Application result type
pub type Result<'a, T> = std::result::Result<T, Error<'a>>;

#[derive(Debug)]
pub struct Error<'a> {
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
        write!(f, "{} error: {}", app::NAME, self.description)
    }
}

impl<'a> From<self::config::ConfigError> for Error<'a> {
    fn from(err: self::config::ConfigError) -> Self {
        match err {
            self::config::ConfigError::Frozen => Error::new("The configuration is frozen"),
            self::config::ConfigError::NotFound(_) => Error::new("Unknown property"),
            self::config::ConfigError::PathParse(_) => Error::new("The configuration path could not be parsed"),
            self::config::ConfigError::FileParse { .. } => Error::new("The configuration could not be parsed from a file"),
            self::config::ConfigError::Type { .. } => Error::new("The property value could not be parsed into the requested file"),
            self::config::ConfigError::Message(_) => Error::new("An error occurred while using the configuration"),
            self::config::ConfigError::Foreign(_) => Error::new("An unadorned error from foreign origin occurred, causing problems with the configuration")
        }
    }
}

impl<'a> From<PoisonError<self::config::ConfigError>> for Error<'a> {
    fn from(_: PoisonError<self::config::ConfigError>) -> Self {
        Error::new("Configuration is poisoned")
    }
}

impl<'a> From<PoisonError<RwLockReadGuard<'a, config::Config>>> for Error<'a> {
    fn from(_: PoisonError<RwLockReadGuard<'a, config::Config>>) -> Self {
        Error::new("The read guard of this configuration is poisoned")
    }
}

impl<'a> From<PoisonError<RwLockWriteGuard<'a, config::Config>>> for Error<'a> {
    fn from(_: PoisonError<RwLockWriteGuard<'a, config::Config>>) -> Self {
        Error::new("The write guard of this configuratin is poisoned")
    }
}

impl<'a> From<yaml_rust::ScanError> for Error<'a> {
    fn from(_: yaml_rust::ScanError) -> Self {
//        use std::error::Error;
//        super::Error::new(err.description())
        Error::new("Failed to parse YAML configuration file")
    }
}

impl<'a> From<std::io::Error> for Error<'a> {
    fn from(_: std::io::Error) -> Self {
//        use std::error::Error;
//        super::Error::new(err.description())
        Error::new("IO operation failed")
    }
}

impl<'a> From<ParseIntError> for Error<'a> {
    fn from(_: ParseIntError) -> Self {
        //        use std::error::Error;
        //        super::Error::new(err.description())
        Error::new("Unable to parse float value")
    }
}

impl<'a> From<ParseFloatError> for Error<'a> {
    fn from(_: ParseFloatError) -> Self {
        //        use std::error::Error;
        //        super::Error::new(err.description())
        Error::new("Unable to parse float value")
    }
}
