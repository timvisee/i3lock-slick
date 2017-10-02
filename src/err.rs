use std;
use std::fmt::{Display, Formatter};

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
