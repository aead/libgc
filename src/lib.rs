
mod parser;
mod circuit;

#[cfg(test)]
mod tests {

    use super::parser;
    use std::path::Path;

    #[test]
    fn example() {
        let mut parser = parser::Parser::new(Path::new("/home/andreas/Desktop/loops1"));
        println!("{}",parser.create_circuit().unwrap());
    }
}
