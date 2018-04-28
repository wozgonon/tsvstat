use std::net::TcpListener;
use std::thread;
use std::env::args;
use std::io::BufReader;

extern crate tsvstat;
use tsvstat::accumulator;

fn usage () {
    println!("Usage: tsvlisten [port] (--help|--version)");
}

fn main() {

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

    match TcpListener::bind(format!("127.0.0.1:{}", port)) {
        Ok(listener) => {
            println!("Listening on port {}", port);
            for stream in listener.incoming() {
                thread::spawn(|| {
                match stream {
                    Ok(tcp_stream) => {
                        let mut accumulators = accumulator::Accumulators::new();
                        let mut buffer = BufReader::new(tcp_stream);
                        accumulators.parse_tsv(&mut buffer);
                        accumulators.print_tsv();
                    },
                    Err(message) => {
                        eprintln!("ERROR {}, could not listen on incoming connection", message);
                    }
                }
                });
            }
        },
        Err(message) => {
            eprintln!("ERROR: Could not bind to port {}, error: {}", port, message);
        }
    }
}
