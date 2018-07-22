use std::thread;
use std::fs;
use std::io::{Write, BufWriter};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::path::Path;

extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;

extern crate mime_guess;

mod request;
use request::Request;

mod status_code;
use status_code::StatusCode;

type ResponseHeaders = Vec<String>;

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
    let request = Request::new(&stream);

    request.debug_request();
    dispatch(request, stream);
}

fn dispatch(request: Request, stream: TcpStream) {
    if request.method == "GET" {
        response(request, stream);
    }
}

fn write_http_status_line(headers: &mut ResponseHeaders, status: StatusCode) {
    headers.push(format!("HTTP/1.1 {} {}", status.to_u16(), status.status_comment().unwrap()));
}

fn write_content_type(public_path: &str, headers: &mut ResponseHeaders) {
    let mime = mime_guess::guess_mime_type(public_path).to_string();
    headers.push(format!("Content-Type: {}; charset=UTF-8", mime));
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
        (public_path, StatusCode::Ok)
    } else {
        ("public/404.html".to_string(), StatusCode::NotFound)
    };

    println!("{}", public_path);

    let data = fs::read(&public_path).expect("Unable to read file");
    let data = match request.headers.get("Accept-Encoding") {
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
    };

    let mut headers = ResponseHeaders::new();
    write_http_status_line(&mut headers, status);
    write_content_type(&public_path, &mut headers);
    write_content_length(&mut headers, data.len());
    write_content_encoding(&mut headers);

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
                    handle_client(stream, addr)
                });
            }
            Err(_) => println!("Connection fail!")
        }
    }
}
