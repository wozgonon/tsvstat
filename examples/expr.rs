extern crate tsvstat;
use tsvstat::expression;
use tsvstat::expression::Parser;

fn main() {

    use std::io;
    use std::io::BufRead;
    let mut parser = Parser::new ();

    let mut count = 0;
    for arg in std::env::args() {
        if count > 0 {
            println!("{}=", arg);
            let result = parser.parse_text(arg.as_str());
            println!("{}={}", arg, result);
        }
        count = count + 1
    }
}

