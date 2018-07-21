use std::io::{BufRead, BufReader};
use std::net::{TcpStream};

use std::collections::HashMap;

pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    pub headers: RequestHeaders
}

type RequestHeaders = HashMap<String, String>;

impl Request {
    pub fn init(stream: &TcpStream) -> Request {
        let mut stream = BufReader::new(stream);
        let mut request_line = String::new();
        match stream.read_line(&mut request_line) {
            Ok(_) => (),
            Err(err) => panic!("error during receive a line: {}", err),
        };

        Request::create_request(&request_line, &mut stream)
    }

    fn create_request(line: &str, stream: &mut BufReader<&TcpStream>) -> Request {
        let mut iter = line.split_whitespace();
        Request {
            method: iter.next().unwrap().to_string(),
            uri: iter.next().unwrap().to_string(),
            version: iter.next().unwrap().to_string(),
            headers: Request::create_header(stream)
        }
    }

    fn create_header(stream: &mut BufReader<&TcpStream>) -> RequestHeaders {
        let mut hash = RequestHeaders::new();

        loop {
            let mut request_line = String::new();
            match stream.read_line(&mut request_line) {
                Ok(size) if size > 2 => {
                    // TODO: if fail to split?
                    let mut contents = request_line.split(":");
                    hash.insert(
                        contents.next().unwrap().trim().to_string(),
                        contents.next().unwrap().trim().to_string());
                },
                Ok(_) =>  break,
                Err(err) => panic!("error during receive a line: {}", err),
            }
        }

        for (key, val) in hash.iter() {
            println!("key: {}, val: {}", key, val);
        }

        hash
    }

    pub fn debug_request(&self) {
        println!("method: {}", self.method);
        println!("uri: {}", self.uri);
        println!("version: {}", self.version);
    }
}
