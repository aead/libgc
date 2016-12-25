
pub mod parser;
pub mod circuit;

#[cfg(test)]
mod tests {

    use super::parser;
    use super::circuit;
    use std::path::Path;
    // use std::fs::File;
    // use std::io::Write;

    #[test]
    fn example() {
        let mut p = parser::Parser::new(Path::new("/home/andreas/Desktop/sum"));

        let gates = match p.parse_gates() {
            Ok(val) => val,
            Err(why) => {
                println!("{}", why);
                return;
            }
        };

        let inputs = match p.parse_inputs() {
            Ok(val) => val,
            Err(why) => {
                println!("{}", why);
                return;
            }
        };

        let mut circuit = circuit::Circuit::new(inputs, gates);
        circuit.sort();
        println!("{}", circuit);

        // let mut f = File::create("/home/andreas/Desktop/loops1_top.txt").unwrap();
        // write!(f, "{}", circuit)
    }
}
