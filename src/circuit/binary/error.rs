use std::io;
use super::super::super::parser;
use std::convert::From;
use std::error::Error as ErrorTrait;
use std::fmt::{Display,Result,Formatter};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ExecError {
    msg: String,
}

impl ExecError {
    pub fn new(msg: String) -> ExecError {
       ExecError{
           msg: msg,
       }
    }
}

impl ErrorTrait for ExecError {
    fn description(&self) -> &str {
         self.msg.as_ref()
    }
}


impl From<io::Error> for ExecError{
    fn from(err: io::Error) -> ExecError{
        ExecError::new(format!("{}", err))
    }
}

impl From<parser::Error> for ExecError{
    fn from(err: parser::Error) -> ExecError{
        ExecError::new(format!("{}", err))
    }
}

impl Display for ExecError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "error on exec: {}", self.msg)
    }
}