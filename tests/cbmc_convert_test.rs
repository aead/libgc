extern crate libgc;

use std::env;
use libgc::cbmc;

#[test]
fn parse_cbmc_sum() {
    let path = env::current_dir().unwrap().join("tests").join("cbmc_output").join("sum");
    let parser: cbmc::Parser = cbmc::Parser::new(path.as_path()).unwrap();
    parser.parse_inputs().unwrap();
    parser.parse_gates().unwrap();
    parser.parse_constant().unwrap();
}
