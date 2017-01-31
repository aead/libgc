use std::fmt::{Display, Formatter, Result as FmtResult};

use super::{ID, Pin};
use super::super::error::ParseError;
use super::super::error::ErrorType::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Wire {
    dst: ID,
    pin: Option<Pin>
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter) -> FmtResult{
        match self.pin {
            Some(pin) => write!(f, "{}:{}", self.dst, pin), 
            _ => write!(f, "{}", self.dst),
        } 
    }
}

impl Wire {
    pub fn parse(expr: &str) -> Result<Wire,ParseError>{
        if expr.starts_with("-") {
            let id = try!(map_err!(expr[1..].parse::<u64>(), ParseError::new(InvalidOutputID, expr)));
            return Ok(Wire::new(ID::Output(id), None))
        }

        let tokens: Vec<&str> = expr.split(":").collect();
        if tokens.len() != 2 {
            fail!(InvalidWire, expr);
        }

        let token = tokens[0].trim();
        let id = try!(map_err!(token.parse::<u64>(), ParseError::new(InvalidGateID, token)));
        let pin = match tokens[1].trim() {
            "0" => Some(Pin::Left),
            "1" => Some(Pin::Right),
            _ => fail!(InvalidPin, token),
        };
        Ok(Wire::new(ID::Gate(id), pin))
    }

    fn new(dst: ID, pin: Option<Pin>) -> Wire {
        Wire{
            dst: dst,
            pin: pin,
        }
    }

    #[inline]
    pub fn dst(&self) -> ID { self.dst }

    #[inline]
    pub fn pin(&self) -> Option<Pin>{ self.pin }

    #[inline]
    pub fn is_output(&self) -> bool {
        match self.dst {
            ID::Output(_) => true,
            _ => false,
        }
    }   
}