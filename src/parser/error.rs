use std::error::Error as ErrorTrait;
use std::fmt::{Display,Result,Formatter};
use std::convert::From;
use std::io;
use std::num;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParseError {
    msg: String,
}

impl ParseError {
    pub fn new(msg: String) -> ParseError {
       ParseError{
           msg: msg,
       }
    }
}

impl ErrorTrait for ParseError {
    fn description(&self) -> &str {
         self.msg.as_ref()
    }
}


impl From<io::Error> for ParseError{
    fn from(err: io::Error) -> ParseError{
        ParseError::new(format!("{}", err))
    }
}

impl From<num::ParseIntError> for ParseError {
    fn from(err: num::ParseIntError) -> ParseError{
        ParseError::new(format!("{}", err))
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "error: {}", self.msg)
    }
}