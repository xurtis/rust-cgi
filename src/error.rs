//! Errors for the CGI crate.

use std::{error, fmt};
use std::convert::From;
use url::ParseError;

/// Results involding errors related to cgi handling and testing.
pub type CgiResult<T> = Result<T, Error>;

/// Errors related handling and testing CGI.
#[derive(Debug)]
pub enum Error {
    /// There were multiple attempts made to load the request.
    MultipleLoad,
    /// Could not form a legitimate url.
    UrlParse(ParseError),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MultipleLoad => "Multiple attempts were made to load the request.",
            Error::UrlParse(_) => "Tried to make the URL invalid."
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::UrlParse(ref err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Error::UrlParse(ref err) => write!(f, "Tried to make url invalid: {}", err),
            _ => write!(f, "{}", error::Error::description(self))
        }
    }
}

impl From<ParseError> for Error {
    fn from(err: ParseError) -> Error {
        Error::UrlParse(err)
    }
}
