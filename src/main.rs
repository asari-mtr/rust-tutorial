use std::thread;
use std::fs;
use std::io::{Write, BufWriter};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::path::Path;

extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;

mod request;
use request::Request;

mod response;
use response::*;

mod status_code;
use status_code::StatusCode;

fn public_path(path: &str) -> String {
    let mut base = String::from("public");

    if path == "/" {
        base.push_str("/index.html")
    } else {
        base.push_str(path)
    }

    base
}

fn dispatch(stream: TcpStream, _addr: SocketAddr) {
    let request = Request::new(&stream);
    request.debug_request();

    if request.method == "GET" {
        response(request, stream);
    }
}

fn response(request: Request, stream: TcpStream) {
    let public_path = public_path(&request.uri);

    let (public_path, status) = valid_file_path(&public_path);

    let data = read_data(request, &public_path);

    let headers = create_response_headers(status, &public_path, &data);

    write_response(stream, headers, data);
}

fn valid_file_path(public_path: &str) -> (String, StatusCode) {
    if Path::new(&public_path).exists() {
        (public_path.to_string(), StatusCode::Ok)
    } else {
        ("public/404.html".to_string(), StatusCode::NotFound)
    }
}

fn read_data(request: Request, public_path: &str) -> Vec<u8> {
    let data = fs::read(&public_path).expect("Unable to read file");
    match request.headers.get("Accept-Encoding") {
        Some(keys) => {
            // It needs to use a more accurate match method.
            if keys.contains("gzip") {
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

fn create_response_headers(status: StatusCode, public_path: &str, data: &Vec<u8>) -> ResponseHeaders {
    let mut headers = ResponseHeaders::new();
    headers.write_http_status_line(status);
    headers.write_content_type(&public_path);
    headers.write_content_length(data.len());
    headers.write_content_encoding();
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

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                thread::spawn(move || {
                    dispatch(stream, addr)
                });
            }
            Err(_) => println!("Connection fail!")
        }
    }
}
