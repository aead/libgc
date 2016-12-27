extern crate libgc;

use std::env;
use std::path::Path;
use libgc::parser::Parser;
use libgc::circuit::Circuit;

// run this by: cargo run --bin example 'path'
// for example: cargo run --bin example /home/andreas/Desktop/sum

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Missing path to cbmc-gc compiler output");
        return;
    }
    let parser = Parser::new(Path::new(&args[1]));
    let gates = match parser.parse_gates() {
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };
    let inputs = match parser.parse_inputs() {
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };
    let mut circuit = Circuit::new(inputs, gates);
    circuit.sort();
    println!("{}", circuit);
}