
use super::wire::Wire;

use std::fmt;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Pin {
    Left,
    Right,
}

impl Display for Pin {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            Pin::Left => write!(f, "0"),
            Pin::Right => write!(f, "1"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IOPin {
    id: ID,
    wires: Vec<Wire>,
}

impl IOPin {
    pub fn new(id: ID) -> IOPin {
        IOPin {
            id: id,
            wires: Vec::new(),
        }
    }

    #[inline]
    pub fn connect(&mut self, wire: Wire) {
        self.wires.push(wire);
    }
}

impl Display for IOPin {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self.id {
            ID::Input(id) => {
                try!(write!(f, "InWire#{}", id));
                if self.wires.len() > 0 {
                    try!(write!(f, " "));
                    for wire in &self.wires {
                        try!(write!(f, "{}", wire));
                    }
                }
            },
            ID::Output(id) => try!(write!(f, "-{}", id)),
            _ => (),
        }
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ID {
    Input(u64),
    Output(u64),
    Gate(u64),
}

impl ID {
    #[inline]
    pub fn u64(&self) -> u64 {
        match *self {
            ID::Input(id) => id,
            ID::Output(id) => id,
            ID::Gate(id) => id,
        }
    }

    #[inline]
    pub fn index(&self) -> usize {
        match *self {
            ID::Input(id) => (id - 1) as usize,
            ID::Output(id) => (id - 1) as usize,
            ID::Gate(id) => (id - 1) as usize,
        }
    }
}

impl Display for ID {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            ID::Input(id) => write!(f, "{}", id),
            ID::Output(id) => write!(f, "-{}", id),
            ID::Gate(id) => write!(f, "{}", id),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Gate {
    gate_type: GateType,
    id: ID,
    wires: Vec<Wire>,
}

impl Gate {
    #[inline]
    pub fn new(gate_type: GateType, id: ID) -> Gate {
        Gate {
            gate_type: gate_type,
            id: id,
            wires: Vec::new(),
        }
    }

    #[inline]
    pub fn gate_type(&self) -> GateType {
        self.gate_type
    }

    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline]
    pub fn wires(&self) -> &[Wire] {
        self.wires.as_slice()
    }

    #[inline]
    pub fn connect(&mut self, wire: Wire) {
        self.wires.push(wire);
    }

    #[inline]
    pub fn disconnect_all(&mut self) {
        self.wires.clear();
    }

    #[inline]
    pub fn copy(&self) -> Gate {
        let mut gate = Gate {
            gate_type: self.gate_type,
            id: self.id,
            wires: Vec::with_capacity(self.wires.len()),
        };
        gate.wires.extend_from_slice(self.wires());
        gate
    }
}

impl Display for Gate {
    fn fmt(&self, f: &mut Formatter) -> Result {
        try!(write!(f, "{} {}", self.id, self.gate_type));
        if self.wires.len() > 0 {
            try!(write!(f, " -> "));
            for wire in &self.wires {
                try!(write!(f, "{} ", wire))
            }
        }
        write!(f, "")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GateType {
    And,
    Xor,
    Or,
    Not,
}

impl GateType {
    #[inline]
    pub fn pins(&self) -> u8 {
        match *self {
            GateType::Not => 1,
            _ => 2,
        }
    }
}

impl Display for GateType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let repr = match *self {
            GateType::And => "AND",
            GateType::Xor => "XOR",
            GateType::Or => "OR",
            GateType::Not => "NOT",
        };
        write!(f, "{}", repr)
    }
}
