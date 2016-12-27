
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

    pub fn parse_gates(& self) -> Result<Vec<Gate>, Error> {
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

    pub fn parse_inputs(& self) -> Result<Vec<IOPin>, Error> {
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

    let mut gate = Gate::new(gate_type, line);
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

    let mut io_pin = IOPin::new_input(pin_id as u64);
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
