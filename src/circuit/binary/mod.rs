mod error;

use std::path::{Path,PathBuf};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
use self::error::ExecError;
use super::super::parser::types::{ID, Pin, GateType, Wire, Gate, InputPin};

type Entry = (ID, Pin);

pub struct BinaryCircuit<'a> {
    path: &'a Path,
    input: HashMap<ID, u8>,
    lookup: HashMap<Entry, u8>,
    output: HashMap<ID, u8>
}

impl<'a> BinaryCircuit<'a> {

    pub fn new(path: &Path) -> BinaryCircuit{
        BinaryCircuit{
            path: path,
            input: HashMap::new(),
            lookup: HashMap::new(),
            output: HashMap::new(),
        }
    }

    pub fn execute(&mut self) -> Result<Vec<u8>,ExecError>{
        let mut buf = PathBuf::new();
        buf.push(self.path);
        buf.push(Path::new("circuit.txt"));
        
        let circuit = BufReader::new(try!(File::open(buf.as_path())));
        for line in circuit.lines(){
            let token = try!(line);
            if token.starts_with("+"){
                let input = try!(InputPin::parse(token.as_str()));
                for wire in &input {
                    try!(self.process_input(input.id(), wire));
                }
                self.input.remove(&(input.id()).into());
            }else{
                let gate = try!(Gate::parse(token.as_str()));
                try!(self.process_gate(&gate));
            }
        }
        self.output()
    }

    pub fn set_input_pin(&mut self, id: u64){
        self.input.insert(ID::Input(id), 1);
    }

    pub fn input(&mut self) -> Vec<u8>{
        let mut in_bits = Vec::with_capacity(self.input.len());
        let mut id = 1;
        while !self.input.is_empty() {
            match self.input.remove(&ID::Input(id)) {
                Some(val) => in_bits.push(val),
                _ => in_bits.push(0),
            };
            id += 1;
        }
        in_bits
    }

    pub fn output(&mut self) -> Result<Vec<u8>,ExecError> {
        let mut out_bits = Vec::with_capacity(self.output.len());
        let mut id = 1;
        while !self.output.is_empty() {
            try!(match self.output.remove(&ID::Output(id)) {
                Some(val) => Ok(out_bits.push(val)),
                _ => Err(ExecError::new(format!("no output bit for {}", ID::Output(id)))),
            });
            id += 1;
        }
        Ok(out_bits)
    }

    fn process_input(&mut self, id: ID, wire: &Wire) -> Result<(),ExecError>{
        let value = match self.input.get(&id){
            Some(val) => *val,
            None => 0,
        };
        try!(match wire.pin() {
            Some(pin) => Ok(self.lookup.insert((wire.dst(), pin), value)),
            _ => Err(ExecError::new(format!("no destination pin for {}->{}", id, wire))),
        });
        Ok(())
    }

    fn process_gate(&mut self, gate: &Gate) -> Result<(),ExecError> {
        let left = try!(match self.lookup.remove(&(gate.id(), Pin::Left)) {
            Some(val) => Ok(val),
            _ => Err(ExecError::new(format!("{}:{} not found", gate.id(), Pin::Left))),
        });
        let right = if gate.gate_type() == GateType::NOT {
            0
        } else {
            try!(match self.lookup.remove(&(gate.id(), Pin::Right)) {
                Some(val) => Ok(val),
                _ => Err(ExecError::new(format!("{}:{} not found", gate.id(), Pin::Right))),
            })
        };

        let result = match gate.gate_type() {
            GateType::AND => left & right,
            GateType::XOR => left ^ right,
            GateType::OR => left | right,
            GateType::NOT => (!left) & 0x01,
        };

        for wire in gate {
            if wire.is_output(){
                self.output.insert(wire.dst(), result);
            }else{
                try!(match wire.pin() {
                    Some(pin) => Ok(self.lookup.insert((wire.dst(), pin), result)),
                    _ => Err(ExecError::new(format!("no destination pin for {}->{}", gate.id(), wire))),
                });
            }
        }
        Ok(())
    }
}
