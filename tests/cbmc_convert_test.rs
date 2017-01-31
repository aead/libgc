extern crate libgc;

use std::env;
use libgc::cbmc;
use libgc::circuit::binary;

#[test]
fn parse_cbmc_sum(){
    let path = env::current_dir().unwrap().join("tests").join("cbmc_output").join("sum");
    let parser: cbmc::Parser = cbmc::Parser::new(path.as_path()).unwrap();   
    parser.parse_inputs().unwrap();
    parser.parse_gates().unwrap();
    parser.parse_constant().unwrap();
}

#[test]
fn execute_libgc_sum(){
    let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum");
    let mut circuit = binary::BinaryCircuit::new(path.as_path());
    
    // test 0 + 0 = 0
    let output: Vec<u8> = circuit.execute().unwrap();
    let expected = vec![0u8; 32];
    assert_eq!(output, expected);

    // test 1 + 0 = 1
    circuit.set_input_pin(1); // Alice
    let output: Vec<u8> = circuit.execute().unwrap();
    let mut expected = vec![0u8; 32];
    expected[0] = 1;
    assert_eq!(output, expected);

    // test 1 + 1 = 2
    circuit.set_input_pin(1);   // Alice
    circuit.set_input_pin(33);  // Bob
    let output: Vec<u8> = circuit.execute().unwrap();
    let mut expected = vec![0u8; 32];
    expected[1] = 1;
    assert_eq!(output, expected);

    // test 4 + 5 = 9
    circuit.set_input_pin(3);  // Alice
    circuit.set_input_pin(1);  // Bob
    circuit.set_input_pin(3);  // Bob
    let output: Vec<u8> = circuit.execute().unwrap();
    let mut expected = vec![0u8; 32];
    expected[0] = 1;
    expected[2] = 1;
    assert_eq!(output, expected);

    // test 0xFFFFFFFF + 2 = 1 // overflow
    let mut i = 1;
    while i <= 32 {             // Alice
        circuit.set_input_pin(i);
        i += 1;
    }
    circuit.set_input_pin(34);  // Bob
    let output: Vec<u8> = circuit.execute().unwrap();
    let mut expected = vec![0u8; 32];
    expected[0] = 1;
    assert_eq!(output, expected);
}