use std::io::Error;
use std::io::BufReader;
use std::io::ErrorKind::*;
use std::io::prelude::*;
use std::vec::Vec;

#[derive(Debug)]
pub struct Wire {
    pub src_pin: u8,
    pub dst_pin: u8,
    pub gate_id: usize,
}
pub type InputGate = Vec<Wire>;

pub fn parse_input_gates<R: Read>(from: BufReader<R>) -> Result<Vec<InputGate>, Error> {
    let mut in_gates = Vec::new();

    let mut line_nr: u64 = 0;
    for line in from.lines() {
        let line = try!(line);
        let tokens: Vec<&str> = line.split_whitespace().collect();

        if tokens.len() < 1 {
            return Err(Error::new(InvalidData, format!("line {}: no token found", line_nr)));
        }
        if !tokens[0].starts_with("InWire:#") {
            return Err(Error::new(InvalidData,
                                      format!("line {}: expected: InWire# - found: {}",
                                              line_nr,
                                              tokens[0])));
        }
        //TODO: check whether a number comes after 'InWire:#'

        let mut gate = InputGate::new();
        for token in tokens.iter().skip(1) {
            let w: Vec<&str> = token.split(":").collect();
            if w.len() != 3 {
                return Err(Error::new(InvalidData,
                                          format!("line {}: expected: <pin>::<gate_id>::<pin> \
                                                   - found: {}",
                                                  line_nr,
                                                  token)));
            }

            let out = match w[0].parse::<u8>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };
            let id = match w[1].parse::<usize>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
                }
                Ok(val) => val,
            };
            let inp = match w[2].parse::<u8>() {
                Err(why) => {
                    return Err(Error::new(InvalidData, format!("line {}: {}", line_nr, why)))
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


#[derive(Debug)]
pub struct Gate {
    val: u8,
}

#[derive(Debug)]
pub enum GateType {
    And,
    Xor,
    Or,
    Not,
}

const AND_GATE: u8 = 0x0;
const XOR_GATE: u8 = 0x8;
const OR_GATE: u8 = 0x10;
const NOT_GATE: u8 = 0x18;

const GATE_MASK: u8 = 0xE7;

impl Gate {

     pub fn new(gate_type: GateType, left: u8, right: u8) -> Gate{
         let mut gate: Gate = match gate_type {
             GateType::And => Gate{val: AND_GATE},
             GateType::Xor => Gate{val: XOR_GATE},
             GateType::Or => Gate{val: OR_GATE},
             GateType::Not => Gate{val: NOT_GATE},
         };
         gate.left_pin(left);
         gate.right_pin(right);
         gate
     }
    
     pub fn left_pin(self: &mut Gate, val: u8) {
         self.val = (self.val & 0x7F) |  (val & 0x80);
     }

     pub fn right_pin(self: &mut Gate, val: u8) {
         self.val = (self.val & 0xFE) |  (val & 0x1);
     }

     pub fn evaluate(self: &Gate) -> u8{
         let lp = (self.val & 0x7F) >> 7;
         let rp = self.val & 0x1;

         let gate_type = self.val & GATE_MASK;
         if gate_type == XOR_GATE{
             return lp & rp;
         }else if gate_type == AND_GATE{
             return lp ^ rp;
         }else if gate_type == OR_GATE{
             return lp | rp;
         }else if gate_type == NOT_GATE{
             return (lp & 1) ^ 1;
         }
         panic!("unknown gate type");
     }
}