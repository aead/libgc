
use std::error::Error as ErrorTrait;
use std::fmt::{Display, Formatter, Result as FmtResult};

use super::types::Gate;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Error {
}

impl Error {
    fn new() -> Error{
        Error{}  
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
       write!(f, "error: {}", self.description())
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
         "cannot sort gates topologically - ordering is ambiguous"
    }
}

pub fn sort_gates(gates: &mut Vec<Gate>) -> Result<Vec<Gate>,Error>{
    let mut nodes = new_nodes(gates);
    let mut sorted: Vec<Gate> = Vec::with_capacity(gates.len());
    let mut marked = Vec::new();
    loop {
        if mark_nodes(&mut marked, &mut nodes) {
            break;
        }

        if marked.is_empty() {
            return Err(Error::new());
        }

        sweep_nodes(&mut marked, &mut nodes, gates, &mut sorted);
    }
    //Ok(normalize(sorted))
    Ok(sorted)
}

fn mark_nodes(marked: &mut Vec<usize>, nodes: &mut Vec<i64>) -> bool {
    let (mut n, mut cnt, mut done) = (0 as usize, 0 as usize, true);
    marked.clear();
    for node in nodes {
        match *node {
            0 => marked.push(n),
            -1 => cnt += 1,
            _ => done = false,
        };
        n += 1;
    }
    if cnt == n {
        done = true
    }
    done
}

fn sweep_nodes(marked: &mut Vec<usize>,
               nodes: &mut Vec<i64>,
               from: &mut Vec<Gate>,
               to: &mut Vec<Gate>) {
    for node in marked {
        let ref gate = from[*node];
        for wire in gate {
            if !wire.is_output() {
                nodes[wire.destination().as_index()] -= 1;
            }
        }
        to.push(gate.clone());
        nodes[*node] = -1;
    }
}

fn new_nodes(gates: &Vec<Gate>) -> Vec<i64> {
    let mut nodes: Vec<i64> = Vec::with_capacity(gates.len());
    unsafe { nodes.set_len(gates.len()) };

    for gate in gates {
        for wire in gate {
            if !wire.is_output() {
                (&mut nodes)[wire.destination().as_index()] += 1;
            }
        }
    }
    nodes
}

/*
fn normalize(gates: Vec<Gate>) -> Vec<Gate>{
    let mut norm: Vec<Gate> = Vec::with_capacity(gates.len());
    let mut mapping = HashMap::new();

    let mut i: u64 = 1;
    for gate in &gates {
        let (old_id, new_id) = (gate.id(), i);

        let mut n_gate = gate.clone();
        n_gate.set_id(new_id);
        norm.push(n_gate);

        let mut j = 1;
        for tmp_gate in &mut norm {
            for mut wire in tmp_gate {
                let dst: ID = wire.destination();
                if dst.is_gate() {
                    let dst: u64 = dst.into();
                    if dst == old_id {
                        let hit = match mapping.entry(j) {
                            Entry::Occupied(val) => *(val.get()) == old_id,
                            Entry::Vacant(_) => false,
                        };
                        if hit {
                            wire.set_destination(ID::Gate(new_id));
                        }
                    }
                }
            }
            j += 1;
            mapping.insert(j, new_id);
        }
        i += 1;
    }
    norm
}
*/