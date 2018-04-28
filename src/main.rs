
extern crate tsvstat;
use tsvstat::accumulator;

fn main() {

    use std::io;

    let mut accumulators = accumulator::Accumulators::new();
    let reader = io::stdin();
    let mut buffer = io::BufReader::new(reader);
    let mut output = io::stdout ();
    accumulators.parse_tsv(&mut buffer);
    accumulators.print_tsv (&mut output);
}

