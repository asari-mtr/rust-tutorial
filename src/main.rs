use std::thread;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream, SocketAddr};

use std::collections::HashMap;

extern crate flate2;
use flate2::Compression;
use flate2::write::GzEncoder;

mod request;
use request::*;

type StatusCode = u32;

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

fn read_file(path: &str) -> Result<File, File> {
    match File::open(path) {
        Ok(f) => Ok(f),
        Err(_) => Err(File::open("public/404.html").expect("File not found"))
    }
}

fn handle_client(stream: TcpStream, _addr: SocketAddr) {
    let mut stream = BufReader::new(stream);

    let mut request_line = String::new();
    let request = match stream.read_line(&mut request_line) {
        Ok(_) => Request::create_request(&request_line),
        Err(err) => panic!("error during receive a line: {}", err),
    };
    request.debug_request();
    create_header(&mut stream);
    dispatch(request, stream.get_mut());
}

fn create_header(stream: &mut BufReader<TcpStream>) -> HashMap<String, String> {
    let mut hash: HashMap<String, String>  = HashMap::new();

    loop {
        let mut request_line = String::new();
        match stream.read_line(&mut request_line) {
            Ok(size) if size > 2 => {
                // TODO: if fail to  split?
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

fn dispatch(request: Request, stream: &mut TcpStream) {
    if request.method == "GET" {
        response(request, stream);
    }
}

fn response(request: Request, stream: &mut TcpStream) {
    let public_path = public_path(&request.uri);
    let (f, status) = match read_file(&public_path) {
        Ok(f) => (f, OK),
        Err(f) => (f, NOT_FOUND)
    };

    // let mut body = String::new();
    // file.read_to_string(&mut body)
    //     .expect("something went wrong reading the file");

    let data = fs::read(&public_path).expect("Unable to read file");

    writeln!(stream, "HTTP/1.1 {} {}", status, status_comment(status)).unwrap();
    writeln!(stream, "Content-Type: image/jpg; charset=UTF-8").unwrap();
    writeln!(stream, "Content-Length: {}", data.len()).unwrap();
    writeln!(stream, "Content-encoding: gzip").unwrap();
    writeln!(stream).unwrap();

    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    e.write(&data).unwrap();
    let bs = match e.finish() {
        Ok(v) => v,
        Err(e) => panic!("fail encode to zip: {}", e)
    };

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
