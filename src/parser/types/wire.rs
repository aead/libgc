use super::{ID, Pin};
use super::super::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Wire {
    dst: ID,
    pin: Option<Pin>
}

impl Wire {
    pub fn parse(expr: &str) -> Result<Wire,ParseError>{
        if expr.starts_with("-") {
            let id = try!(expr[1..].parse::<u64>());
            return Ok(Wire::new(ID::Output(id), None))
        }

        let tokens: Vec<&str> = expr.split(":").collect();
        if tokens.len() != 2 {
            return Err(ParseError::new(format!("expected 'ID:'PIN' pattern - found {}", expr)));
        }

        let token = tokens[0].trim();
        let id = ID::Gate(try!(token.parse::<u64>()));
        let pin = match tokens[1].trim() {
            "0" => Some(Pin::Left),
            "1" => Some(Pin::Right),
            _ => return Err(ParseError::new(format!("expected '0' or '1' - found {}", token))),
        };
        Ok(Wire::new(id, pin))
    }

    fn new(dst: ID, pin: Option<Pin>) -> Wire {
        Wire{
            dst: dst,
            pin: pin,
        }
    }

    #[inline]
    pub fn dst(&self) -> ID {
        self.dst
    }

    #[inline]
    pub fn pin(&self) -> Option<Pin>{
        match self.pin {
            Some(pin) => Some(pin),
            _ => None,
        }
    }

    #[inline]
    pub fn is_output(&self) -> bool {
        match self.dst {
            ID::Output(_) => true,
            _ => false,
        }
    }   
}