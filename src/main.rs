extern crate time;

use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let mut stream = listener.accept().unwrap().0;
    let message = "Hello, World!";
    let format = "%a, %d %b %Y %T GMT";
    let time = time::now_utc();
    let response = format!("HTTP/1.1 200 OK\r\n
Date: {}\r\n
Content-Type: text/html; charset=utf-8\r\n
Content-Length: {}\r\n
\r\n
{}",
                            time::strftime(format, &time).unwrap(),
                            message.len(),
                            message);
    let _ = stream.write(response.as_bytes());
}
