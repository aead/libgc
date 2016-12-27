
extern crate libgc;

use libgc::parser::{Wire, Pin, ID};

#[test]
fn destination_type() {
    let wire = Wire::new(Pin::Left, Pin::Right, ID::Input(0));
    assert!(wire.is_input());
    assert!(!wire.is_output());

    let wire = Wire::new(Pin::Left, Pin::Right, ID::Output(0));
    assert!(!wire.is_input());
    assert!(wire.is_output());

    let wire = Wire::new(Pin::Left, Pin::Right, ID::Gate(0));
    assert!(!wire.is_input());
    assert!(!wire.is_output()); 
}

#[test]
fn pins() {
    let wire = Wire::new(Pin::Left, Pin::Right, ID::Input(0));
    assert_eq!(Pin::Left, wire.src_pin());
    assert_eq!(Pin::Right, wire.dst_pin());

    let wire = Wire::new(Pin::Right, Pin::Left, ID::Input(0));
    assert_eq!(Pin::Right, wire.src_pin());
    assert_eq!(Pin::Left, wire.dst_pin());
}

#[test]
fn destination_gate() {
    let wire = Wire::new(Pin::Left, Pin::Right, ID::Input(0));
    assert_eq!(ID::Input(0), wire.dst_gate());

    let wire = Wire::new(Pin::Right, Pin::Left, ID::Output(1));
    assert_eq!(ID::Output(1), wire.dst_gate());

    let wire = Wire::new(Pin::Right, Pin::Left, ID::Gate(2));
    assert_eq!(ID::Gate(2), wire.dst_gate());
}