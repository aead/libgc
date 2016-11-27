use std::io;
use std::io::ErrorKind::*;
use std::io::prelude::*;
use std::fs::File;
use std::vec::Vec;
use std::path::Path;

#[derive(Debug)]
pub struct Wire {
    pub src_pin: u8,
    pub dst_pin: u8,
    pub gate_id: usize,
}
pub type InputGate = Vec<Wire>;

pub fn parse_input_gates(path: &Path) -> Result<Vec<InputGate>, io::Error> {
    let file = try!(File::open(path));

    let reader = io::BufReader::new(file);
    let mut in_gates = Vec::new();

    let mut line_nr: u64 = 0;
    for line in reader.lines() {
        let line = try!(line);
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() < 1 {
            return Err(io::Error::new(InvalidData, format!("line {}: no token found", line_nr)));
        }
        if !tokens[0].starts_with("InWire:#") {
            return Err(io::Error::new(InvalidData,
                                      format!("line {}: expected: InWire# - found: {}",
                                              line_nr,
                                              tokens[0])));
        }

        let mut gate = InputGate::new();
        for token in tokens.iter().skip(1) {
            let w: Vec<&str> = token.split(":").collect();
            if w.len() != 3 {
                return Err(io::Error::new(InvalidData,
                                          format!("line {}: expected: <pin>::<gate_id>::<pin> \
                                                   - found: {}",
                                                  line_nr,
                                                  token)));
            }

            let out = match w[0].parse::<u8>() {
                Err(why) => {
                    return Err(io::Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };
            let id = match w[1].parse::<usize>() {
                Err(why) => {
                    return Err(io::Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };
            let inp = match w[2].parse::<u8>() {
                Err(why) => {
                    return Err(io::Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };

            gate.push(Wire {
                src_pin: inp,
                dst_pin: out,
                gate_id: id,
            });
        }
        in_gates.push(gate);

        line_nr += 1;
    }

    Ok(in_gates)
}
