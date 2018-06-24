use std::thread;
use std::io;
use std::io::{Write, BufRead};
use std::net::{TcpListener, TcpStream, SocketAddr};

fn handle_client(stream: TcpStream, addr: SocketAddr) {
    let mut stream = io::BufReader::new(stream);
    let message = "<html><head><title>Hello</title></head><body>Hello World!</body></html>";

    println!("ip: {}", addr.ip());
    println!("port: {}", addr.port());

    loop {
        let mut line = String::new();
        match stream.read_line(&mut line) {
            Ok(2) => {
                let mut stream = stream.get_mut();
                writeln!(stream, "HTTP/1.1 200 OK").unwrap();
                writeln!(stream, "Content-Type: text/html; charset=UTF-8").unwrap();
                writeln!(stream, "Content-Length: {}", message.len()).unwrap();
                writeln!(stream).unwrap();
                writeln!(stream, "{}", message).unwrap();
                break;
            },
            Err(err) => panic!("error during receive a line: {}", err),
            _ => println!("{}", line.trim_right_matches("\r\n"))
        }
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
            Err(e) => {
                println!("Connection fail!");
            }
        }
    }
}
