use std::thread;
use std::fs;
use std::io::{Write, BufWriter};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::path::Path;

extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;

mod request;
use request::*;

type StatusCode = u32;
type ResponseHeaders = Vec<String>;

const OK:           StatusCode = 200;
const NOT_FOUND:    StatusCode = 404;

fn public_path(path: &str) -> String {
    let mut base = String::from("public");

    if path == "/" {
        base.push_str("/index.html")
    } else {
        base.push_str(path)
    }

    base
}

fn handle_client(stream: TcpStream, _addr: SocketAddr) {
    let request = Request::init(&stream);

    request.debug_request();
    dispatch(request, stream);
}

fn dispatch(request: Request, stream: TcpStream) {
    if request.method == "GET" {
        response(request, stream);
    }
}

fn write_http_status_line(headers: &mut ResponseHeaders, status: StatusCode) {
    headers.push(format!("HTTP/1.1 {} {}", status, status_comment(status)));
}

fn write_content_type(headers: &mut ResponseHeaders) {
    headers.push("Content-Type: image/jpg; charset=UTF-8".to_string());
}

fn write_content_length(headers: &mut ResponseHeaders, size: usize) {
    headers.push(format!("Content-Length: {}", size));
}

fn write_content_encoding(headers: &mut ResponseHeaders) {
    headers.push("Content-encoding: gzip".to_string());
}

fn response(request: Request, stream: TcpStream) {
    let public_path = public_path(&request.uri);

    let (public_path, status) = if Path::new(&public_path).exists() {
        (public_path, OK)
    } else {
        ("public/404.html".to_string(), NOT_FOUND)
    };

    println!("{}", public_path);

    // let mut body = String::new();
    // file.read_to_string(&mut body)
    //     .expect("something went wrong reading the file");

    let data = fs::read(&public_path).expect("Unable to read file");

    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    e.write(&data).unwrap();
    let bs = match e.finish() {
        Ok(v) => v,
        Err(e) => panic!("fail encode to zip: {}", e)
    };

    let mut headers = ResponseHeaders::new();
    write_http_status_line(&mut headers, status);
    write_content_type(&mut headers);
    write_content_length(&mut headers, data.len());
    write_content_encoding(&mut headers);

    let mut stream = BufWriter::new(stream);

    for header in headers {
        writeln!(stream, "{}", header).unwrap();
    }
    writeln!(stream).unwrap();

    stream.write(&bs).unwrap();
}

fn status_comment(status: StatusCode) -> String {
    match status {
        OK              => String::from("OK"),
        NOT_FOUND       => String::from("Not Found"),
        _               => String::from("")
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                thread::spawn(move || {
                    handle_client(stream, addr)
                });
            }
            Err(_) => println!("Connection fail!")
        }
    }
}
