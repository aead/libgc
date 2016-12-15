
use std::fmt;
use std::fmt::Display;

use super::parser::Gate;

pub struct Circuit{
    input_gates: Vec<Gate>,
    gates: Vec<Gate>,
    output_gates: Vec<Gate>,
}

impl Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for gate in self.gates.as_slice() {
            try!(writeln!(f, "{}", gate));
        }
        write!(f,"")
    }
}

impl Circuit{
    pub fn new(input_gates: Vec<Gate>, gates: Vec<Gate>, output_gates: Vec<Gate>) -> Circuit{
        Circuit{
            input_gates: input_gates,
            gates: gates,
            output_gates: output_gates,
        }
    }

    pub fn sort(&mut self){

    }
}