extern crate libgc;

use std::fs;
use std::env;
use std::path::Path;
use std::process;

use libgc::circuit::binary::BinaryCircuit;

macro_rules! fail_on_error {
    ($exp:expr, $msg:expr) => {
        match $exp {
            Ok(val) => val,
            Err(why) => {
                println!("{} - {}", $msg, why);
                process::exit(1);
            },
        }
    };
    ($exp:expr) => {
        match $exp {
            Ok(val) => val,
            Err(why) => {
                println!("{}", why);
                process::exit(1);
            },
        }
    };
}

fn show_help(){
    println!("gc-binexec is a tool for executing a binary circuit from a libgc circuit-file\n");
    println!("Usage:");
    println!("\t gc-binexec -c [circuit_path] [input bits]\n");
    println!("\t c\n \t\t path to a directory containing the libgc circuit files.");
    println!("");
    println!("Example:");
    println!("\t gc-binexec -c /home/foo/sum32 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
    process::exit(0);
}

pub fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() >= 2 && args[0].starts_with("-c") {
        let path = fail_on_error!(fs::canonicalize(Path::new(args[1].as_str())), args[1]);
        let mut circuit = BinaryCircuit::new(path.as_path());

        let mut i = 1;
        for arg in args.into_iter().skip(2) {
            if arg.trim() == "1" {
                circuit.set_input_pin(i);
            }
            i += 1
        }

        let out_bits = fail_on_error!(circuit.execute());
        for bit in out_bits {
            print!("{} ", bit);
        }
        println!("");
    }else{ 
        show_help();
    }
}