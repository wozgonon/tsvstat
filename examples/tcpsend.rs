extern crate tsvstat;

use std::io;
use std::env::args;
use std::str::FromStr;
use tsvstat::tcp;

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

    tcp::Sender::send (address, io::stdin());
}
