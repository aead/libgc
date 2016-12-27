extern crate libgc;

use libgc::parser::{Gate, ID, GateType, Wire, Pin};

#[test]
fn gatetype_pins() {
    assert_eq!(2, GateType::And.pins());
    assert_eq!(2, GateType::Xor.pins());
    assert_eq!(2, GateType::Or.pins());
    assert_eq!(1, GateType::Not.pins());
}

#[test]
fn id_u64() {
    assert_eq!(ID::Gate(0), 0);
    assert_eq!(ID::Gate(1), 1);
    assert_eq!(ID::Gate(0xFFFFFFFFFFFFFFFF), 0xFFFFFFFFFFFFFFFF);

    assert_eq!(ID::Input(0), 0);
    assert_eq!(ID::Input(1), 1);
    assert_eq!(ID::Input(0xFFFFFFFFFFFFFFFF), 0xFFFFFFFFFFFFFFFF);

    assert_eq!(ID::Output(0), 0);
    assert_eq!(ID::Output(1), 1);
    assert_eq!(ID::Output(0xFFFFFFFFFFFFFFFF), 0xFFFFFFFFFFFFFFFF);
}

#[test]
fn id_index() {
    assert_eq!(ID::Gate(1).index(), 0);
    assert_eq!(ID::Gate(0xFFFFFFFFFFFFFFFF).index(), 0xFFFFFFFFFFFFFFFE);

    assert_eq!(ID::Input(1).index(), 0);
    assert_eq!(ID::Input(0xFFFFFFFFFFFFFFFF).index(), 0xFFFFFFFFFFFFFFFE);

    assert_eq!(ID::Output(1).index(), 0);
    assert_eq!(ID::Output(0xFFFFFFFFFFFFFFFF).index(), 0xFFFFFFFFFFFFFFFE);
}

#[test]
fn gate_gate_type() {
    assert_eq!(GateType::And, Gate::new(GateType::And, 1).gate_type());
    assert_eq!(GateType::Or, Gate::new(GateType::Or, 1).gate_type());
    assert_eq!(GateType::Xor, Gate::new(GateType::Xor, 1).gate_type());
    assert_eq!(GateType::Not, Gate::new(GateType::Not, 1).gate_type());
}

#[test]
fn gate_id() {
    assert_eq!(Gate::new(GateType::And, 1).id(), ID::Gate(1));
    assert_eq!(Gate::new(GateType::Or, 2).id(), ID::Gate(2));
    assert_eq!(Gate::new(GateType::Xor, 3).id(), ID::Gate(3));
    assert_eq!(Gate::new(GateType::Not, 19).id(), ID::Gate(19));
}

#[test]
fn gate_copy() {
    let mut gate = Gate::new(GateType::And, 1);
    gate.connect(Wire::new(Pin::Left, Pin::Right, ID::Gate(2)));
    gate.connect(Wire::new(Pin::Right, Pin::Left, ID::Gate(3)));

    let copy = gate.copy();

    assert!(gate == copy);

    let mut copy2 = gate.copy();
    copy2.disconnect_all();
    copy2.connect(Wire::new(Pin::Left, Pin::Right, ID::Gate(1)));
    copy2.connect(Wire::new(Pin::Right, Pin::Right, ID::Gate(3)));

    assert!(gate == copy);
    assert!(gate != copy2);
    assert!(copy != copy2);
}