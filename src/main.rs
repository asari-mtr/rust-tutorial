use std::thread;
use std::str::SplitWhitespace;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufWriter, BufRead, BufReader};
use std::net::{TcpListener, TcpStream, SocketAddr};

extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::write::ZlibEncoder;

mod request;
use request::Request;

fn create_request(iter: &mut SplitWhitespace) -> Request {
    Request {
        method: iter.next().unwrap().to_string(),
        uri: iter.next().unwrap().to_string(),
        version: iter.next().unwrap().to_string(),
    }
}

fn debug_request(request: &Request) {
    println!("method: {}", request.method);
    println!("uri: {}", request.uri);
    println!("version: {}", request.version);
}

fn public_path(path: &str) -> String {
    let mut base = String::from("public");

    if path == "/" {
        base.push_str("/index.html")
    } else {
        base.push_str(path)
    }

    base
}

fn read_file(path: &str) -> Result<File, File> {
    match File::open(path) {
        Ok(f) => Ok(f),
        Err(err) => {
            Err(File::open("public/404.html").expect("File not found"))
        }
    }
}

fn response(request: Request, stream: &mut BufReader<TcpStream>) {
    loop {
        let mut line = String::new();
        let public_path = public_path(&request.uri);
        match stream.read_line(&mut line) {
            Ok(2) => {

                let (mut f, status) = match read_file(&public_path) {
                    Ok(f) => (f, 200),
                    Err(f) => (f, 404)
                };

                // println!("status: {}", status);
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .expect("something went wrong reading the file");

                write_response(stream.get_mut(), status, contents);
                break;
            },
            // Ok(size) => println!("{} {}", line.trim_right_matches("\r\n"), size),
            Ok(_) => (),
            Err(err) => panic!("error during receive a line: {}", err),
        }
    }
}

fn handle_client(stream: TcpStream, addr: SocketAddr) {
    let mut stream = BufReader::new(stream);

    let mut request_line = String::new();
    match stream.read_line(&mut request_line) {
        Ok(_) => {
            let request = create_request(&mut request_line.split_whitespace());
             debug_request(&request);
            // println!("ip: {}", addr.ip());
            ok_handler(request, &mut stream)
        },
        Err(err) => panic!("error during receive a line: {}", err),
    }
}

fn ok_handler(request: Request, stream: &mut BufReader<TcpStream>) {
    if request.method == "GET" {
        response(request, stream);
    }
}

fn write_response(stream: &mut TcpStream, status: u32, body: String) {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    e.write(body.as_bytes());
    let bs = match e.finish() {
        Ok(v) => v,
        Err(e) => panic!("fail encode to zip: {}", e)
    };

    writeln!(stream, "HTTP/1.1 {} Not Found", status);
    writeln!(stream, "Content-Type: text/html; charset=UTF-8");
    writeln!(stream, "Content-Length: {}", bs.len());
    writeln!(stream, "content-encoding: gzip");
    writeln!(stream);
    stream.write(&bs);
    // writeln!(stream, "{:?}", bs).unwrap();
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
            Err(_) => {
                println!("Connection fail!");
            }
        }
    }
}
