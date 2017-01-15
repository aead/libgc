
use std::vec;
use std::slice;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::convert::Into;
use std::iter::{Iterator, IntoIterator};

use super::error::Error;

macro_rules! must {
    ($exp:expr, $line:expr, $tok:expr, $msg:expr) => {
        try!(match $exp {
            Ok(val) => Ok(val),
            Err(_) => Err(Error::new($line, format!("{} {}", $tok, $msg))),
        })
    };
}

macro_rules! fail {
    ($line:expr, $tok:expr, $msg:expr) => {
        return Err(Error::new($line, format!("{} {}", $tok, $msg)));
    };
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ID {
    Input(u64),
    Output(u64),
    Gate(u64),
}

impl Into<u64> for ID {
    fn into(self) -> u64 {
        match self {
            ID::Input(id) => id,
            ID::Output(id) => id,
            ID::Gate(id) => id,
        }
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            ID::Input(id) => write!(f, "+{}", id),
            ID::Output(id) => write!(f, "-{}", id),
            ID::Gate(id) => write!(f, "{}", id),
        }
    }
}

impl ID {
    #[inline]
    pub fn as_index(self) -> usize{
        let id = match self {
            ID::Input(id) => id,
            ID::Output(id) => id,
            ID::Gate(id) => id,
        };
        (id-1) as usize
    }

