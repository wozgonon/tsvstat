//use expression::Parser;

mod accumulator;
mod expression;

fn main() {

    use std::io;
    use std::io::BufRead;

    //let mut parser = Parser::new();
    //parser.parse_tokens ();

    let delimiter = "\t";
    let mut accumulators = accumulator::Accumulators::new();
    let reader = io::stdin();
    let buffer = io::BufReader::new(reader);

    for line in buffer.lines () {
        accumulators.new_row();
        match line {
            Err(message) => eprintln!("Error '{}' while parsing line", message),
            Ok(string) => {
                let mut split = string.split(delimiter);
                for value in split {
                    accumulators.add_column_value(value);
                }
            }
        }
    }
    accumulators.print_tsv ();
}

