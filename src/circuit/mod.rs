
use std::fmt;
use std::fmt::Display;

use std::vec::Vec;
use std::ops::Index;

use super::parser::Gate;

pub struct Circuit {
    input_gates: Vec<Gate>,
    gates: Vec<Gate>,
    output_gates: Vec<Gate>,
}

impl Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for gate in self.gates.as_slice() {
            try!(writeln!(f, "{}", gate));
        }
        write!(f, "")
    }
}

impl Circuit {
    pub fn new(input_gates: Vec<Gate>, gates: Vec<Gate>, output_gates: Vec<Gate>) -> Circuit {
        Circuit {
            input_gates: input_gates,
            gates: gates,
            output_gates: output_gates,
        }
    }

    pub fn sort(&mut self) {
        let mut nodes = new_nodes(&(self.gates));
        let mut top_gates: Vec<Gate> = Vec::with_capacity(self.gates.len());
        let mut marked = Vec::new();
        loop {
            let done = mark_nodes(&mut marked, &mut nodes);

            if marked.is_empty() {
                panic!("Cannot sort circuit"); // TODO: Error handling
            }

            sweep_nodes(&mut marked, &mut nodes, &mut self.gates, &mut top_gates);

            if done {
                break;
            }
        }

        self.gates = top_gates;
    }
}

fn mark_nodes(marked: &mut Vec<usize>, nodes: &mut Vec<i64>) -> bool {
    let (mut n, mut done) = (0 as usize, true);
    marked.clear();
    for node in nodes {
        match *node {
            0 => marked.push(n),
            -1 => (),
            _ => done = false,
        };
        n += 1;
    }
    done
}

fn sweep_nodes(marked: &mut Vec<usize>,
               nodes: &mut Vec<i64>,
               from: &mut Vec<Gate>,
               to: &mut Vec<Gate>) {
    for node in marked {
        let ref gate = from[*node];
        for wire in gate.wires() {
            if !wire.is_output() {
                nodes[wire.dst_gate() as usize] -= 1;
            }
        }
        to.push(gate.copy());
        nodes[*node] = -1;
    }
}

fn new_nodes(gates: &Vec<Gate>) -> Vec<i64> {
    let mut nodes: Vec<i64> = Vec::with_capacity(gates.len());
    unsafe { nodes.set_len(gates.len()) };

    for gate in gates {
        for wire in gate.wires() {
            if !wire.is_output() {
                (&mut nodes)[wire.dst_gate() as usize] += 1;
            }
        }
    }
    nodes
}