    #[inline]
    pub fn is_input(self) -> bool{
        match self {
            ID::Input(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_output(self) -> bool{
        match self {
            ID::Output(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_gate(self) -> bool{
        match self {
            ID::Gate(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IOPin {
    id: ID,
    wires: Vec<Wire>,
}

impl IntoIterator for IOPin {
    type Item = Wire;
    type IntoIter = vec::IntoIter<Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.into_iter()
    } 
}

impl<'a> IntoIterator for &'a IOPin {
    type Item = &'a Wire;
    type IntoIter = slice::Iter<'a, Wire>;

    fn into_iter(self) -> Self::IntoIter {
        self.wires.iter()
    }
}

impl Display for IOPin {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (mut i, len) = (0, self.wires.len());
        if len == 0 {
            return Ok(());
        }

        try!(write!(f, "{}->", self.id));
        for wire in &self.wires {
            try!(write!(f, "{}", wire));
            if i < len-1 {
                try!(write!(f, " "));
                i += 1;
            }
        }
        Ok(())
    }
}

impl IOPin {
    pub fn new_input(id: u64) -> IOPin {
        IOPin {
            id: ID::Input(id),
            wires: Vec::new(),
        }
    }

    pub fn parse_input(expr: &str, line: u64) -> Result<IOPin, Error> {
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        if tokens.len() < 2 {
            fail!(line,
                  &expr,
                  "doesn't match 'InWire:#_':'[src_pin:dst_id:dst_pin]'");
        }

        let token = tokens[0].trim();
        let io_pin: Vec<&str> = token.split("#").collect();
        if io_pin.len() != 2 {
            fail!(line, token, "doesn't match 'InWire:#'number''");
        }

        if io_pin[0] != "InWire:" {
            fail!(line, io_pin[0], "doesn't match 'InWire'");
        }

        let pin1 = io_pin[1].trim();
        let pin_id = must!(pin1.parse::<i64>(), line, pin1, "is not a valid IO pin id");

        if pin_id < 0 {
            fail!(line, pin1, "is not a valid IO pin id");
        }

        let mut io_pin = IOPin::new_input(pin_id as u64);
        for wire_expr in tokens.iter().skip(1) {
            io_pin.connect(try!(Wire::parse(wire_expr, line)));
        }

        Ok(io_pin)
    }

    pub fn new_output(id: u64) -> IOPin {
        IOPin {
            id: ID::Output(id),
            wires: Vec::new(),
        }
    }

    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }    

    #[inline]
    pub fn connect(&mut self, wire: Wire) {
        self.wires.push(wire);
    }

    #[inline]
    pub fn connect_to_gate(&mut self, to: u64, pin: Pin) {
        self.wires.push(Wire::to_gate(to, pin));
    }

    #[inline]
    pub fn connect_to_output(&mut self, to: u64) {
        self.wires.push(Wire::to_output(to));
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Pin {
    Left,
    Right,
}

impl Into<u8> for Pin {
    fn into(self) -> u8 {
        match self {
            Pin::Left => 0,
            Pin::Right => 1,
       }
    }
}

impl Display for Pin {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
       match *self {
            Pin::Left => write!(f, "0"),
            Pin::Right =>  write!(f, "1"),
       }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Wire {
    to: ID,
    pin: Option<Pin>,
}

impl Display for Wire {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self.pin {
            Some(pin) => write!(f, "{}:{}", self.to, pin),
            None => write!(f, "{}",self.to),
        }
    }
}

impl Wire {
    #[inline]
    fn new(to: ID, pin: Option<Pin>) -> Wire {
        Wire { to: to, pin: pin }
    }

    pub fn parse(expr: &str, line: u64) -> Result<Wire, Error> {
        let tokens: Vec<&str> = expr.trim().split(":").collect();
        if tokens.len() != 3 {
            fail!(line, expr, "doesn't match 'src_pin':'dst_id':'dst_pin'");
        }

        let token = tokens[0].trim();
        let src_pin = must!(token.parse::<u8>(),
                            line,
                            token,
                            "is not a valid pin number");
        if src_pin != 0 {
            fail!(line, src_pin, "is an invalid src pin - expected '0'");
        }

        let token = tokens[1].trim();
        let dst_id = must!(token.parse::<i64>(), line, token, "is not a valid gate id");

        let token = tokens[2].trim();
        let dst_pin = must!(token.parse::<u8>(),
                            line,
                            token,
                            "is not a valid pin number");

        let dst_pin = match dst_pin {
            0 => Pin::Left, 
            1 => Pin::Right,
            _ => fail!(line, dst_pin, "is an invalid dst pin - expected '0' or '1'"),
        };

        if dst_id >= 0 {
            Ok(Self::to_gate(dst_id as u64, dst_pin))
        } else {
            Ok(Self::to_output((-1 * dst_id) as u64))
        }
    }

    pub fn to_gate(to: u64, pin: Pin) -> Wire {
        Self::new(ID::Gate(to), Some(pin))
    }

    pub fn to_output(to: u64) -> Wire {
        Self::new(ID::Output(to), None)
    }

    #[inline]
    pub fn destination(&self) -> ID {
        self.to
    }

    #[inline]
    pub fn set_destination(&mut self, id: ID) {
        self.to = id;
    }

    #[inline]
    pub fn destionation_pin(&self) -> Option<Pin> {
        self.pin
    }

    #[inline]
    pub fn is_output(&self) -> bool {
        match self.to {
            ID::Output(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GateType {
    AND,
    OR,
    XOR,
    NOT,
}

impl Display for GateType {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            GateType::AND => write!(f, "A"),
            GateType::OR => write!(f, "O"),
            GateType::XOR => write!(f, "X"),
            GateType::NOT => write!(f, "N"),
        }
    }
}

impl GateType {
    #[inline]
    pub fn pins(&self) -> u8 {
        match *self {
            GateType::NOT => 1,
            _ => 2,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Gate {
    gtype: GateType,
    id: ID,
    wires: Vec<Wire>,
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let (mut i, len) = (0, self.wires.len());
        if len == 0 {
            return Ok(());
        }
        
        try!(write!(f, "{}:{}->", self.gtype, self.id));
        for wire in &self.wires {
            try!(write!(f, "{}", wire));
            
            if i < len-1 {
                try!(write!(f, " "));
                i += 1;
            }
        }
        Ok(())
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

impl Gate {
    
    pub fn new(gate_type: GateType, id: u64) -> Gate {
        Gate {
            gtype: gate_type,
            id: ID::Gate(id),
            wires: Vec::new(),
        }
    }

    pub fn parse(expr: &str, line: u64) -> Result<Gate, Error> {
        let tokens: Vec<&str> = expr.split_whitespace().collect();
        if tokens.len() < 3 {
            fail!(line,
                  &expr,
                  "doesn't match 'gate_type':'pin_number':'[src_pin:dst_id:dst_pin]'");
        }

        let token = tokens[0].trim();
        let gate_type = match token {
            "AND" => GateType::AND,
            "XOR" => GateType::XOR,
            "OR" => GateType::OR,
            "NOT" => GateType::NOT,
            _ => fail!(line, token, "is an unknown gate type"),
        };

        let token = tokens[1].trim();
        let pin_num = must!(token.parse::<u8>(), line, token, "is not a number");

        if gate_type.pins() != pin_num {
            fail!(line,
                  token,
                  format!("doesn't match '{}' gate - expect: {}",
                          gate_type,
                          gate_type.pins()));
        }

        let mut gate = Gate::new(gate_type, line);
        for wire_expr in tokens.iter().skip(2) {
            gate.connect(try!(Wire::parse(wire_expr, line)));
        }
        Ok(gate)
    }

    #[inline]
    pub fn connect(&mut self, wire: Wire) {
        self.wires.push(wire);
    }

    #[inline]
    pub fn connect_to_gate(&mut self, dst: u64, pin: Pin) {
        self.wires.push(Wire::to_gate(dst, pin))
    }

     #[inline]
    pub fn connect_to_output(&mut self, dst: u64) {
        self.wires.push(Wire::to_output(dst))
    }

    #[inline]
    pub fn replace(&mut self, index: usize, with: Wire) {
        self.wires.push(with);
        self.wires.swap_remove(index);
    }

    #[inline]
    pub fn set_id(&mut self, id: u64) {
        self.id = ID::Gate(id);
    }

    #[inline]
    pub fn id(&self) -> ID {
        self.id.into()
    }

    #[inline]
    pub fn get_type(&self) -> GateType{
        self.gtype
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.wires.len()
    }

    pub fn clone(&self) -> Gate {
        let mut gate = Gate {
            gtype: self.gtype,
            id: self.id,
            wires: Vec::with_capacity(self.wires.len()),
        };
        gate.wires.extend_from_slice(&self.wires);
        gate
    }
}
