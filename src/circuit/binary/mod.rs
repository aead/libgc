mod error;

use std::path::PathBuf;
use std::collections::HashMap;
use self::error::ExecError;

use super::super::parser;
use super::super::parser::MetaInfo;
use super::super::parser::types::*;
use super::super::parser::types::ID::*;
use super::super::parser::types::GateType::*;

type Entry = (ID, Pin);

pub struct Circuit {
    info: MetaInfo,
    input: HashMap<ID, u8>,
    output: HashMap<ID, u8>,
    lookup: HashMap<Entry, u8>,
    sub: HashMap<String, Circuit>,
}

impl Circuit {
    pub fn new(info: MetaInfo) -> Result<Circuit, ExecError> {
        let mut c = Circuit {
            info: info,
            input: HashMap::new(),
            output: HashMap::new(),
            lookup: HashMap::new(),
            sub: HashMap::new(),
        };
        if !c.info.sub_circuits.is_empty() {
            for (key, val) in &(c.info.sub_circuits) {
                let path = PathBuf::from(val);
                let inf = try!(parser::parse_meta_info(path.as_path()));
                c.sub.insert(key.clone(), try!(Circuit::new(inf)));
            }
        }
        match c.info.one {
            Some(ref node) => {
                for edge in node.edges() {
                    match edge.id() {
                        Output(id) => {
                            c.output.insert(Output(id), 1);
                        }
                        Gate(id) => {
                            let pin = try!(Circuit::expect_some(edge.pin(),
                                                                format!("ONE: pin is required \
                                                                         for edge: {}",
                                                                        edge)));
                            c.lookup.insert((Gate(id), pin), 1);
                        }
                        Input(id) => {
                            let key = try!(Circuit::expect_some(edge.circuit(),
                                                                format!("invalid edge: \
                                                                         expected sub circuit \
                                                                         - {}",
                                                                        edge)));
                            let sub: &mut Circuit =
                                try!(Circuit::expect_some(c.sub.get_mut(&key),
                                                          format!("unknown sub circuit: {}", key)));
                            sub.set_input(id, 1);
                            if sub.is_executable() {
                                try!(sub.execute());
                            }
                        }
                        _ => {
                            return Err(ExecError::from(format!("invalid edge: {}", edge)));
                        }
                    }
                }
            }
            None => (),
        };
        Ok(c)
    }

    pub fn is_executable(&self) -> bool {
        self.info.inputs == self.input.len() as u64
    }

    pub fn set_input(&mut self, id: u64, val: u8) {
        if val == 0 {
            self.input.insert(ID::Input(id), 0);
        } else {
            self.input.insert(ID::Input(id), 1);
        }

    }

    pub fn get_output(&self, id: u64) -> Result<u8, ExecError> {
        match self.output.get(&Output(id)) {
            Some(val) => Ok(*val),
            None => Err(ExecError::from(format!("no output for id {}", id))),
        }
    }

    pub fn collect_output(&self) -> Vec<u8> {
        let mut output = Vec::with_capacity(self.output.len());
        let mut i = 1;
        while i < self.output.len() + 1 {
            let v = self.output.get(&Output(i as u64)).unwrap();
            output.push(*v);
            i += 1;
        }
        output
    }

    pub fn execute(&mut self) -> Result<(), ExecError> {
        if !self.is_executable() {
            return Err(ExecError::from("circuit is not executable"));
        }
        let path = PathBuf::from(self.info.path.as_path());
        for node in try!(parser::open_circuit(path.as_path())) {
            let mut node: Node = try!(node);

            try!(match node.id() {
                ID::Input(_) => self.process_input(&mut node),
                ID::Output(_) => self.process_output(&mut node),
                ID::Gate(_) => self.process_gate(&mut node),
                _ => Err(ExecError::from(format!("invalid node id: {}", node.id()))),
            });
        }
        Ok(())
    }

    fn check<T>(expr: bool, msg: T) -> Result<(), ExecError>
        where ExecError: From<T>
    {
        match expr {
            true => Ok(()),
            false => Err(ExecError::from(msg)),
        }
    }

    fn expect_some<O, T>(expr: Option<O>, msg: T) -> Result<O, ExecError>
        where ExecError: From<T>
    {
        match expr {
            Some(val) => Ok(val),
            None => Err(ExecError::from(msg)),
        }
    }

    fn process_input(&mut self, node: &mut Node) -> Result<(), ExecError> {
        let val = *try!(Circuit::expect_some(self.input.get(&node.id()),
                                             format!("missing input value {}", node.id())));
        for edge in node.edges() {
            match edge.circuit() {
                Some(key) => {
                    try!(Circuit::check(edge.id().is_input(),
                                        format!("invalid edge: expected input id - {}", edge)));
                    let sub: &mut Circuit =
                        try!(Circuit::expect_some(self.sub.get_mut(&key),
                                                  format!("unknown sub circuit: {}", key)));
                    sub.set_input(edge.id().into(), val);
                    if sub.is_executable() {
                        try!(sub.execute());
                    }
                }
                None => {
                    match edge.id() {
                        Output(id) => {
                            self.output.insert(Output(id), val);
                        }
                        Gate(id) => {
                            let pin = try!(Circuit::expect_some(edge.pin(),
                                                                format!("pin is required for \
                                                                         edge: {}",
                                                                        edge)));
                            self.lookup.insert((Gate(id), pin), val);
                        }
                        _ => {
                            return Err(ExecError::from(format!("invalid edge: {}", edge)));
                        }
                    }
                }
            };
        }
        Ok(())
    }

