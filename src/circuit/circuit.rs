
use super::super::parser::{Gate, IOPin, One};

use std::fmt;

pub struct Circuit {
    inputs: Vec<IOPin>,
    gates: Vec<Gate>,
    outputs: Vec<IOPin>,
    constants: Option<One>,
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.constants.is_some() {
            try!(writeln!(f, "{}", self.constants.as_ref().unwrap()));
        }
        for pin in &self.inputs {
            try!(writeln!(f, "{}", pin));
        }

        for gate in &self.gates {
            try!(writeln!(f, "{}", gate));
        }

        for pin in &self.outputs {
            try!(writeln!(f, "{}", pin));
        }
        write!(f, "")
    }
}

impl Circuit {
    pub fn new(inputs: Vec<IOPin>, gates: Vec<Gate>, consts: Option<One>) -> Circuit {
        Circuit {
            inputs: inputs,
            outputs: count_outputs(&gates),
            gates: gates,
            constants: consts,
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

fn count_outputs(gates: &Vec<Gate>) -> Vec<IOPin>{
    let mut max: u64 = 0;
    for gate in gates {
        for wire in gate.wires(){
            if wire.is_output(){
                let dst_id = wire.dst_gate().u64();
                if max < dst_id {
                    max = dst_id;
                }
            }
        }
    }
    let mut outputs = Vec::with_capacity(max as usize);
    let mut i: u64 = 0;
    while i < max {
        outputs.push(IOPin::new_output(i));
        i += 1;
    }
    outputs
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
        let ref mut gate = from[*node];
        for wire in gate.wires() {
            if !wire.is_output() {
                nodes[wire.dst_gate().index()] -= 1;
            }
        }
        to.push(gate.copy());
        gate.disconnect_all();
        nodes[*node] = -1;
    }
}

fn new_nodes(gates: &Vec<Gate>) -> Vec<i64> {
    let mut nodes: Vec<i64> = Vec::with_capacity(gates.len());
    unsafe { nodes.set_len(gates.len()) };

    for gate in gates {
        for wire in gate.wires() {
            if !wire.is_output() {
                (&mut nodes)[wire.dst_gate().index()] += 1;
            }
        }
    }
    nodes
}
