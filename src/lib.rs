
pub mod parser;
pub mod circuit;

#[cfg(test)]
mod tests {

    use super::parser;
    use std::path::Path;
    // use std::fs::File;
    // use std::io::Write;

    #[test]
    fn example() {
        let mut parser = parser::Parser::new(Path::new("/home/andreas/Desktop/sum"));
        let mut circuit = parser.create_circuit().unwrap();
        circuit.sort();
        println!("{}", circuit);

        // let mut f = File::create("/home/andreas/Desktop/loops1_top.txt").unwrap();
        // write!(f, "{}", circuit)
    }
}
