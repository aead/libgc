extern crate libgc;

use std::env;
use std::path::Path;
use libgc::parser;
use libgc::circuit::binary;

#[test]
fn parse_libgc() {
    let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum");
    parser::parse_meta_info(path.as_path()).unwrap();

    let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum");
    parser::parse_circuit(path.as_path()).unwrap();
}

#[test]
fn execute_libgc_sum() {
    let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum");
    execute_libgc(path.as_path());
}

// to run this test, you must set the c0 path in sum2/sub/meta_info.txt to a valid path
// change: /home/andreas/rust/libgc/tests/libgc_output/sum2/sub to a valid path
// #[test] 
fn execute_libgc_sum2() {
    let path = env::current_dir().unwrap().join("tests").join("libgc_output").join("sum2");
    execute_libgc(path.as_path());
}

fn execute_libgc(path: &Path) {
    let info = parser::parse_meta_info(path).unwrap();
    let mut circuit = binary::Circuit::new(info).unwrap();

    // test 0 + 0 = 0
    set_pin_from_to(&mut circuit, 1, 64, 0);
    circuit.execute().unwrap();
    let output: Vec<u8> = circuit.collect_output();
    let expected = vec![0u8; 32];
    assert_eq!(output, expected);

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
