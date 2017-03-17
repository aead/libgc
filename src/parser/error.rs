use std::error::Error;
use std::io;
use std::fmt::{Display, Result, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorType {
    IOError,

    UnknownGateType,
    InvalidInputID,
    InvalidGateID,
    InvalidOutputID,
    InvalidPin,
    InvalidInputPin,
    InvalidGate,
    InvalidWire,
    InvalidMetaInfo,
    Unknown,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ErrorType::IOError => write!(f, "IO operation failed"),

            ErrorType::UnknownGateType => write!(f, "unknown gate type"),
            ErrorType::InvalidInputID => write!(f, "invalid input ID"),
            ErrorType::InvalidGateID => write!(f, "invalid gate ID"),
            ErrorType::InvalidOutputID => write!(f, "invalid output ID"),
            ErrorType::InvalidPin => write!(f, "invalid pin"),
            ErrorType::InvalidInputPin => write!(f, "invalid input pin"),
            ErrorType::InvalidGate => write!(f, "invalid gate"),
            ErrorType::InvalidWire => write!(f, "invalid wire"),
            ErrorType::InvalidMetaInfo => write!(f, "invalid meta info"),
            ErrorType::Unknown => write!(f, "unknown"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParseError {
    err_type: ErrorType,
    msg: String,
}

impl ParseError {
    pub fn new(err_type: ErrorType, msg: &str) -> ParseError {
        ParseError {
            err_type: err_type,
            msg: format!("{}", msg),
        }
    }
}

impl Error for ParseError {
    fn description(&self) -> &str {
        self.msg.as_ref()
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::new(ErrorType::IOError, err.description())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}: {}", self.err_type, self.msg)
    }
}
