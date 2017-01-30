extern crate libgc;
extern crate getopts;

use std::fs;
use std::env;
use std::path::Path;

use libgc::cbmc::{Parser,Converter,sort_gates};
use getopts::{Options,Matches};

macro_rules! fail_on_error {
    ($exp:expr, $msg:expr) => {
        match $exp {
            Ok(val) => val,
            Err(why) => {
                println!("{} - {}", $msg, why);
                return;
            },
        }
    };
    ($exp:expr) => {
        match $exp {
            Ok(val) => val,
            Err(why) => {
                println!("{}", why);
                return;
            },
        }
    };
}

macro_rules! must {
    ($exp:expr, $fail:expr) => {
        match $exp {
            Some(val) => val,
            None => {
                $fail;
                return;
            },
        }
    };
}

const GIT_REPO: &str = "https://github.com/aead/libgc/issues";

fn help(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn short_help(program: &str, opts: Options) {
    println!("{}", opts.short_usage(program));
}

fn bug() {
    println!("This is a bug: Please open an issue at: {}" , GIT_REPO);
}

// cargo build --release
// ./target/release/gc-convert --src SRC -dst DST --keep-NOT

pub fn main(){
    let mut opts = Options::new();
    opts.optopt("", "src", "path to a directory containing the cbmc-gc files.", "SRC");
    opts.optopt("", "dst", "path to a directory (must exists) for the libgc files.", "DST");
    opts.optopt("", "cap", "IO buffering in MB - default is 16", "CAPACITY");
    opts.optflag("", "keep-NOT", "disable NOT gate replacement - a binary circuit containing NOT gates cannot turned into a garbled circuit");
    opts.optflag("h", "help", "print this help menu");
    
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let matches: Matches = fail_on_error!(opts.parse(&args[1..]));
    
    if matches.opt_present("h") {
        help(&program, opts);
        return;
    }

    let src = must!(matches.opt_str("src"), short_help(&program, opts));
    let dst = must!(matches.opt_str("dst"), short_help(&program, opts));
    
    let src_path = fail_on_error!(fs::canonicalize(Path::new(src.as_str())), src);
    let dst_path = fail_on_error!(fs::canonicalize(Path::new(dst.as_str())), dst);

    let cap = 1024 * 1024 * if matches.opt_present("cap") {
        fail_on_error!(must!(matches.opt_default("cap", "16"), bug()).parse::<usize>())
    }else{
        16
    };

    let parser = fail_on_error!(Parser::with_capacity(cap, src_path.as_path()));
    let gates = fail_on_error!(parser.parse_gates());
    let sorted_gates = fail_on_error!(sort_gates(&gates));
    let inputs = fail_on_error!(parser.parse_inputs());
    let constant = fail_on_error!(parser.parse_constant());

    let converter = fail_on_error!(Converter::with_capacity(cap, dst_path.as_path()));    
    let (sorted_gates, constant) = if !matches.opt_present("keep-NOT") { 
        converter.replace_not_gates(&sorted_gates, constant)
    }else{
        (sorted_gates, constant)
    };

    fail_on_error!(converter.convert_circuit(&inputs, &sorted_gates));
    fail_on_error!(converter.create_meta_info(&inputs, &sorted_gates, constant));
}