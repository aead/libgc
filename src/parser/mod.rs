
pub mod gate;
pub mod wire;

use super::circuit::Circuit;
use self::gate::{Gate, GateType};
use self::wire::Wire;

use std::io::{BufRead, BufReader, Error};
use std::io::ErrorKind::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::vec::Vec;
use std::string::String;

const NUM_OF_GATES: &'static str = "output.numberofgates.txt";
const NUM_OF_OUTUT_BITS: &'static str = "output.noob.txt";
const GATES: &'static str = "output.gate.txt";
const INPUT_GATES: &'static str = "output.inputs.txt";

#[derive(Debug)]
pub struct Parser<'a> {
    path: &'a Path,
}

impl<'a> Parser<'a> {
    pub fn new(path: &Path) -> Parser {
        Parser { path: path }
    }

    pub fn parse_number_of_gates(&self) -> Result<usize, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(NUM_OF_GATES));

        let mut buf = String::new();
        let mut reader = BufReader::new(try!(File::open(pathbuf.as_path())));
        try!(reader.read_line(&mut buf));
        match buf.trim().parse::<usize>() {
            Ok(val) => Ok(val),
            Err(why) => Err(Error::new(InvalidData, why)),
        }
    }

    pub fn parse_number_of_output_bits(&self) -> Result<usize, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(NUM_OF_OUTUT_BITS));

        let mut buf = String::new();
        let mut reader = BufReader::new(try!(File::open(pathbuf.as_path())));
        try!(reader.read_line(&mut buf));
        match buf.trim().parse::<usize>() {
            Ok(val) => Ok(val),
            Err(why) => Err(Error::new(InvalidData, why)),
        }
    }

    pub fn parse_gates(&mut self) -> Result<Vec<Gate>, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(GATES));
        let reader = BufReader::new(try!(File::open(pathbuf.as_path())));
        _parse_gates(reader.lines())
    }

    pub fn parse_input_gates(&mut self) -> Result<Vec<Gate>, Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(INPUT_GATES));
        let reader = BufReader::new(try!(File::open(pathbuf.as_path())));
        _parse_input_gates(reader.lines())
    }

    pub fn create_circuit(&mut self) -> Result<Circuit, Error> {
        let mut num_of_out = try!(self.parse_number_of_output_bits());
        let mut output_gates = Vec::new();
        let mut id = -1;
        while num_of_out > 0 {
            output_gates.push(Gate::new(1, GateType::Output, id));
            id -= 1;
            num_of_out -= 1;
        }

        Ok(Circuit::new(try!(self.parse_input_gates()),
                        try!(self.parse_gates()),
                        output_gates))
    }
}


fn _parse_input_gates<I>(from: I) -> Result<Vec<Gate>, Error>
    where I: Iterator<Item = Result<String, Error>>
{
    let mut gates = Vec::with_capacity(50); // avoid the first reallocs
    let mut line_nr: i64 = 0;
    for line in from {
        let line = try!(line);
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();

        if tokens.len() < 1 {
            return Err(Error::new(InvalidData, format!("line {}: no token found", line_nr)));
        }
        if !tokens[0].trim().starts_with("InWire:#") {
            return Err(Error::new(InvalidData,
                                  format!("line {}: expected: InWire# - found: {}",
                                          line_nr,
                                          tokens[0])));
        }
        // TODO: check whether a number comes after 'InWire:#'

        let mut gate = Gate::new(1, GateType::Input, line_nr);

        for token in tokens.iter().skip(1) {
            let w: Vec<&str> = token.trim().split(":").collect();
            if w.len() != 3 {
                return Err(Error::new(InvalidData,
                                      format!("line {}: expected: <pin>::<gate_id>::<pin> - \
                                               found: {}",
                                              line_nr,
                                              token)));
            }

            let src = match w[0].parse::<u8>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };
            let id = match w[1].parse::<i64>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val - 1, // the compiler ids starts at 1, but we start at 0
            };
            let dst = match w[2].parse::<u8>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };
            gate.connect(Wire::new(src, dst, id));
        }

        gates.push(gate);

        line_nr += 1;
    }

    Ok(gates)
}

fn _parse_gates<I>(from: I) -> Result<Vec<Gate>, Error>
    where I: Iterator<Item = Result<String, Error>>
{
    let mut gates = Vec::with_capacity(500); // avoid the first reallocs
    let mut line_nr: i64 = 0;
    for line in from {
        let line = try!(line);
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() < 3 {
            return Err(Error::new(InvalidData,
                                  format!("line {}: expected at least 3 tokens - found {} \
                                           tokens",
                                          line_nr,
                                          tokens.len())));
        }

        let gate_type = match tokens[0].trim() {
            "AND" => GateType::And,
            "XOR" => GateType::Xor,
            "OR" => GateType::Or,
            "NOT" => GateType::Not,
            _ => {
                return Err(Error::new(InvalidData,
                                      format!("line {}: unknown gate type {}", line_nr, tokens[0])))
            }
        };

        let num_pins = match tokens[1].trim().parse::<u8>() {
            Err(why) => return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why))),
            Ok(val) => val,
        };

        let mut gate = Gate::new(num_pins, gate_type, line_nr);

        for token in tokens.iter().skip(2) {
            let w: Vec<&str> = token.trim().split(":").collect();
            if w.len() != 3 {
                return Err(Error::new(InvalidData,
                                      format!("line {}: expected: <pin>::<gate_id>::<pin> - \
                                               found: {}",
                                              line_nr,
                                              token)));
            }

            let src = match w[0].trim().parse::<u8>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };
            let id = match w[1].trim().parse::<i64>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val - 1, // the compiler ids starts at 1, but we start at 0
            };
            let dst = match w[2].trim().parse::<u8>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };

            gate.connect(Wire::new(src, dst, id));
        }
        gates.push(gate);

        line_nr += 1;
    }

    return Ok(gates);
}
