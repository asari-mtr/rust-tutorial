extern crate flate2;

mod constants;
mod request;
mod response;
mod status_code;
mod http_method;

use constants::*;
use request::Request;
use response::*;
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;

fn dispatch(stream: TcpStream, _addr: SocketAddr) {
    let request = Request::new(&stream);
    println!("{:?}", request);

    if request.method.is_get() {
        response(request, stream);
    }
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
