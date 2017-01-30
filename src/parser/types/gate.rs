use std::slice;
use std::vec;
use std::iter::IntoIterator;
use super::{ID, Wire};
use super::super::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GateType{
    AND,
    XOR,
    OR,
    NOT,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Gate {
    gate_type: GateType,
    id: u64,
    wires: Vec<Wire>,
}

impl Gate{
    pub fn parse(expr: &str) -> Result<Gate,ParseError> {
        let tokens: Vec<&str> = expr.split("->").collect();
        if tokens.len() != 2 {
            return Err(ParseError::new(format!("expected 'GATE'->'WIRE' pattern - found {}", expr)));
        }

        let token = tokens[0].trim();
        if token.len() < 3 {
            return Err(ParseError::new(format!("expected 'TYPE':'ID' pattern - found {}", token)));     
        }

        let gtype = try!(match &token[0..2]{
            "A:" => Ok(GateType::AND),
            "X:" => Ok(GateType::XOR),
            "O:" => Ok(GateType::OR),
            "N:" => Ok(GateType::NOT),
            _ => Err(ParseError::new(format!("unknown gate type: {}", &token[0..2])))
        });
        let id = try!(token[2..].parse::<u64>());

        let mut wires = Vec::new();
        for token in tokens[1].split_whitespace() {
            wires.push(try!(Wire::parse(token.trim())));
        }

        Ok(Gate::new(gtype, id, wires))
    }

    #[inline]
    fn new(gate_type: GateType, id: u64, wires: Vec<Wire>) -> Gate {
        Gate{
            gate_type: gate_type,
            id: id,
            wires: wires,
        }
    }

    #[inline]
    pub fn gate_type(&self) -> GateType {
        self.gate_type
    }

    #[inline]
    pub fn id(&self) -> ID {
        ID::Gate(self.id)
    }
}

impl IntoIterator for Gate {
    type Item = Wire;
    type IntoIter = vec::IntoIter<Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.into_iter()
    } 
}

impl<'a> IntoIterator for &'a Gate {
    type Item = &'a Wire;
    type IntoIter = slice::Iter<'a, Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.iter()
    }
}

impl<'a> IntoIterator for &'a mut Gate {
    type Item = &'a mut Wire;
    type IntoIter = slice::IterMut<'a, Wire>;

    fn into_iter(mut self) -> Self::IntoIter {
        self.wires.iter_mut()
    }
}