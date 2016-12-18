
use std::fmt;
use std::fmt::Display;

use std::vec::Vec;
use std::ops::Index;

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
        let mut nodes: Vec<i64> = Vec::with_capacity(self.gates.len());
        let cap = nodes.capacity();
        unsafe { nodes.set_len(cap)};  // That's ok because the cap is set correctly
        
        for gate in &(self.gates){
            for wire in gate.wires(){
                if !wire.is_output() {
                    (&mut nodes)[wire.dst_gate() as usize] += 1;
                }
            }
        }

        let mut top_gates: Vec<Gate> = Vec::with_capacity(self.gates.len());
        loop {
            let ( mut done, mut n, mut marked) = (true, 0, Vec::new());

            // mark
            while n < nodes.len() {
                let node = n as usize;
                match nodes[node] {
                    0 => marked.push(node),
                    -1 => (),
                    _ => done = false,
                };
                n += 1;
            }

            if marked.is_empty() {
                panic!("Cannot sort circuit"); // TODO: Error handling
            }

            // sweep
            for node in marked{
                let ref gate = self.gates[node];
                for wire in gate.wires(){
                    if !wire.is_output() {
                        nodes[wire.dst_gate() as usize] -= 1;
                    }
                }
                top_gates.push(gate.copy());
                nodes[node] = -1;
            }

            if done {
                break;
            }
        }

        self.gates = top_gates;
    }
}