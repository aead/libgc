
pub mod error;
pub mod types;

use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead, Lines};
use std::fs::File;
use std::default::Default;
use std::collections::{HashSet, HashMap};

use self::error::{ParseError, ErrorType};
use self::error::ErrorType::*;
use self::types::*;
use self::types::ID::*;

struct Context {
    line: u64,
}

impl Context {
    pub fn new() -> Context {
        Context { line: 1 }
    }

    pub fn next_line(&mut self) {
        self.line += 1;
    }

    pub fn fail(&self, err_type: ErrorType, msg: &str) -> ParseError {
        ParseError::new(err_type, format!("Line: {} - {}", self.line, msg).as_str())
    }
}

pub struct MetaInfo {
    pub path: PathBuf,
    pub inputs: u64,
    pub outputs: u64,
    pub gates: u64,
    pub one: Option<Node>,
    pub sub_circuits: HashMap<String, String>,
}

impl Default for MetaInfo {
    fn default() -> MetaInfo {
        MetaInfo {
            path: PathBuf::new(),
            inputs: 0,
            outputs: 0,
            gates: 0,
            one: None,
            sub_circuits: HashMap::new(),
        }
    }
}

pub struct Circuit<B> {
    ctx: Context,
    lines: Lines<B>,
}

impl<B: BufRead> Iterator for Circuit<B> {
    type Item = Result<Node, ParseError>;

    fn next(&mut self) -> Option<Result<Node, ParseError>> {
        let result = match self.lines.next() {
            None => None,
            Some(val) => {
                match val {
                    Ok(val) => Some(parse_node(val, &mut self.ctx)),
                    Err(why) => Some(Err(ParseError::from(why))),
                }
            }
        };
        self.ctx.next_line();
        result
    }
}

pub fn open_circuit(path: &Path) -> Result<Circuit<BufReader<File>>, ParseError> {
    let p = PathBuf::from(path).join("circuit.txt");
    let reader = BufReader::new(try!(File::open(p.as_path())));
    Ok(Circuit {
        ctx: Context::new(),
        lines: reader.lines(),
    })
}

pub fn parse_circuit(path: &Path) -> Result<Vec<Node>, ParseError> {
    let mut nodes = Vec::new();
    for node in try!(open_circuit(path)) {
        nodes.push(try!(node));
    }
    Ok(nodes)
}

pub fn parse_meta_info(path: &Path) -> Result<MetaInfo, ParseError> {
    let p = PathBuf::from(path).join("meta_info.txt");
    let reader = BufReader::new(try!(File::open(p.as_path())));

    let mut ctx = Context { line: 1 };
    let mut info = MetaInfo::default();
    info.path = PathBuf::from(path);
    let mut keys: HashSet<String> = HashSet::new();
    for line in reader.lines() {
        let line = try!(line);
        let tokens: Vec<&str> = line.split("=").collect();
        if tokens.len() != 2 {
            return Err(ctx.fail(Unknown, ""));
        }

        let token = tokens[0].trim();
        if keys.contains(&token.to_string()) {
            return Err(ctx.fail(Unknown, ""));
        }
        keys.insert(token.to_string());
        match token {
            "INPUTS" => {
                info.inputs = try!(match tokens[1].trim().parse::<u64>() {
                    Ok(val) => Ok(val),
                    Err(_) => Err(ctx.fail(Unknown, "")),
                })
            }
            "OUTPUTS" => {
                info.outputs = try!(match tokens[1].trim().parse::<u64>() {
                    Ok(val) => Ok(val),
                    Err(_) => Err(ctx.fail(Unknown, "")),
                })
            }
            "GATES" => {
                info.gates = try!(match tokens[1].trim().parse::<u64>() {
                    Ok(val) => Ok(val),
                    Err(_) => Err(ctx.fail(Unknown, "")),
                })
            }
            "ONE" => {
                let edges = try!(parse_edges(String::from(tokens[1].trim()), &mut ctx));
                info.one = Some(Node::new(Const, None, None, edges));
            }
            _ => {
                if token == "A" || token == "X" || token == "O" || token == "N" {
                    return Err(ctx.fail(Unknown, ""));
                }
                info.sub_circuits.insert(token.to_string(), tokens[1].trim().to_string());
            }
        }
        ctx.next_line();
    }
    Ok(info)
}

