extern crate libgc;

use std::env;
use std::path::Path;
use libgc::cbmc::{Parser,sort_gates};

// run this by: cargo run --bin example 'path'
// for example: cargo run --bin example /home/andreas/Desktop/sum

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Missing path to cbmc-gc compiler output");
        return;
    }
    let parser = Parser::new(Path::new(&args[1]));
    let mut gates = match parser.parse_gates() {
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        }
    };
    match sort_gates(&mut gates){
        Ok(val) => {
            for gate in val {
                println!("{}", gate);
            }
        },
        Err(why) => {
            println!("{}", why);
            return;
        }
    }
}