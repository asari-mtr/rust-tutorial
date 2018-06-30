use std::thread;
use std::str::SplitWhitespace;
use std::io::{Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream, SocketAddr};

struct Request {
    method: String,
    uri: String,
    version: String,
}

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

fn response(stream: &mut BufReader<TcpStream>) {
    loop {
        let mut line = String::new();
        match stream.read_line(&mut line) {
            Ok(2) => {
                let message = "<html><head><title>Hello</title></head><body>Hello World!</body></html>".to_string();
                write_response(stream.get_mut(), message);
                break;
            },
            _ => println!("{}", line.trim_right_matches("\r\n")),
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
            println!("ip: {}", addr.ip());
            ok_handler(request, &mut stream)
        },
        Err(err) => panic!("error during receive a line: {}", err),
    }
}

fn ok_handler(request: Request, stream: &mut BufReader<TcpStream>) {
    if request.method == "GET" && request.uri == "/" {
        response(stream);
    }
}

fn write_response(stream: &mut TcpStream, body: String) {
    writeln!(stream, "HTTP/1.1 200 OK").unwrap();
    writeln!(stream, "Content-Type: text/html; charset=UTF-8").unwrap();
    writeln!(stream, "Content-Length: {}", body.len()).unwrap();
    writeln!(stream).unwrap();
    writeln!(stream, "{}", body).unwrap();
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
