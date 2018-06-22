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

                loop {
                    let mut line = String::new();
                    match stream.read_line(&mut line) {
                        Ok(0) => {
                            println!("end");
                            break;
                        },
                        Err(err) => panic!("error during receive a line: {}", err),
                        _ => print!("{}", line)
                    }
                }
            }
            Err(e) => {
                println!("Connection fail!");
            }
        }
    }
}
