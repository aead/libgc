
use std::path::{Path,PathBuf};
use std::io::{BufRead, BufReader, Error};
use std::fs::File;
use std::collections::HashMap;

use super::super::parser::{ID, Pin, GateType, Wire, Gate, InputPin, ParseError};

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

    pub fn execute(&mut self) -> Result<Vec<u8>,ParseError>{
        let mut buf = PathBuf::new();
        buf.push(self.path);
        buf.push(Path::new("circuit.txt"));
        
        let circuit = BufReader::new(try!(File::open(buf.as_path())));
        for line in circuit.lines(){
            let token = try!(line);
            if token.starts_with("+"){
                let input = try!(InputPin::parse(token.as_str()));
                for wire in &input {
                    self.process_input(input.id(), wire);
                }
                self.input.remove(&(input.id()).into());
            }else{
                let gate = try!(Gate::parse(token.as_str()));
                self.process_gate(&gate);
            }
        }
        Ok(self.output())
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
                _ => panic!("there must be always a value"),
            };
            id += 1;
        }
        in_bits
    }

    pub fn output(&mut self) -> Vec<u8> {
        let mut out_bits = Vec::with_capacity(self.output.len());
        let mut id = 1;
        while !self.output.is_empty() {
            match self.output.remove(&ID::Output(id)) {
                Some(val) => out_bits.push(val),
                _ => panic!("there must be always a value"),
            };
            id += 1;
        }
        out_bits
    }

    fn process_input(&mut self, id: ID, wire: &Wire) {
        let value = match self.input.get(&id){
            Some(val) => *val,
            None => 0,
        };
        match wire.pin() {
            Some(pin) => self.lookup.insert((wire.dst(), pin), value),
            _ => panic!("there must be always a value"),
        };
    }

    fn process_gate(&mut self, gate: &Gate) {
        let left = match self.lookup.remove(&(gate.id(), Pin::Left)) {
            Some(val) => val,
            _ => panic!("there must be always a value"),
        };
        let right = if gate.gate_type() == GateType::NOT {
            0
        } else {
            match self.lookup.remove(&(gate.id(), Pin::Right)) {
                Some(val) => val,
                _ => panic!("there must be always a value"),
            }
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
                match wire.pin() {
                    Some(pin) => self.lookup.insert((wire.dst(), pin), result),
                    _ => panic!("there must be always a value"),
                };
            }
        }
    }
}
