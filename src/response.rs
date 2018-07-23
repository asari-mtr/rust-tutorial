extern crate mime_guess;

use status_code::StatusCode;

pub type ResponseHeaders = Vec<String>;

pub trait WriteResponseHeaders {
    fn write_http_status_line(&mut self, status: StatusCode);
    fn write_content_type(&mut self, public_path: &str);
    fn write_content_length(&mut self, size: usize);
    fn write_content_encoding(&mut self);
}

impl WriteResponseHeaders for ResponseHeaders {
    fn write_http_status_line(&mut self, status: StatusCode) {
        self.push(format!("http/1.1 {} {}", status.to_u16(), status.status_comment().unwrap()));
    }

    fn write_content_type(&mut self, public_path: &str) {
        let mime = mime_guess::guess_mime_type(public_path).to_string();
        self.push(format!("content-type: {}; charset=utf-8", mime));
    }

    fn write_content_length(&mut self, size: usize) {
        self.push(format!("content-length: {}", size));
    }

    fn write_content_encoding(&mut self) {
        self.push("content-encoding: gzip".to_string());
    }
}

