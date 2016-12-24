// use super::gate::{Gate,GateType};
// use super::wire::Wire;
// use super::circuit::Circuit;
//
// use std::io;
// use std::io::ErrorKind;
// use std::io::{BufReader, BufRead};
// use std::path::{Path, PathBuf};
// use std::fs::File;
// use std::vec::Vec;
// use std::string::String;
// use std::fmt;
//
// const NUM_OF_GATES: &'static str = "output.numberofgates.txt";
// const NUM_OF_OUTUT_BITS: &'static str = "output.noob.txt";
// const GATES: &'static str = "output.gate.txt";
// const INPUT_GATES: &'static str = "output.inputs.txt";
//
// #[derive(Debug)]
// pub struct Parser<'a> {
// path: &'a Path,
// }
//
// impl<'a> Parser<'a> {
// pub fn new(path: &Path) -> Parser {
// Parser { path: path }
// }
//
// pub fn parse_number_of_gates(&self) -> Result<usize, io::Error> {
// let mut pathbuf = PathBuf::new();
// pathbuf.push(self.path);
// pathbuf.push(Path::new(NUM_OF_GATES));
//
// let mut buf = String::new();
// let mut reader = io::BufReader::new(try!(File::open(pathbuf.as_path())));
// try!(reader.read_line(&mut buf));
// match buf.trim().parse::<usize>() {
// Ok(val) => Ok(val),
// Err(why) => Err(io::Error::new(ErrorKind::InvalidData, why)),
// }
// }
//
// pub fn parse_number_of_output_bits(&self) -> Result<usize, io::Error> {
// let mut pathbuf = PathBuf::new();
// pathbuf.push(self.path);
// pathbuf.push(Path::new(NUM_OF_OUTUT_BITS));
//
// let mut buf = String::new();
// let mut reader = BufReader::new(try!(File::open(pathbuf.as_path())));
// try!(reader.read_line(&mut buf));
// match buf.trim().parse::<usize>() {
// Ok(val) => Ok(val),
// Err(why) => Err(io::Error::new(ErrorKind::InvalidData, why)),
// }
// }
//
// pub fn parse_gates(&mut self) -> Result<Vec<Gate>, io::Error> {
// let mut pathbuf = PathBuf::new();
// pathbuf.push(self.path);
// pathbuf.push(Path::new(GATES));
// let reader = BufReader::new(try!(File::open(pathbuf.as_path())));
// _parse_gates(reader.lines())
// }
//
// pub fn parse_input_gates(&mut self) -> Result<Vec<Gate>, io::Error> {
// let mut pathbuf = PathBuf::new();
// pathbuf.push(self.path);
// pathbuf.push(Path::new(INPUT_GATES));
// let reader = BufReader::new(try!(File::open(pathbuf.as_path())));
// _parse_input_gates(reader.lines())
// }
//
// pub fn create_circuit(&mut self) -> Result<Circuit, io::Error> {
// let mut num_of_out = try!(self.parse_number_of_output_bits());
// let mut output_gates = Vec::new();
// let mut id = -1;
// while num_of_out > 0 {
// output_gates.push(Gate::new(1, GateType::Output, id));
// id -= 1;
// num_of_out -= 1;
// }
//
// Ok(Circuit::new(try!(self.parse_input_gates()),
// try!(self.parse_gates()),
// output_gates))
// }
// }
//
//
// fn _parse_input_gates<I>(from: I) -> Result<Vec<Gate>, io::Error>
// where I: Iterator<Item = Result<String, io::Error>>
// {
// let mut gates = Vec::with_capacity(50); // avoid the first reallocs
// let mut line_nr: i64 = 0;
// for line in from {
// let line = try!(line);
// let tokens: Vec<&str> = line.trim().split_whitespace().collect();
//
// try!(check_if(tokens.len() >= 1, line_nr, format!("no token found")));
// try!(check_if(tokens[0].trim().starts_with("InWire:#"),
// line_nr,
// format!("expected: InWire# - found: {}", tokens[0])));
// TODO: check whether a number comes after 'InWire:#'
//
// let mut gate = Gate::new(1, GateType::Input, line_nr);
//
// for token in tokens.iter().skip(1) {
// let w: Vec<&str> = token.trim().split(":").collect();
// try!(check_if(w.len() == 3,
// line_nr,
// format!("expected: <pin>::<gate_id>::<pin> - found: {}", token)));
//
// let src = try!(check_error(w[0].trim().parse::<u8>(), line_nr));
// let id = try!(check_error(w[1].trim().parse::<i64>(), line_nr)) - 1; // the compiler ids starts at 1, but we start at 0
// let dst = try!(check_error(w[2].trim().parse::<u8>(), line_nr));
//
// gate.connect(Wire::new(src, dst, id));
// }
//
// gates.push(gate);
//
// line_nr += 1;
// }
//
// Ok(gates)
// }
//
// fn _parse_gates<I>(from: I) -> Result<Vec<Gate>, io::Error>
// where I: Iterator<Item = Result<String, io::Error>>
// {
// let mut gates = Vec::with_capacity(500); // avoid the first reallocs
// let mut line_nr: i64 = 0;
// for line in from {
// let line = try!(line);
// let tokens: Vec<&str> = line.split_whitespace().collect();
//
// try!(check_if(tokens.len() >= 3,
// line_nr,
// format!("expected at least 3 tokens - found: {} tokens",
// tokens.len())));
//
// let gate_type = match tokens[0].trim() {
// "AND" => GateType::And,
// "XOR" => GateType::Xor,
// "OR" => GateType::Or,
// "NOT" => GateType::Not,
// _ => {
// return Err(io::Error::new(ErrorKind::InvalidData,
// format!("line {}: unknown gate type {}",
// line_nr,
// tokens[0])))
// }
// };
//
//
//
// let num_pins = try!(check_error(tokens[1].trim().parse::<u8>(), line_nr));
//
// let mut gate = Gate::new(num_pins, gate_type, line_nr);
// for token in tokens.iter().skip(2) {
// let w: Vec<&str> = token.trim().split(":").collect();
// try!(check_if(w.len() == 3,
// line_nr,
// format!("expected: <pin>::<gate_id>::<pin> - found: {}", token)));
//
// let src = try!(check_error(w[0].trim().parse::<u8>(), line_nr));
// let id = try!(check_error(w[1].trim().parse::<i64>(), line_nr)) - 1; // the compiler ids starts at 1, but we start at 0
// let dst = try!(check_error(w[2].trim().parse::<u8>(), line_nr));
//
// gate.connect(Wire::new(src, dst, id));
// }
// gates.push(gate);
//
// line_nr += 1;
// }
//
// return Ok(gates);
// }
//
// fn check_if(cond: bool, line_nr: i64, err_msg: String) -> Result<bool, io::Error> {
// match cond {
// false => {
// Err(io::Error::new(ErrorKind::InvalidData,
// format!("line {}: {}", line_nr, err_msg)))
// }
// _ => Ok(cond),
// }
// }
//
// fn check_error<T, E: fmt::Display>(result: Result<T, E>, line_nr: i64) -> Result<T, io::Error> {
// match result {
// Err(why) => {
// Err(io::Error::new(ErrorKind::InvalidData, format!("line {}: {}", line_nr, why)))
// }
// Ok(val) => Ok(val),
// }
// }

