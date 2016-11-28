
mod parser;

#[cfg(test)]
mod tests {
    use super::parser;
    use std::path::Path;
    use std::fs::File;
    use std::io::BufReader;

    #[test]
    fn example() {
        // replace it with the path to the output.inputs.txt
        // e.g. /home/me/Desktop/output.inputs.txt
        let path = Path::new("/home/andreas/Desktop/output.inputs.txt");
        let reader = BufReader::new(try!(File::open(path)));


        let input_gates = !try(parser::parse_input_gates(reader));
        let mut i = 0;
        for gate in input_gates {
            println!("Input gate: {}", i);
            i += 1;

            let mut j = 0;
            for wire in &gate {
                print!("\tpin {} -> pin {} of gate {}",
                       wire.src_pin,
                       wire.dst_pin,
                       wire.gate_id);
                if j < &gate.len() - 1 {
                    print!("\tAND");
                    j += 1;
                }
            }
            println!("");
        }
    }
}
