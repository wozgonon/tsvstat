use std::io::BufReader;
use std::net::TcpListener;
use std::thread;
use std::io::Write;
use std::net::TcpStream;
use std::io::BufRead;
use std::io::Stdout;

use accumulator;
use std::io::Read;
use std::io;

pub struct Sender {
}

impl Sender {
    pub fn send<R: Read> (address : String, reader : R) {

        let ip_address : String;
        if ! address.contains(":") {
            ip_address = format!("127.0.0.1:{}", address);
        } else {
            ip_address = address;
        }
        println!("Connecting to address: {}", ip_address);

        match TcpStream::connect(&ip_address) {
            Ok(mut tcp_stream) => {
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
                //tcp_stream.flush();
            },
            Err(message) => eprintln!("Error '{}' openning connection to address: {}", message, ip_address)
        }
    }
}
pub trait Writer<W>  : Sync  + Send {
    fn output (&self) -> W;
}

pub struct StdoutWriter  {
}
impl StdoutWriter {
    pub fn new () -> StdoutWriter {
        return StdoutWriter {};
    }
}
impl Writer<Stdout> for StdoutWriter {
    fn output (&self) -> Stdout {
        return io::stdout ();
    }
}

pub struct Listener {
}

impl Listener {

    pub fn listen_forever<W : Write + Send> (port : u32, writer : &Writer<W>) {
        Listener::listen (port, -1, writer);
    }
    pub fn listen<W: Write> (port : u32, mut live : i32, writer : &Writer<W>) -> bool {
        match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(listener) => {
                println!("Listening on port {}", port);
                for stream in listener.incoming() {
                    let mut output = io::stdout ();   // This should be passed as a parameter
                    thread::spawn(move || {
                        match stream {
                            Ok(tcp_stream) => {
                                let mut accumulators = accumulator::Accumulators::new();
                                let mut buffer = BufReader::new(tcp_stream);
                                accumulators.parse_tsv(&mut buffer);
                                accumulators.print_tsv(&mut output);
                                return true;
                            },
                            Err(message) => {
                                eprintln!("ERROR {}, could not listen on incoming connection", message);
                                return false;
                            }
                        };
                    });
                    live = live - 1;
                    if live <= 0 {
                        return true;
                    }
                }
            },
            Err(message) => {
                eprintln!("ERROR: Could not bind to port {}, error: {}", port, message);
                return false;
            }
        }
        return false;
    }
}