
mod parser;

#[cfg(test)]
mod tests {

    use super::parser;
    use std::path::Path;

    #[test]
    fn example() {
        let mut parser = parser::Parser::new(Path::new("/home/andreas/Desktop/sum"));
        println!("{}",parser.parse_number_of_gates().unwrap());
        println!("{}",parser.parse_number_of_output_bits().unwrap());
        for gate in parser.parse_gates().unwrap(){
            println!("{}",gate);
        }
    }
}
