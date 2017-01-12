
use std::error::Error as ErrorTrait;
use std::fmt::{Display,Result,Formatter};
use std::convert::From;
use std::io;
use std::num;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Error {
    line: Option<u64>,
    msg: String,
}

impl Error {
    pub fn new(line: u64, msg: String) -> Error {
       Self::_new(Some(line), msg)
    }

    #[inline]
    fn _new(line: Option<u64>, msg: String) -> Error {
        Error {
            line: line,
            msg: msg,
       }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
         self.msg.as_ref()
    }
}


impl From<io::Error> for Error{
    fn from(err: io::Error) -> Error{
        Error::_new(None, format!("{}", err))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error{
        Error::_new(None, format!("{}", err))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.line {
            Some(line_nr) => write!(f, "Line: {} error: {}", line_nr, self.msg),
            None => write!(f, "error: {}", self.msg),
        }
    }
}
