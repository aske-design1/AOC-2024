
use std::fmt;
use std::io;

use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    InvalidDayNumber(u8),
    Parse(std::num::ParseIntError),
    Path(io::Error),
    InvalidPath { path: PathBuf },
    NetworkError(String),
    NotEnoughArgs(usize),
    InvalidOperation(String)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            InvalidDayNumber(num) => write!(f, "Invalid Day number: {}", num),
            Parse(err) => write!(f, "Parse error: {}", err),
            Path(err) => write!(f, "Path error: {}", err),
            InvalidPath { path } => write!(f, "Invalid path: {:?}", path),
            NetworkError(err) => write!(f, "Networking error: {}", err),
            NotEnoughArgs(amt) => write!(f, "Argument error: Only {} argument was supplied", amt),
            InvalidOperation(err) => write!(f, "Argument error: Operation inputted is not recognized -> {}", err)
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;
        match self {
            Parse(err) => Some(err),
            Path(err) => Some(err),
            _ => None, // Custom and InvalidPath errors don't have a source
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::Parse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Path(err)
    }
}
