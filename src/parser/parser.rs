
use super::error::Error;
use super::wire::Wire;
use super::gate::{Gate, GateType, ID, Pin, IOPin};

use std::fmt;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::fs::File;

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
        let reader = BufReader::new(try!(File::open(pathbuf.as_path())));

        let mut gates = Vec::with_capacity(500); // avoid the first reallocs
        let mut line_nr: u64 = 0;
        for line in reader.lines() {
            gates.push(try!(parse_gate(line_nr, try!(line))));
            line_nr += 1
        }
        Ok(gates)
    }

    pub fn parse_inputs(&self) -> Result<Vec<IOPin>, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(INPUTS));
        let reader = BufReader::new(try!(File::open(pathbuf.as_path())));

        let mut pins = Vec::with_capacity(50); // avoid the first reallocs
        let mut line_nr: u64 = 0;
        for line in reader.lines() {
            pins.push(try!(parse_input(line_nr, try!(line))));
            line_nr += 1
        }
        Ok(pins)
    }
}

#[inline]
fn parse_err<R, T: fmt::Display>(line: u64, found: T, expected: &'static str) -> Result<R, Error> {
    Err(Error::new(line, format!("{} {}", found, expected)))
}

#[inline]
fn check<T, E, F>(expr: Result<T, E>,
                  line: u64,
                  found: F,
                  expected: &'static str)
                  -> Result<T, Error>
    where E: fmt::Display,
          F: fmt::Display
{
    match expr {
        Ok(val) => Ok(val),
        Err(_) => parse_err(line, found, expected),
    }
}

fn parse_wire<F>(line: u64, expr: &str, to_id: F) -> Result<Wire, Error>
    where F: Fn(i64) -> Result<ID, Error>
{
    let tokens: Vec<&str> = expr.trim().split(":").collect();
    if tokens.len() != 3 {
        return parse_err(line, expr, "doesn't match 'src_pin':'dst_id':'dst_pin'");
    }

    let src_pin = try!(check(tokens[0].trim().parse::<u8>(),
                             line,
                             tokens[0].trim(),
                             "is not a valid pin number"));

    let dst_id = try!(check(tokens[1].trim().parse::<i64>(),
                            line,
                            tokens[1].trim(),
                            "is not a valid gate id"));

    let dst_pin = try!(check(tokens[2].trim().parse::<u8>(),
                             line,
                             tokens[2].trim(),
                             "is not a valid pin number"));

    let src_pin = try!(match src_pin {
        0 => Ok(Pin::Left),
        1 => Ok(Pin::Right),
        _ => parse_err(line, src_pin, "is an invalid src pin - expected '0' or '1'"),
    });
    let dst_pin = try!(match dst_pin {
        0 => Ok(Pin::Left), 
        1 => Ok(Pin::Right),
        _ => parse_err(line, dst_pin, "is an invalid dst pin - expected '0' or '1'"),
    });

    Ok(Wire::new(src_pin, dst_pin, try!(to_id(dst_id))))
}

fn parse_gate(line: u64, expr: String) -> Result<Gate, Error> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.len() < 3 {
        return parse_err(line,
                         &expr,
                         "doesn't match 'gate_type':'pin_number':'[src_pin:dst_id:dst_pin]'");
    }

    let gate_type = try!(match tokens[0].trim() {
        "AND" => Ok(GateType::And),
        "XOR" => Ok(GateType::Xor),
        "OR" => Ok(GateType::Or),
        "NOT" => Ok(GateType::Not),
        _ => parse_err(line, tokens[0].trim(), "is an unknown gate type"),
    });

    let pin_num = try!(check(tokens[1].trim().parse::<u8>(),
                             line,
                             tokens[1].trim(),
                             "is not a number"));

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

    let mut gate = Gate::new(gate_type, line);
    for wire_expr in tokens.iter().skip(2) {
        gate.connect(try!(parse_wire(line, wire_expr, &to_id)));
    }
    Ok(gate)
}

fn parse_input(line: u64, expr: String) -> Result<IOPin, Error> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.len() < 2 {
        return parse_err(line,
                         &expr,
                         "doesn't match 'InWire:#_':'[src_pin:dst_id:dst_pin]'");
    }

    let io_pin: Vec<&str> = tokens[0].split("#").collect();
    if io_pin.len() != 2 {
        return parse_err(line, tokens[0], "doesn't match 'InWire:#'number''");
    }

    if io_pin[0] != "InWire:" {
        return parse_err(line, io_pin[0], "doesn't match 'InWire'");

    }

    let pin_id = try!(check(io_pin[1].trim().parse::<i64>(),
                            line,
                            io_pin[1].trim(),
                            "is not a valid IO pin id"));

    if pin_id < 0 {
        return parse_err(line, io_pin[1], "is not a valid IO pin id");
    }

    let to_id = |x| if x >= 0 {
        Ok(ID::Input(x as u64))
    } else {
        parse_err(line, x, "is not a valid 'dst_id'")
    };

    let mut io_pin = IOPin::new_input(pin_id as u64);
    for wire_expr in tokens.iter().skip(1) {
        let wire = try!(parse_wire(line, wire_expr, &to_id));
        if wire.src_pin() == Pin::Right {
            return parse_err(line, wire.src_pin(), "invalid src pin '{}' - expect '0'");
        }
        io_pin.connect(wire);
    }

    Ok(io_pin)
}
