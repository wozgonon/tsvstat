extern crate tsvstat;

use tsvstat::tcp;
use std::path::Path;
use std::io::BufReader;
use std::fs::File;
use std::thread;
use std::time::Duration;
use tsvstat::tcp::StdoutWriter;


#[test]
fn should_send_recieve_and_summarize_tsv() {

    let port = 19999;
    let name = "./tests/test1.tsv";
    let path = Path::new(name);

    match File::open(&path) {
        Err(message) => {
            assert!(false, format!("ERROR: open file {}, error: {}", name, message));
        },
        Ok(file) => {
            let reader = BufReader::new(file);
            let writer = StdoutWriter::new ();

            thread::spawn(move || {
                tcp::Listener::listen(port, 1, &writer);
            });
            tcp::Sender::send(port.to_string(), reader);
            // FIXME send to somewhere other stand stdout
            thread::sleep(Duration::from_millis(500));  // FIXME This is very brittle.
        }
    }
}
