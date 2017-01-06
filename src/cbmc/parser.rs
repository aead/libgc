
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};
use std::fs::File;

use super::types::*;
use super::error::Error;

const GATES: &'static str = "output.gate.txt";
const INPUTS: &'static str = "output.inputs.txt";
const CONSTS: &'static str = "output.constants.txt";
const NUM_OF_GATES: &'static str = "output.numberofgates.txt";
const NUM_OUT_BITS: &'static str = "output.noob.txt";

pub struct Parser<'a> {
    path: &'a Path,
}

impl<'a> Parser<'a> {
    fn join_path(&self, filename: &'static str) -> PathBuf {
        let mut pathbuf = PathBuf::new();
        pathbuf.push(self.path);
        pathbuf.push(Path::new(filename));
        pathbuf
    }

    pub fn new(path: &Path) -> Parser {
        Parser { path: path }
    }

    pub fn parse_gates(&self) -> Result<Vec<Gate>, Error> {
        let mut gates = Vec::with_capacity(try!(self.parse_num_of_gates()));
        let mut line_nr: u64 = 1;

        let reader = BufReader::new(try!(File::open(self.join_path(GATES).as_path())));
        for line in reader.lines() {
            gates.push(try!(Gate::parse(try!(line).as_str(), line_nr)));
            line_nr += 1;
        }
        Ok(gates)
    }

    fn parse_num_of_gates(&self) -> Result<usize, Error> {
         let mut reader = BufReader::new(try!(File::open(self.join_path(NUM_OF_GATES).as_path())));
         let mut buf = String::default();
         try!(reader.read_line(&mut buf));
         
         let num_of_gates = try!(buf.trim().parse::<usize>());
         Ok(num_of_gates)
    }

    pub fn parse_inputs(&self) -> Result<Vec<IOPin>, Error> {
        let mut pins = Vec::with_capacity(try!(self.parse_num_of_output_bits()));
        let mut line_nr: u64 = 1;

        let reader = BufReader::new(try!(File::open(self.join_path(INPUTS).as_path())));
        for line in reader.lines() {
            pins.push(try!(IOPin::parse_input(try!(line).as_str(), line_nr)));
            line_nr += 1;
        }
        Ok(pins)
    }

    fn parse_num_of_output_bits(&self) -> Result<usize, Error> {
         let mut reader = BufReader::new(try!(File::open(self.join_path(NUM_OUT_BITS).as_path())));
         let mut buf = String::default();
         try!(reader.read_line(&mut buf));
         
         let num_of_gates = try!(buf.trim().parse::<usize>());
         Ok(num_of_gates)
    }
}
