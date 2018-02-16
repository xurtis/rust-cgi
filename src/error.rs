//! Errors for the CGI crate.

use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    /// There were multiple attempts made to load the request.
    MultipleLoad,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MultipleLoad => "Multiple attempts were made to load the request.",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::MultipleLoad => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::MultipleLoad => write!(f, "Multiple attempts were made to liad the request."),
        }
    }
}