    fn process_output(&mut self, node: &mut Node) -> Result<(), ExecError> {
        match node.circuit() {
            Some(key) => {
                let val = try!(try!(Circuit::expect_some(self.sub.get(&key),
                                                         format!("unknown sub circuit: {}", key)))
                    .get_output(node.id().into()));
                for edge in node.edges() {
                    match edge.id() {
                        Output(id) => {
                            self.output.insert(Output(id), val);
                        }
                        Gate(id) => {
                            let pin = try!(Circuit::expect_some(edge.pin(),
                                                                format!("pin is required for \
                                                                         edge: {}",
                                                                        edge)));
                            self.lookup.insert((Gate(id), pin), val);
                        }
                        Input(id) => {
                            let key = try!(Circuit::expect_some(edge.circuit(),
                                                                format!("invalid edge: \
                                                                         expected sub circuit \
                                                                         - {}",
                                                                        edge)));
                            let sub: &mut Circuit =
                                try!(Circuit::expect_some(self.sub.get_mut(&key),
                                                          format!("unknown sub circuit: {}", key)));
                            sub.set_input(id, val);
                            if sub.is_executable() {
                                try!(sub.execute());
                            }
                        }
                        _ => {
                            return Err(ExecError::from(format!("invalid edge: {}", edge)));
                        }
                    }
                }
            }
            None => {
                let val = try!(self.get_output(node.id().into()));
                for edge in node.edges() {
                    match edge.id() {
                        Input(id) => {
                            let key = try!(Circuit::expect_some(edge.circuit(),
                                                                format!("invalid edge: \
                                                                         expected sub circuit \
                                                                         - {}",
                                                                        edge)));
                            let sub: &mut Circuit =
                                try!(Circuit::expect_some(self.sub.get_mut(&key),
                                                          format!("unknown sub circuit: {}", key)));
                            sub.set_input(id, val);
                            if sub.is_executable() {
                                try!(sub.execute());
                            }
                        }
                        _ => {
                            return Err(ExecError::from(format!("invalid edge: {}", edge)));
                        }
                    };
                }
            }
        }
        Ok(())
    }

    fn process_gate(&mut self, node: &mut Node) -> Result<(), ExecError> {
        try!(Circuit::check(node.circuit().is_none(),
                            format!("node with id: {} cannot reference sub circuit", node.id())));
        let gate_type = try!(Circuit::expect_some(node.gate_type(),
                                                  format!("node with id: {} must have a gate \
                                                           type",
                                                          node.id())));
        let val = match gate_type.operands() {
            1 => {
                let v0 = try!(Circuit::expect_some(self.lookup.remove(&(node.id(), Pin::Left)),
                                                   format!("cannot find value for node: {}",
                                                           node.id())));
                !v0 & 0x01  // NOT
            }
            2 => {
                let v0 = try!(Circuit::expect_some(self.lookup.remove(&(node.id(), Pin::Left)),
                                                   format!("cannot find value for left pin of \
                                                            node: {}",
                                                           node.id())));
                let v1 = try!(Circuit::expect_some(self.lookup.remove(&(node.id(), Pin::Right)),
                                                   format!("cannot find value for right pin of \
                                                            node: {}",
                                                           node.id())));
                match gate_type {
                    AND => v0 & v1,
                    XOR => v0 ^ v1,
                    OR => v0 | v1,
                    _ => {
                        panic!("impossible situation");
                    }
                }
            }
            _ => {
                panic!("impossible situation");
            }
        };
        for edge in node.edges() {
            match edge.id() {
                Output(id) => {
                    self.output.insert(Output(id), val);
                }
                Gate(id) => {
                    let pin = try!(Circuit::expect_some(edge.pin(),
                                                        format!("pin is required for edge: {}",
                                                                edge)));
                    self.lookup.insert((Gate(id), pin), val);
                }
                Input(id) => {
                    let key = try!(Circuit::expect_some(edge.circuit(),
                                                        format!("invalid edge: expected sub \
                                                                 circuit - {}",
                                                                edge)));
                    let sub: &mut Circuit =
                        try!(Circuit::expect_some(self.sub.get_mut(&key),
                                                  format!("unknown sub circuit: {}", key)));
                    sub.set_input(id, val);
                    if sub.is_executable() {
                        try!(sub.execute());
                    }
                }
                _ => {
                    return Err(ExecError::from(format!("invalid edge: {}", edge)));
                }
            }
        }
        Ok(())
    }
}
