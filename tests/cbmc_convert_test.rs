extern crate libgc;

use std::env;
use libgc::cbmc;
use libgc::parser;
use libgc::circuit::binary;

#[test]
fn parse_cbmc_sum(){
    //let path = env::current_dir().unwrap().join("tests").join("cbmc_output").join("sum");
    let path = env::home_dir().unwrap().join("rust/libgc/tests/cbmc_output/sum");
    let parser: cbmc::Parser = cbmc::Parser::new(path.as_path()).unwrap();   
    parser.parse_inputs().unwrap();
    parser.parse_gates().unwrap();
    parser.parse_constant().unwrap();
}

#[test]
fn parse_libgc() {
    //let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum").join("meta_info.txt");
    let path = env::home_dir().unwrap().join("rust/libgc/tests/libgc_output/sum/");
    let info = parser::parse_meta_info(path.as_path()).unwrap();

    let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum").join("circuit.txt");
    match parser::parse_circuit(path.as_path()) {
        Err(why) => println!("{}", why),
        Ok(val) => {
            for node in val {
                //println!("{}", node);
            }   
        },
    }
}

#[test]
fn execute_libgc_sum(){
    //let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum").join("meta_info.txt");
    let path = env::home_dir().unwrap().join("rust/libgc/tests/libgc_output/sum/");
    let info = parser::parse_meta_info(path.as_path()).unwrap();
    let mut circuit = binary::Circuit::new(info).unwrap();
    
    // test 0 + 0 = 0
    set_pin_from_to(&mut circuit, 1, 64, 0);
    circuit.execute().unwrap();
    let output: Vec<u8> = circuit.collect_output();
    let expected = vec![0u8; 32];
    assert_eq!(output, expected);

    println!("Hallo");

    // test 1 + 0 = 1
    set_pin_from_to(&mut circuit, 1, 64, 0);
    circuit.set_input(1, 1); // Alice
    circuit.execute().unwrap();
    let output: Vec<u8> = circuit.collect_output();
    let mut expected = vec![0u8; 32];
    expected[0] = 1;
    assert_eq!(output, expected);

    // test 1 + 1 = 2
    set_pin_from_to(&mut circuit, 1, 64, 0);
    circuit.set_input(1, 1);  // Alice
    circuit.set_input(33, 1); // Bob
    circuit.execute().unwrap();
    let output: Vec<u8> = circuit.collect_output();
    let mut expected = vec![0u8; 32];
    expected[1] = 1;
    assert_eq!(output, expected);

    // test 4 + 5 = 9
    set_pin_from_to(&mut circuit, 1, 64, 0);
    circuit.set_input(3, 1);  // Alice
    circuit.set_input(33, 1); // Bob
    circuit.set_input(35, 1);  // Bob
    circuit.execute().unwrap();
    let output: Vec<u8> = circuit.collect_output();
    let mut expected = vec![0u8; 32];
    expected[0] = 1;
    expected[3] = 1;
    assert_eq!(output, expected);

    // test 0xFFFFFFFF + 2 = 1 // overflow
    set_pin_from_to(&mut circuit, 1, 64, 0);
    set_pin_from_to(&mut circuit, 1, 32, 1); // Alice
    circuit.set_input(34, 1); // Bob
    circuit.execute().unwrap();
    let output: Vec<u8> = circuit.collect_output();
    let mut expected = vec![0u8; 32];
    expected[0] = 1;
    assert_eq!(output, expected);
}

fn set_pin_from_to(c: &mut binary::Circuit, from: u64, to: u64, val: u8) {
    let mut i = from;
    while i <= to {
        c.set_input(i, val);
        i += 1;
    }
}