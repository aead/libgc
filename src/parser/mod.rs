
pub mod gate;
pub mod wire;

use super::circuit;
use self::gate::GateType;

use std::io;
use std::io::ErrorKind;
use std::io::{BufReader, BufRead};
use std::path::{Path, PathBuf};
use std::fs::File;
use std::vec::Vec;
use std::string::String;
use std::fmt;

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

    pub fn parse_number_of_gates(&self) -> Result<usize, io::Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(NUM_OF_GATES));

        let mut buf = String::new();
        let mut reader = io::BufReader::new(try!(File::open(pathbuf.as_path())));
        try!(reader.read_line(&mut buf));
        match buf.trim().parse::<usize>() {
            Ok(val) => Ok(val),
            Err(why) => Err(io::Error::new(ErrorKind::InvalidData, why)),
        }
    }

    pub fn parse_number_of_output_bits(&self) -> Result<usize, io::Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(NUM_OF_OUTUT_BITS));

        let mut buf = String::new();
        let mut reader = BufReader::new(try!(File::open(pathbuf.as_path())));
        try!(reader.read_line(&mut buf));
        match buf.trim().parse::<usize>() {
            Ok(val) => Ok(val),
            Err(why) => Err(io::Error::new(ErrorKind::InvalidData, why)),
        }
    }

    pub fn parse_gates(&mut self) -> Result<Vec<gate::Gate>, io::Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(GATES));
        let reader = BufReader::new(try!(File::open(pathbuf.as_path())));
        _parse_gates(reader.lines())
    }

    pub fn parse_input_gates(&mut self) -> Result<Vec<gate::Gate>, io::Error> {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(INPUT_GATES));
        let reader = BufReader::new(try!(File::open(pathbuf.as_path())));
        _parse_input_gates(reader.lines())
    }

    pub fn create_circuit(&mut self) -> Result<circuit::Circuit, io::Error> {
        let mut num_of_out = try!(self.parse_number_of_output_bits());
        let mut output_gates = Vec::new();
        let mut id = -1;
        while num_of_out > 0 {
            output_gates.push(gate::Gate::new(1, GateType::Output, id));
            id -= 1;
            num_of_out -= 1;
        }

        Ok(circuit::Circuit::new(try!(self.parse_input_gates()),
                                 try!(self.parse_gates()),
                                 output_gates))
    }
}


fn _parse_input_gates<I>(from: I) -> Result<Vec<gate::Gate>, io::Error>
    where I: Iterator<Item = Result<String, io::Error>>
{
    let mut gates = Vec::with_capacity(50); // avoid the first reallocs
    let mut line_nr: i64 = 0;
    for line in from {
        let line = try!(line);
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();

        try!(check_if(tokens.len() >= 1, line_nr, format!("no token found")));
        try!(check_if(tokens[0].trim().starts_with("InWire:#"),
                      line_nr,
                      format!("expected: InWire# - found: {}", tokens[0])));
        // TODO: check whether a number comes after 'InWire:#'

        let mut gate = gate::Gate::new(1, GateType::Input, line_nr);

        for token in tokens.iter().skip(1) {
            let w: Vec<&str> = token.trim().split(":").collect();
            try!(check_if(w.len() == 3,
                          line_nr,
                          format!("expected: <pin>::<gate_id>::<pin> - found: {}", token)));

            let src = try!(check_error(w[0].trim().parse::<u8>(), line_nr));
            let id = try!(check_error(w[1].trim().parse::<i64>(), line_nr)) - 1; // the compiler ids starts at 1, but we start at 0
            let dst = try!(check_error(w[2].trim().parse::<u8>(), line_nr));

            gate.connect(wire::Wire::new(src, dst, id));
        }

        gates.push(gate);

        line_nr += 1;
    }

    Ok(gates)
}

fn _parse_gates<I>(from: I) -> Result<Vec<gate::Gate>, io::Error>
    where I: Iterator<Item = Result<String, io::Error>>
{
    let mut gates = Vec::with_capacity(500); // avoid the first reallocs
    let mut line_nr: i64 = 0;
    for line in from {
        let line = try!(line);
        let tokens: Vec<&str> = line.split_whitespace().collect();

        try!(check_if(tokens.len() >= 3,
                      line_nr,
                      format!("expected at least 3 tokens - found: {} tokens",
                              tokens.len())));

        let gate_type = match tokens[0].trim() {
            "AND" => GateType::And,
            "XOR" => GateType::Xor,
            "OR" => GateType::Or,
            "NOT" => GateType::Not,
            _ => {
                return Err(io::Error::new(ErrorKind::InvalidData,
                                          format!("line {}: unknown gate type {}",
                                                  line_nr,
                                                  tokens[0])))
            }
        };



        let num_pins = try!(check_error(tokens[1].trim().parse::<u8>(), line_nr));

        let mut gate = gate::Gate::new(num_pins, gate_type, line_nr);
        for token in tokens.iter().skip(2) {
            let w: Vec<&str> = token.trim().split(":").collect();
            try!(check_if(w.len() == 3,
                          line_nr,
                          format!("expected: <pin>::<gate_id>::<pin> - found: {}", token)));

            let src = try!(check_error(w[0].trim().parse::<u8>(), line_nr));
            let id = try!(check_error(w[1].trim().parse::<i64>(), line_nr)) - 1; // the compiler ids starts at 1, but we start at 0
            let dst = try!(check_error(w[2].trim().parse::<u8>(), line_nr));

            gate.connect(wire::Wire::new(src, dst, id));
        }
        gates.push(gate);

        line_nr += 1;
    }

    return Ok(gates);
}

fn check_if(cond: bool, line_nr: i64, err_msg: String) -> Result<bool, io::Error> {
    match cond {
        false => {
            Err(io::Error::new(ErrorKind::InvalidData,
                               format!("line {}: {}", line_nr, err_msg)))
        }
        _ => Ok(cond),
    }
}

fn check_error<T, E: fmt::Display>(result: Result<T, E>, line_nr: i64) -> Result<T, io::Error> {
    match result {
        Err(why) => {
            Err(io::Error::new(ErrorKind::InvalidData, format!("line {}: {}", line_nr, why)))
        }
        Ok(val) => Ok(val),
    }
}
