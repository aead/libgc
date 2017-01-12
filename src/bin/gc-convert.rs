extern crate libgc;

use std::fs;
use std::env;
use std::path::Path;
use std::process;

use libgc::cbmc::{Parser,Converter,sort_gates};

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
    println!("gc-convert is a tool for converting cbmc-gc compiler output files into the libgc format\n");
    println!("Usage:");
    println!("\t gc-convert -src [src_path] -dst [dst_path]\n");
    println!("\t src\n \t\t path to a directory containing the cbmc-gc files.");
    println!("\t dst\n \t\t path to a directory (must exists) for the libgc files.");
    process::exit(0);
}

// cargo build --release
// ./target/release/gc-convert -src [src_path] -dst [dst_path]

pub fn main(){
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() == 4 && args[0].starts_with("src") && args[2].starts_with("dst") {
        let src_path = fail_on_error!(fs::canonicalize(Path::new(args[1].as_str())), args[1]);
        let dst_path = fail_on_error!(fs::canonicalize(Path::new(args[3].as_str())), args[3]);

        let parser = fail_on_error!(Parser::new(src_path.as_path()));
        let gates = fail_on_error!(parser.parse_gates());
        let sorted_gates = fail_on_error!(sort_gates(&gates));
        let inputs = fail_on_error!(parser.parse_inputs());

        let converter = fail_on_error!(Converter::with_capacity(16*1024*1024, dst_path.as_path()));
        fail_on_error!(converter.convert_gates(&sorted_gates));
        fail_on_error!(converter.convert_wires(&inputs, &sorted_gates));
        fail_on_error!(converter.create_info(&inputs, &sorted_gates));
    }else{ 
        show_help();
    }
}