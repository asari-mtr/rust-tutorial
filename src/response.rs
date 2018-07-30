extern crate mime_guess;

use constants::*;
use flate2::Compression;
use flate2::write::GzEncoder;
use request::Request;
use status_code::StatusCode;
use std::fs;
use std::io::{Write, BufWriter};
use std::net::{TcpStream};
use std::path::Path;

pub type ResponseHeaders = Vec<String>;

pub trait WriteResponseHeaders {
    fn write_http_status_line(&mut self, status: StatusCode);
    fn write_content_type(&mut self, public_path: &str);
    fn write_content_length(&mut self, size: usize);
    fn write_content_encoding(&mut self);
}

impl WriteResponseHeaders for ResponseHeaders {
    fn write_http_status_line(&mut self, status: StatusCode) {
        self.push(format!("http/1.1 {} {}", status.to_u16(), status.status_comment().unwrap()));
    }

    fn write_content_type(&mut self, public_path: &str) {
        let mime = mime_guess::guess_mime_type(public_path).to_string();
        self.push(format!("content-type: {}; charset=utf-8", mime));
    }

    fn write_content_length(&mut self, size: usize) {
        self.push(format!("content-length: {}", size));
    }

    fn write_content_encoding(&mut self) {
        self.push(format!("content-encoding: {}", GZIP));
    }
}

pub fn response(request: Request, stream: TcpStream) {
    let public_path = public_path(&request.uri);

    let (public_path, status) = valid_file_path(&public_path);

    let data = read_data(&request, &public_path);

    let headers = create_response_headers(&request, status, &public_path, &data);

    write_response(stream, headers, data);
}

fn public_path(path: &str) -> String {
    if path.ends_with("/") {
        vec![ROOT_DIR, path, "index.html"].concat()
    } else {
        vec![ROOT_DIR, path].concat()
    }
}

fn valid_file_path(path_str: &str) -> (String, StatusCode) {
    let path = Path::new(&path_str);

    if path.exists() && path.is_file() {
        (path_str.to_string(), StatusCode::Ok)
    } else {
        (public_path(&"/404.html".to_string()).to_string(), StatusCode::NotFound)
    }
}

fn read_data(request: &Request, public_path: &str) -> Vec<u8> {
    let data = fs::read(&public_path).expect("Unable to read file");
    match request.headers.get(ACCEPT_ENCODING) {
        Some(keys) => {
            // It needs to use a more accurate match method.
            if keys.contains(GZIP) {
                let mut e = GzEncoder::new(Vec::new(), Compression::default());

                e.write(&data).unwrap();
                match e.finish() {
                    Ok(v) => v,
                    Err(e) => panic!("fail encode to zip: {}", e)
                }
            } else {
                data
            }
        },
        None => data
    }
}

fn create_response_headers(request: &Request, status: StatusCode, public_path: &str, data: &Vec<u8>) -> ResponseHeaders {
    let mut headers = ResponseHeaders::new();
    headers.write_http_status_line(status);
    headers.write_content_type(&public_path);
    headers.write_content_length(data.len());
    match request.headers.get(ACCEPT_ENCODING) {
        Some(keys) if keys.contains(GZIP) => headers.write_content_encoding(),
        _ => ()
    }
    headers
}

fn write_response(stream: TcpStream, headers: ResponseHeaders, data: Vec<u8>) {
    let mut stream = BufWriter::new(stream);

    for header in headers {
        writeln!(stream, "{}", header).unwrap();
    }
    writeln!(stream).unwrap();

    stream.write(&data).unwrap();
}
