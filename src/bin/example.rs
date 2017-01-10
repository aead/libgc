extern crate libgc;

use std::env;
use std::path::Path;
use libgc::cbmc::{Parser, Converter, sort_gates};

// run this by: cargo run --bin example 'src_path' 'dst_path'
// for example: cargo run --bin example /home/andreas/Desktop/sum /home/andreas/Desktop/ligc_sum

fn main(){
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Missing path to cbmc-gc compiler output");
        return;
    }
    if args.len() != 3 {
        println!("Missing path to libgc output");
        return;
    }
    let parser = Parser::new(Path::new(&args[1]));
    let mut gates = match parser.parse_gates() {
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };
    gates = match sort_gates(&mut gates){
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };
    let inputs = match parser.parse_inputs() {
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };
    let converter = match Converter::new(Path::new(&args[2])){
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };

    match converter.convert_gates(&gates){
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };
    match converter.convert_wires(&inputs, &gates){
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };
    match converter.create_info(&inputs, &gates){
        Ok(val) => val,
        Err(why) => {
            println!("{}", why);
            return;
        },
    };
}