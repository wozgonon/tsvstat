use std::env::args;

extern crate tsvstat;
//use tsvstat::accumulator;
use tsvstat::tcp;
use tsvstat::tcp::StdoutWriter;

fn usage () {
    println!("Usage: tsvlisten [port] (--help|--version)");
}


fn main() {

    let writer = StdoutWriter::new ();

    let mut port = 9999;
    let mut count = 0;
    for arg in args() {
        if arg == "--help" {
            usage();
            return;
        }
        if arg == "--version" {
            println!("tsvlisten {}", tsvstat::version ());
            return;
        }
        if count > 1 {
            match arg.parse() {
                Err(message) => {
                    eprintln!("ERROR {}, Expect port number, not '{}'", message, arg);
                },
                Ok(number) => {
                    port = number;
                    break;
                }
            }
        }
        count = count + 1;
    }

    //tcp::Listener::listen (port, 1);
    tcp::Listener::listen_forever (port, &writer);
 }
