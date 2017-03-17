use std::io;
use super::super::super::parser::error::ParseError;
use std::convert::From;
use std::error;
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExecError {
    msg: String,
}

impl<'a> From<&'a str> for ExecError {
    fn from(s: &'a str) -> ExecError {
        ExecError { msg: String::from(s) }
    }
}

impl From<String> for ExecError {
    fn from(s: String) -> ExecError {
        ExecError { msg: s }
    }
}

impl error::Error for ExecError {
    fn description(&self) -> &str {
        self.msg.as_ref()
    }
}


impl From<io::Error> for ExecError {
    fn from(err: io::Error) -> ExecError {
        ExecError::from(format!("{}", err))
    }
}

impl From<ParseError> for ExecError {
    fn from(err: ParseError) -> ExecError {
        ExecError::from(format!("{}", err))
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error on exec: {}", self.msg)
    }
}
