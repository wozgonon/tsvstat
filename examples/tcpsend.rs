use std::io::Write;
use std::env::args;
use std::net::TcpStream;
use std::io::BufRead;
use std::str::FromStr;
use std::io;

fn usage () {
    println!("Usage: tcpsend [<address>:<port>]");
}

/// Reads lines of text from standard input and sends them over a TCP connection.
fn main() {

    let mut address: String = String::from_str ("9999").unwrap();
    let mut count = 0;
    for arg in args () {
        if arg == "--help" {
            usage ();
            break;
        }
        if count > 0 {
            address = arg;
            break;
        }
        count = count + 1;
    }
    if ! address.contains(":") {
        address = format!("127.0.0.1:{}", address);
    }
    let ip_address = address.clone();
    println!("Connecting to address: {}", ip_address);

    match TcpStream::connect(&ip_address) {
        Ok(mut tcp_stream) => {
            let reader = io::stdin();
            let buffer = io::BufReader::new(reader);
            for line in buffer.lines() {
                match line {
                    Ok(line) => {
                        match tcp_stream.write(line.as_bytes()) {
                            Ok(_size) => {
                                tcp_stream.write ("\n".as_bytes()).unwrap();
                                /* Is this okay to ignore? */
                            },
                            Err(message) => {
                                eprintln!("Error '{}' writing to: {}", message, ip_address);
                                break;
                            }
                        }
                    },
                    Err(message) => eprintln!("Error '{}' reading stdin", message)
                }
            }
        },
        Err(message) => eprintln!("Error '{}' openning connection to address: {}", message, ip_address)
    }
}