fn parse_node(line: String, ctx: &mut Context) -> Result<Node, ParseError> {
    let tokens: Vec<&str> = line.split("->").collect();
    if tokens.len() != 2 {
        return Err(ctx.fail(Unknown, ""));
    }

    let node = tokens[0].trim();
    let edges = try!(parse_edges(tokens[1].trim().to_string(), ctx));
    if node.starts_with("+") {
        let id = try!(match (&node[1..]).parse::<u64>() {
            Ok(val) => Ok(val),
            Err(_) => Err(ctx.fail(InvalidInputID, "input ID is not a number")),
        });
        return Ok(Node::new(Input(id), None, None, edges));
    }
    if node.starts_with("-") {
        let id = try!(match (&node[1..]).parse::<u64>() {
            Ok(val) => Ok(val),
            Err(_) => Err(ctx.fail(InvalidOutputID, "output ID is not a number")),
        });
        return Ok(Node::new(Output(id), None, None, edges));
    }

    let tokens: Vec<&str> = node.split(":").collect();
    if tokens.len() != 2 {
        return Err(ctx.fail(Unknown, ""));
    }

    let token = tokens[0].trim();
    if token == "A" || token == "X" || token == "O" || token == "N" {
        let gate_type = match token {
            "A" => GateType::AND,
            "X" => GateType::XOR,
            "O" => GateType::OR,
            "N" => GateType::NOT,
            _ => panic!("impossible situation"),
        };
        let id = try!(match tokens[1].trim().parse::<u64>() {
            Ok(val) => Ok(val),
            Err(_) => Err(ctx.fail(InvalidGateID, "gate ID is not a number")),
        });
        return Ok(Node::new(Gate(id), Some(gate_type), None, edges));
    }

    let node = tokens[1].trim();
    let id = try!(match (&node[1..]).parse::<u64>() {
        Ok(val) => Ok(val),
        Err(_) => Err(ctx.fail(InvalidOutputID, "output ID is not a number")),
    });
    return Ok(Node::new(Output(id), None, Some(token.to_string()), edges));
}

fn parse_edges(line: String, ctx: &mut Context) -> Result<Vec<Edge>, ParseError> {
    let tokens: Vec<&str> = line.split(" ").collect();
    let mut edges = Vec::with_capacity(tokens.len());
    for token in tokens {
        edges.push(try!(parse_edge(token, ctx)));
    }
    Ok(edges)
}

fn parse_edge(line: &str, ctx: &mut Context) -> Result<Edge, ParseError> {
    let tokens: Vec<&str> = line.split(":").collect();
    if tokens.len() == 1 {
        let node = tokens[0].trim();
        if !node.starts_with("-") {
            return Err(ctx.fail(Unknown, ""));
        }
        let id = try!(match (&node[1..]).parse::<u64>() {
            Ok(val) => Ok(val),
            Err(_) => Err(ctx.fail(InvalidOutputID, "output ID is not a number")),
        });
        return Ok(Edge::new(Output(id), None, None));
    }
    if tokens.len() != 2 {
        return Err(ctx.fail(Unknown, ""));
    }
    let token = tokens[0].trim();
    let c = try!(match token.chars().next() {
        Some(val) => Ok(val),
        None => Err(ctx.fail(Unknown, "")),
    });
    if c == '0' || c == '1' || c == '2' || c == '3' || c == '4' || c == '5' || c == '6' ||
       c == '7' || c == '8' || c == '9' {
        let pin = try!(match tokens[1].trim() {
            "0" => Ok(Some(Pin::Left)),
            "1" => Ok(Some(Pin::Right)),
            _ => Err(ctx.fail(InvalidPin, "pin is not 0 nor 1")),
        });
        let id = try!(match token.parse::<u64>() {
            Ok(val) => Ok(val),
            Err(_) => Err(ctx.fail(InvalidGateID, "gate ID is not a number")),
        });
        return Ok(Edge::new(Gate(id), pin, None));
    }
    let sub_circuit = Some(token.to_string());
    let token = tokens[1].trim();
    if !token.starts_with("+") {
        return Err(ctx.fail(InvalidInputID, ""));
    }
    let id = try!(match (&token[1..]).parse::<u64>() {
        Ok(val) => Ok(val),
        Err(_) => Err(ctx.fail(InvalidInputID, "input ID is not a number")),
    });
    return Ok(Edge::new(Input(id), None, sub_circuit));
}
