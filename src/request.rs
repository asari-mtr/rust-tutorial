use std::io::{BufRead, BufReader, Error, Read};

use std::collections::HashMap;

use http_method::*;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub uri: String,
    pub version: String,
    pub headers: RequestHeaders,
}

type RequestHeaders = HashMap<String, String>;

impl Request {
    pub fn new<R: Read>(stream: R) -> Result<Request, String> {
        let mut stream = BufReader::new(stream);
        let mut request_line = String::new();
        if let Err(err) = stream.read_line(&mut request_line) {
            return Err(err.to_string());
        };

        let mut iter = request_line.split_whitespace();
        let n: Vec<&str> = iter.collect();
        let method = match HttpMethod::from_str(n[0]) {
            Some(method) => method,
            None => return Err(String::from("Invalid method")),
        };
        if n.len() == 3 {
            Ok(Request {
                method: method,
                uri: String::from(n[1]),
                version: String::from(n[2]),
                headers: Request::create_header(&mut stream),
            })
        } else {
            Err(String::from("Invalid request line"))
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
                        contents.next().unwrap().trim().to_string(),
                    );
                }
                Ok(_) => break,
                Err(err) => panic!("error during receive a line: {}", err),
            }
        }

        headers
    }
}

#[cfg(test)]
mod request_test {
    extern crate stringreader;
    use self::stringreader::StringReader;
    use super::*;

    #[test]
    fn new_test() {
        let request = Request::new(StringReader::new("GET / HTTP/1.1\n")).unwrap();

        assert_eq!(HttpMethod::GET, request.method);
        assert_eq!("/", request.uri);
        assert_eq!("HTTP/1.1", request.version);
    }

    #[test]
    fn new_test_when_invalid() {
        if let Err(err) = Request::new(StringReader::new("GET /\n")) {
            assert_eq!("Invalid request line", err);
        }
    }

    #[test]
    fn new_test_when_invalid_method() {
        if let Err(err) = Request::new(StringReader::new("TAKE / HTTP/1.1\n")) {
            assert_eq!("Invalid method", err);
        }
    }
}