use super::error::Error;
use super::wire::Wire;
use super::gate::{Gate, GateType, ID, Pin, IOPin};

use std::fmt;
use std::io;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::fs::File;

const NUM_OF_GATES: &'static str = "output.numberofgates.txt";
const NUM_OF_OUTUT_BITS: &'static str = "output.noob.txt";
const GATES: &'static str = "output.gate.txt";
const INPUTS: &'static str = "output.inputs.txt";

#[derive(Debug)]
pub struct Parser<'a> {
    path: &'a Path,
}

impl<'a> Parser<'a> {
    pub fn new(path: &Path) -> Parser {
        Parser { path: path }
    }

    pub fn parse_gates(&self) -> Result<Vec<Gate>, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(GATES));
        let reader = BufReader::new(try!(match File::open(pathbuf.as_path()) {
            Ok(val) => Ok(val),
            Err(why) => Err(Error::new(0, format!("error: {}", why))),
        }));

        let mut gates = Vec::with_capacity(500); // avoid the first reallocs
        let mut line_nr: u64 = 0;
        for line in reader.lines() {
            let line = try!(match line {
                Ok(val) => Ok(val),
                Err(why) => Err(Error::new(line_nr, format!("error: {}", why))),
            });

            gates.push(try!(parse_gate(line_nr, line)));
            line_nr += 1
        }
        Ok(gates)
    }

    pub fn parse_inputs(&self) -> Result<Vec<IOPin>, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(INPUTS));
        let reader = BufReader::new(try!(match File::open(pathbuf.as_path()) {
            Ok(val) => Ok(val),
            Err(why) => Err(Error::new(0, format!("error: {}", why))),
        }));

        let mut pins = Vec::with_capacity(50); // avoid the first reallocs
        let mut line_nr: u64 = 0;
        for line in reader.lines() {
            let line = try!(match line {
                Ok(val) => Ok(val),
                Err(why) => Err(Error::new(line_nr, format!("error: {}", why))),
            });

            pins.push(try!(parse_input(line_nr, line)));
            line_nr += 1
        }
        Ok(pins)
    }
}

#[inline]
fn check<T, E: fmt::Display>(expr: Result<T, E>, line: u64, msg: String) -> Result<T, Error> {
    match expr {
        Ok(val) => Ok(val),
        Err(_) => Err(Error::new(line, msg)),
    }
}

