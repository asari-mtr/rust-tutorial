use std::thread;
use std::io;
use std::io::{Write, BufRead};
use std::net::{TcpListener, TcpStream, SocketAddr};

fn handle_client(stream: TcpStream, addr: SocketAddr) {
    let mut stream = io::BufReader::new(stream);

    let mut request_line = String::new();
    match stream.read_line(&mut request_line) {
        Ok(r) => {
            let mut iter = request_line.split_whitespace();
            let method = iter.next().unwrap();
            let uri = iter.next().unwrap();
            let version = iter.next().unwrap();
            println!("method: {}", method);
            println!("ip: {}", addr.ip());
            println!("{}", request_line);
            if method == "GET" && uri == "/" {
                loop {
                    let mut line = String::new();
                    match stream.read_line(&mut line) {
                        Ok(2) => {
                            let message = "<html><head><title>Hello</title></head><body>Hello World!</body></html>".to_string();
                            write_response(stream.get_mut(), message);
                            break;
                        },
                        Err(err) => panic!("error during receive a line: {}", err),
                        _ => println!("{}", line.trim_right_matches("\r\n"))
                    }
                }
            }
        },
        Err(err) => panic!("error during receive a line: {}", err),
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
            Err(e) => {
                println!("Connection fail!");
            }
        }
    }
}
