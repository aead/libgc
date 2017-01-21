
use std::slice;
use std::convert;

use super::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ID {
    Input(u64),
    Gate(u64),
    Output(u64),
}

impl convert::Into<u64> for ID {
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
    wires: Vec<Wire>
}

impl InputPin {
    pub fn parse(expr: &str) -> Result<InputPin,ParseError> {
        let tokens: Vec<&str> = expr.split("->").collect();
        if tokens.len() != 2 {
            return Err(ParseError::new(format!("{} is not a valid input expression", expr)));
        }

        let token = tokens[0].trim();
        if token.len() < 2 || !token.starts_with("+") {
            return Err(ParseError::new(format!("{} doesn't match +'ID'", token)));     
        }
        let id = try!(token[1..].parse::<u64>());

        let mut wires = Vec::new();
        for token in tokens[1].split_whitespace() {
            wires.push(try!(Wire::parse(token.trim())));
        }

        Ok(InputPin::new(id, wires))
    }
    
    pub fn new(id: u64, wires: Vec<Wire>) -> InputPin {
        InputPin{
            id: id,
            wires: wires,
        }
    }

    pub fn id(&self) -> ID{
        ID::Input(self.id)
    }
}

impl<'a> IntoIterator for &'a InputPin {
    type Item = &'a Wire;
    type IntoIter = slice::Iter<'a, Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.iter()
    }
}

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
            return Err(ParseError::new(format!("{} is not a valid gate expression", expr)));
        }

        let token = tokens[0].trim();
        if token.len() < 3 {
            return Err(ParseError::new(format!("{} doesn't match 'Type':''ID'", token)));     
        }
        let gtype = try!(match &token[0..2]{
            "A:" => Ok(GateType::AND),
            "X:" => Ok(GateType::XOR),
            "O:" => Ok(GateType::OR),
            "N:" => Ok(GateType::NOT),
            _ => Err(ParseError::new(format!("{} is a unknown gate type", &token[0..2])))
        });
        let id = try!(token[2..].parse::<u64>());

        let mut wires = Vec::new();
        for token in tokens[1].split_whitespace() {
            wires.push(try!(Wire::parse(token.trim())));
        }

        Ok(Gate::new(gtype, id, wires))
    }

    pub fn new(gate_type: GateType, id: u64, wires: Vec<Wire>) -> Gate {
        Gate{
            gate_type: gate_type,
            id: id,
            wires: wires,
        }
    }

    pub fn gate_type(&self) -> GateType {
        self.gate_type
    }

    pub fn id(&self) -> ID{
        ID::Gate(self.id)
    }
}

impl<'a> IntoIterator for &'a Gate {
    type Item = &'a Wire;
    type IntoIter = slice::Iter<'a, Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.iter()
    }
}

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
            return Err(ParseError::new(format!("'{}' is not a valid wire expression", expr)));
        }

        let id = ID::Gate(try!(tokens[0].trim().parse::<u64>()));
        let pin = match tokens[1].trim() {
            "0" => Some(Pin::Left),
            "1" => Some(Pin::Right),
            _ => return Err(ParseError::new(format!("'{}' is not a valid pin", tokens[1].trim()))),
        };
        Ok(Wire::new(id, pin))
    }

    pub fn new(dst: ID, pin: Option<Pin>) -> Wire {
        Wire{
            dst: dst,
            pin: pin,
        }
    }

    pub fn dst(&self) -> ID {
        self.dst
    }

    pub fn pin(&self) -> Option<Pin>{
        match self.pin {
            Some(pin) => Some(pin),
            _ => None,
        }
    }

    pub fn is_output(&self) -> bool {
        match self.dst {
            ID::Output(_) => true,
            _ => false,
        }
    }   
}