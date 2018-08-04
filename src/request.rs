use std::io::{Error, Read, BufRead, BufReader};

use std::collections::HashMap;

use http_method::*;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: RequestHeaders
}

type RequestHeaders = HashMap<String, String>;

impl Request {
    pub fn new<R: Read>(stream: R) -> Request {
        let mut stream = BufReader::new(stream);
        let mut request_line = String::new();
        if let Err(err) = stream.read_line(&mut request_line) {
            panic!("error during receive a line: {}", err)
        };

        let mut iter = request_line.split_whitespace();
        Request {
            method: HttpMethod::from_str(iter.next().unwrap()).unwrap(),
            uri: iter.next().unwrap().to_string(),
            version: iter.next().unwrap().to_string(),
            headers: Request::create_header(&mut stream)
        }
    }

    fn create_header<R: Read>(stream: &mut BufReader<R>) -> RequestHeaders {
        let mut headers = RequestHeaders::new();

        loop {
            let mut request_line = String::new();
            match stream.read_line(&mut request_line) {
                Ok(size) if size > 2 => {
                    // TODO: if fail to split?
                    let mut contents = request_line.split(":");
                    headers.insert(
                        contents.next().unwrap().trim().to_string(),
                        contents.next().unwrap().trim().to_string());
                },
                Ok(_) =>  break,
                Err(err) => panic!("error during receive a line: {}", err),
            }
        }

        headers
    }
}

#[cfg(test)]
mod RequestTest {
    use std::io::Read;
    use super::*;

    struct ReadMock<'a> {
        data: &'a [u8]
    }

    impl <'a> ReadMock<'a> {
        fn new() -> ReadMock<'a> {
            ReadMock {
                data: &b"GET / 1.1"
            }
        }
    }

    impl <'a> Read for ReadMock<'a> {
        fn read(&mut self, buf: &mut 'a [u8]) -> Result<usize, Error> {
            buf = self.data.next().unwrap();
            Ok(1)
        }
    }

    #[test]
    fn new() {
        let request = Request::new(ReadMock::new());
    }
}
