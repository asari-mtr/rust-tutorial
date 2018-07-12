use std::thread;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream, SocketAddr};

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
        Err(err) => Err(File::open("public/404.html").expect("File not found")) 
    }
}

fn response(request: Request, stream: &mut BufReader<TcpStream>) {
    let mut line = String::new();
    let public_path = public_path(&request.uri);
    loop {
        match stream.read_line(&mut line) {
            Ok(_) => {
                let (mut f, status) = match read_file(&public_path) {
                    Ok(f) => (f, OK),
                    Err(f) => (f, NOT_FOUND)
                };

                // println!("status: {}", status);
                let mut contents = String::new();
                f.read_to_string(&mut contents)
                    .expect("something went wrong reading the file");

                write_response(stream.get_mut(), status, contents);
                break;
            },
            Err(err) => panic!("error during receive a line: {}", err),
        }
    }
}

fn handle_client(stream: TcpStream, addr: SocketAddr) {
    let mut stream = BufReader::new(stream);

    let mut request_line = String::new();

    let request = match stream.read_line(&mut request_line) {
        Ok(_) => {
            let request = Request::create_request(&request_line);
            request.debug_request();
            request
        },
        Err(err) => panic!("error during receive a line: {}", err),
    };

    // let mut array: Vec<String> = create_header(stream.get_mut());
    // println!("ip: {}", addr.ip());
    let mut headers: Vec<String>  = vec!();

    loop {
        let mut request_line = String::new();
        match stream.read_line(&mut request_line) {
            Ok(size) if size> 2 => {
                println!("header: {}", request_line);
                headers.push(request_line.to_string())
            },
            Ok(_) =>  break,
            Err(err) => panic!("error during receive a line: {}", err),
        }
    }

    // return headers
    
    dispatch(request, &mut stream);
}

// fn create_header(stream: &mut TcpStream) -> Vec<String> {
//     loop {
//         println!("read");
//         match stream.read_line(&mut line) {
//             Ok(size) if size> 2 => {
//                 println!("header: {}", line);
//                 headers.push(line.to_string());
//                 break;
//             },
//             Ok(size) => {
//                 println!("header: {}({})", line, size);
//                 break;
//             },
//             Err(err) => panic!("error during receive a line: {}", err),
//         }
//     }
//     println!("create_header finish");
//     return headers
// }

fn dispatch(request: Request, stream: &mut BufReader<TcpStream>) {
    if request.method == "GET" {
        response(request, stream);
    }
}

fn write_response(stream: &mut TcpStream, status: StatusCode, body: String) {
    let mut e = GzEncoder::new(Vec::new(), Compression::default());

    e.write(body.as_bytes());
    let bs = match e.finish() {
        Ok(v) => v,
        Err(e) => panic!("fail encode to zip: {}", e)
    };

    writeln!(stream, "HTTP/1.1 {} {}", status, status_comment(status));
    writeln!(stream, "Content-Type: text/html; charset=UTF-8");
    writeln!(stream, "Content-Length: {}", bs.len());
    writeln!(stream, "content-encoding: gzip");
    writeln!(stream);
    stream.write(&bs);
    // writeln!(stream, "{:?}", bs).unwrap();
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
