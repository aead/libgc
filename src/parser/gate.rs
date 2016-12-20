
use std::fmt;
use super::wire;

#[derive(Debug,Clone,Eq,PartialEq,Ord,PartialOrd)]
pub struct Gate {
    pins: u8,
    gate_type: GateType,
    id: i64,
    wires: Vec<wire::Wire>,
}


impl Gate {
    pub fn new(n_pins: u8, gate_type: GateType, gate_id: i64) -> Gate {
        Gate {
            pins: n_pins,
            gate_type: gate_type,
            id: gate_id,
            wires: Vec::new(),
        }
    }

    pub fn pins(&self) -> u8 {
        self.pins
    }

    pub fn gate_type(&self) -> GateType {
        self.gate_type
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn wires(&self) -> &[wire::Wire] {
        self.wires.as_slice()
    }

    pub fn connect(&mut self, wire: wire::Wire) {
        self.wires.push(wire);
    }

    pub fn disconnect_all(&mut self) {
        self.wires.clear();
    }

    pub fn copy(&self) -> Gate {
        let mut gate = Gate {
            pins: self.pins,
            gate_type: self.gate_type,
            id: self.id,
            wires: Vec::with_capacity(self.wires.len()),
        };
        gate.wires.extend_from_slice(self.wires());
        gate
    }
}

impl fmt::Display for Gate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{} {} {}", self.id, self.gate_type, self.pins));
        if self.wires.len() > 0 {
            try!(write!(f, " -> "));
            for wire in &self.wires {
                try!(write!(f, "{} ", wire))
            }
        }
        write!(f, "")
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq,Ord,PartialOrd)]
pub enum GateType {
    Input,
    Output,
    And,
    Xor,
    Or,
    Not,
}

impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{}",
               match *self {
                   GateType::Input => "Input",
                   GateType::Output => "Output",
                   GateType::And => "AND",
                   GateType::Xor => "XOR",
                   GateType::Or => "OR",
                   GateType::Not => "NOT",
               })
    }
}
