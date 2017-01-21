mod gate;
mod wire;

pub use self::gate::{Gate, GateType};
pub use self::wire::Wire;

use std::slice;
use std::vec;
use std::convert::Into;
use std::iter::IntoIterator;
use super::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ID {
    Input(u64),
    Gate(u64),
    Output(u64),
}

impl Into<u64> for ID {
    fn into(self) -> u64 {
        match self {
            ID::Input(v) => v,
            ID::Gate(v) => v,
            ID::Output(v) => v,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Pin{
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InputPin{
    id: u64,
    wires: Vec<Wire>,
}

impl InputPin {
    pub fn parse(expr: &str) -> Result<InputPin,ParseError> {
        let tokens: Vec<&str> = expr.split("->").collect();
        if tokens.len() != 2 {
            return Err(ParseError::new(format!("expected pattern +'ID'->'WIRE' - found {}", expr)));
        }

        let token = tokens[0].trim();
        if token.len() < 2 || !token.starts_with("+") {
            return Err(ParseError::new(format!("expected pattern +'ID' - found {}", token)));     
        }
        let id = try!(token[1..].parse::<u64>());

        let mut wires = Vec::new();
        for token in tokens[1].split_whitespace() {
            wires.push(try!(Wire::parse(token.trim())));
        }

        Ok(InputPin::new(id, wires))
    }
    
    #[inline]
    fn new(id: u64, wires: Vec<Wire>) -> InputPin {
        InputPin{
            id: id,
            wires: wires,
        }
    }

    #[inline]
    pub fn id(&self) -> ID{
        ID::Input(self.id)
    }
}

impl IntoIterator for InputPin {
    type Item = Wire;
    type IntoIter = vec::IntoIter<Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.into_iter()
    } 
}

impl<'a> IntoIterator for &'a InputPin {
    type Item = &'a Wire;
    type IntoIter = slice::Iter<'a, Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.iter()
    }
}

impl<'a> IntoIterator for &'a mut InputPin {
    type Item = &'a mut Wire;
    type IntoIter = slice::IterMut<'a, Wire>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.wires.iter_mut()
    }
}