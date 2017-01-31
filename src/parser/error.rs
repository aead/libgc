use std::error::Error as ErrorTrait;
use std::fmt::{Display,Result,Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ErrorType {
    UnknownGateType,
    InvalidInputID,
    InvalidGateID,
    InvalidOutputID,
    InvalidPin,
    InvalidInputPin,
    InvalidGate,
    InvalidWire,
    Unknown,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> Result{
        match *self {
            ErrorType::UnknownGateType => write!(f, "unknown gate type"),
            ErrorType::InvalidInputID => write!(f, "expected +'ID'"),
            ErrorType::InvalidGateID => write!(f, "expected 'ID'"),
            ErrorType::InvalidOutputID => write!(f, "expected -'ID'"),
            ErrorType::InvalidPin => write!(f, "expected '0' or '1'"),
            ErrorType::InvalidInputPin => write!(f, "expected pattern +'ID'->'WIRE'"),
            ErrorType::InvalidGate => write!(f, "expected 'GATE'->'WIRE' pattern"),
            ErrorType::InvalidWire => write!(f, "expected 'ID:'PIN' pattern"),
            ErrorType::Unknown => write!(f, "unknown parser error"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ParseError {
    err_type: ErrorType,
    msg: String
}

impl ParseError {
    pub fn new<T: Display>(err_type: ErrorType, expr: T) -> ParseError {
       ParseError{
           err_type: err_type,
           msg: format!("{} : {}", err_type, expr),
       }
    }
}

impl ErrorTrait for ParseError {
    fn description(&self) -> &str {
         self.msg.as_ref()
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "error: {}", self.msg)
    }
}