fn parse_wire<F1: Fn(i64) -> Result<ID, Error>>(line: u64,
                                                expr: &str,
                                                to_id: F1)
                                                -> Result<Wire, Error> {
    let tokens: Vec<&str> = expr.trim().split(":").collect();
    if tokens.len() != 3 {
        return Err(Error::new(line,
                              format!("error: '{}' doesn't match 'src_pin':'dst_id':'dst_pin'",
                                      expr)));
    }

    let src_pin = try!(check(tokens[0].trim().parse::<u8>(),
                             line,
                             format!("error: '{}' is not a valid pin number", tokens[0])));
    let dst_id = try!(check(tokens[1].trim().parse::<i64>(),
                            line,
                            format!("error: '{}' is not a valid gate id", tokens[1])));
    let dst_pin = try!(check(tokens[2].trim().parse::<u8>(),
                             line,
                             format!("error: '{}' is not a valid pin number", tokens[2])));

    let src_pin = try!(match src_pin {
        0 => Ok(Pin::Left),
        1 => Ok(Pin::Right),
        _ => {
            Err(Error::new(line,
                           format!("error: invalid src pin '{}' - expect '0' or '1'", src_pin)))
        }
    });
    let dst_pin = try!(match dst_pin {
        0 => Ok(Pin::Left), 
        1 => Ok(Pin::Right),
        _ => {
            Err(Error::new(line,
                           format!("error: invalid dst pin '{}' - expect '0' or '1'", dst_pin)))
        }
    });

    Ok(Wire::new(src_pin, dst_pin, try!(to_id(dst_id))))
}

fn parse_gate(line: u64, expr: String) -> Result<Gate, Error> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.len() < 3 {
        return Err(Error::new(line,
                              format!("error: '{}' doesn't match \
                                       'gate_type':'pin_number':'[src_pin:dst_id:dst_pin]'",
                                      expr)));
    }

    let gate_type = try!(match tokens[0].trim() {
        "AND" => Ok(GateType::And),
        "XOR" => Ok(GateType::Xor),
        "OR" => Ok(GateType::Or),
        "NOT" => Ok(GateType::Not),
        _ => {
            Err(Error::new(line,
                           format!("error: Unknown gate type: '{}'", tokens[0].trim())))
        }
    });

    let pin_num = try!(check(tokens[1].trim().parse::<u8>(),
                             line,
                             format!("error: Type mismatch: '{}' is not a number",
                                     tokens[1].trim())));

    if gate_type.pins() != pin_num {
        return Err(Error::new(line,
                              format!("error: {0} doesn't match '{1}' gate - expect: {1} {2}",
                                      tokens[1].trim(),
                                      gate_type,
                                      gate_type.pins())));
    }

    let to_id = |x| if x >= 0 {
        Ok(ID::Gate(x as u64))
    } else {
        Ok(ID::Output((-1 * x) as u64))
    };

    let mut gate = Gate::new(gate_type, ID::Gate(line));
    for wire_expr in tokens.iter().skip(2) {
        gate.connect(try!(parse_wire(line, wire_expr, &to_id)));
    }
    Ok(gate)
}

fn parse_input(line: u64, expr: String) -> Result<IOPin, Error> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.len() < 2 {
        return Err(Error::new(line,
                              format!("error: '{}' doesn't match \
                                       'InWire:#_':'[src_pin:dst_id:dst_pin]'",
                                      expr)));
    }

    let io_pin: Vec<&str> = tokens[0].split("#").collect();
    if io_pin.len() != 2 {
        return Err(Error::new(line,
                              format!("error: '{}' doesn't match \
                                       'InWire:#'number''",
                                      tokens[0])));
    }

    if io_pin[0] != "InWire:" {
        return Err(Error::new(line,
                              format!("error: '{}' doesn't match \
                                       'InWire'",
                                      io_pin[0])));

    }

    let pin_id = try!(check(io_pin[1].trim().parse::<i64>(),
                            line,
                            format!("error: '{}' is not a valid IO pin id", io_pin[1])));

    if pin_id < 0 {
        return Err(Error::new(line,
                              format!("error: '{}' is not a valid IO pin id", io_pin[1])));
    }

    let to_id = |x| if x >= 0 {
        Ok(ID::Input(x as u64))
    } else {
        Err(Error::new(line, format!("error: '{}' is not a valid 'dst_id'", x)))
    };

    let mut io_pin = IOPin::new(ID::Input(pin_id as u64));
    for wire_expr in tokens.iter().skip(1) {
        let wire = try!(parse_wire(line, wire_expr, &to_id));
        if wire.src_pin() == Pin::Right {
            return Err(Error::new(line,
                                  format!("error: invalid src pin '{}' - expect '0'",
                                          wire.src_pin())));
        }
        io_pin.connect(wire);
    }

    Ok(io_pin)
}
