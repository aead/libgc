
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Error {
    line: u64,
    msg: String,
}

impl Error {
    pub fn new(line: u64, msg: String) -> Error {
        Error {
            line: line,
            msg: msg,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line: {} : {}", self.line, self.msg)
    }
}
