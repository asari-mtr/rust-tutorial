extern crate time;

use std::io;
use std::io::{Write, BufRead};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let mut stream = io::BufReader::new(stream);

                let mut first_line = String::new();

                if let Err(err) = stream.read_line(&mut first_line) {
                    panic!("error during receive a line: {}", err);
                }

                println!("{}", first_line);
            }
            Err(e) => {
                println!("Connection fail!");
            }
        }
    }
}
