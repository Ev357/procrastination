use std::num::ParseIntError;

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Path(std::path::StripPrefixError),
    InvalidMethod,
    InvalidPath,
    InvalidVersion,
    ProtocolError,
    NotFound(String),
    InternalError,
    Arg(ArgError),
    ParseError(String),
    ParseIntError(ParseIntError),
}

impl From<ParseIntError> for Error {
    fn from(error: ParseIntError) -> Self {
        Error::ParseIntError(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::ParseError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IO(error)
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(error: std::path::StripPrefixError) -> Self {
        Error::Path(error)
    }
}

impl From<std::convert::Infallible> for Error {
    fn from(_: std::convert::Infallible) -> Self {
        Error::InternalError
    }
}

#[derive(Debug)]
pub enum ArgError {
    InvalidPath(String),
    InvalidPort(String),
    InvalidArgument(String),
    MissingArgument(String),
}
