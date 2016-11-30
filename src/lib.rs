
mod parser;

#[cfg(test)]
mod tests {
    use super::parser;
    use std::path::Path;
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    #[test]
    fn example_parse_input_gates() {
        // replace it with the path to the output.inputs.txt
        // e.g. /home/me/Desktop/output.inputs.txt
        let path = Path::new("/home/andreas/Desktop/output.inputs.txt");
        let reader = BufReader::new(File::open(path).unwrap());

        let input_gates = parser::parse_input_gates(reader.lines()).unwrap();

        let mut i = 0;
        for gate in input_gates {
            print!("Input gate: {} -> ", i);
            i += 1;

            let mut j = 0;
            for wire in &gate {
                print!("pin {} of gate {}",
                       wire.dst_pin,
                       wire.gate_id);
                if j < &gate.len() - 1 {
                    print!("\tAND\t");
                    j += 1;
                }
            }
            println!("");
        }
    }

    #[test]
    fn example_parse_gates(){
        let path = Path::new("/home/andreas/Desktop/output.gate.txt");
        let reader = BufReader::new(File::open(path).unwrap());

        let gates = parser::parse_gates(reader.lines()).unwrap();

        let mut i = 0;
        for gate in gates {

            print!("{} gate: {} > ", &gate.get_type(), i);
            i += 1;

            let mut j = 0;
            let wires = gate.get_connections();
            for wire in wires {
                print!("in {} of gate {}",
                       wire.dst_pin,
                       wire.gate_id);
                if j < wires.len() - 1 {
                    print!("\t");
                    j += 1;
                }
            }
            println!("");
        }
    }
}
