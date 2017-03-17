use std::fmt;
use std::convert;

/// ID represents an unique node / edge identifier of the circuit graph.
/// There a three general purpose node types
///
///   - INPUT
///   - OUTPUT
///   - GATE
///
/// and a special Const for the ONE constant of a circuit.
/// Notice that an `ID::Input(5)` is different from an `ID::Gate(5)`.
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ID {
    Input(u64),
    Output(u64),
    Gate(u64),
    Const,
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ID::Input(val) => write!(f, "+{}", val),
            ID::Output(val) => write!(f, "-{}", val),
            ID::Gate(val) => write!(f, "{}", val),
            _ => write!(f, "ONE"),
        }
    }
}

impl convert::Into<u64> for ID {
    fn into(self) -> u64 {
        match self {
            ID::Input(val) => val,
            ID::Output(val) => val,
            ID::Gate(val) => val,
            ID::Const => panic!("cannot convert: {}", ID::Const),
        }
    }
}

impl ID {
    /// Returns true if the ID is ID::Input.
    #[inline]
    pub fn is_input(&self) -> bool {
        match *self {
            ID::Input(_) => true,
            _ => false,
        }
    }

    /// Returns true if the ID is ID::Output.
    #[inline]
    pub fn is_output(&self) -> bool {
        match *self {
            ID::Output(_) => true,
            _ => false,
        }
    }

    /// Returns true if the ID is ID::Gate.
    #[inline]
    pub fn is_gate(&self) -> bool {
        match *self {
            ID::Gate(_) => true,
            _ => false,
        }
    }
}

/// GateType represents the four basic gate types:
///
///   - AND
///   - XOR
///   - OR
///   - NOT
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GateType {
    XOR,
    AND,
    OR,
    NOT,
}

impl GateType {
    pub fn operands(&self) -> usize {
        match *self {
            GateType::NOT => 1,
            _ => 2,
        }
    }
}

impl fmt::Display for GateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GateType::AND => write!(f, "A"),
            GateType::XOR => write!(f, "X"),
            GateType::OR => write!(f, "O"),
            GateType::NOT => write!(f, "N"),
        }
    }
}

/// Pin represents the two different pins of a binary Gate:
///
///   - Left (= 0)
///   - Right (= 1)
///
/// NOT gates use always the Left pin.
///
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Pin {
    Left,
    Right,
}

impl fmt::Display for Pin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pin::Left => write!(f, "0"),
            Pin::Right => write!(f, "1"),
        }
    }
}

/// An edge is a connection from one node to another node.
/// An edge is equivalent to a wire if you prefer to think in gates and wires.
/// An edge can also connect two nodes of different circuits.
///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Edge {
    id: ID,
    pin: Option<Pin>,
    circuit: Option<String>,
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.circuit {
            Some(ref val) => try!(write!(f, "{}:", val)),
            None => (),
        };
        match self.pin {
            Some(val) => write!(f, "{}:{}", self.id, val),
            _ => write!(f, "{}", self.id),
        }
    }
}

impl Edge {
    pub fn new(id: ID, pin: Option<Pin>, circuit: Option<String>) -> Edge {
        Edge {
            id: id,
            pin: pin,
            circuit: circuit,
        }
    }

    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline]
    pub fn pin(&self) -> Option<Pin> {
        self.pin
    }

    #[inline]
    pub fn circuit(&self) -> Option<String> {
        match self.circuit {
            Some(ref val) => Some(val.clone()),
            None => None,
        }
    }
}

/// A node represents an entity within a circuit. This can be a:
///
/// - "input pin / IOPin / IOGate"
/// - gate
/// - "output pin / IOPin / IOGate"
///
/// Nodes are connected through edges. A circuit is logically a graph.
///
pub struct Node {
    id: ID,
    gate_type: Option<GateType>,
    circuit: Option<String>,
    edges: Vec<Edge>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.circuit {
            Some(ref val) => try!(write!(f, "{}:", val)),
            None => (),
        };
        try!(match self.gate_type {
            Some(val) => write!(f, "{}:{}", val, self.id),
            _ => write!(f, "{}", self.id),
        });
        if self.edges.len() > 0 {
            let mut i = 0;
            try!(write!(f, "->"));
            for edge in &(self.edges) {
                try!(write!(f, "{}", edge));
                if i < self.edges().len() - 1 {
                    try!(write!(f, " "));
                    i += 1;
                }
            }
        }
        write!(f, "")
    }
}

impl Node {
    pub fn new(id: ID,
               gate_type: Option<GateType>,
               circuit: Option<String>,
               edges: Vec<Edge>)
               -> Node {
        Node {
            id: id,
            gate_type: gate_type,
            circuit: circuit,
            edges: edges,
        }
    }

    #[inline]
    pub fn id(&self) -> ID {
        self.id
    }

    #[inline]
    pub fn is_input(&self) -> bool {
        match self.id {
            ID::Input(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_output(&self) -> bool {
        match self.id {
            ID::Output(_) => true,
            _ => false,
        }
    }

    #[inline]
    pub fn is_constant(&self) -> bool {
        match self.id {
            ID::Const => true,
            _ => false,
        }
    }

    #[inline]
    pub fn circuit(&self) -> Option<String> {
        match self.circuit {
            Some(ref val) => Some(val.clone()),
            None => None,
        }
    }

    #[inline]
    pub fn gate_type(&self) -> Option<GateType> {
        self.gate_type
    }

    #[inline]
    pub fn edges(&self) -> &[Edge] {
        self.edges.as_slice()
    }

    #[inline]
    pub fn connect(&mut self, edges: &[Edge]) {
        self.edges.extend_from_slice(edges);
    }
}
