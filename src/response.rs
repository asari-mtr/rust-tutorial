extern crate mime_guess;

use constants::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use request::Request;
use status_code::StatusCode;
use std::fs;
use std::io::Error;
use std::io::{BufWriter, Write};
use std::path::Path;

pub type ResponseHeaders = Vec<String>;

pub trait WriteResponseHeaders {
    fn write_http_status_line(&mut self, status: StatusCode);
    fn write_content_type(&mut self, public_path: &str);
    fn write_content_length(&mut self, size: usize);
    fn write_content_encoding(&mut self);
}

impl WriteResponseHeaders for ResponseHeaders {
    fn write_http_status_line(&mut self, status: StatusCode) {
        self.push(format!(
            "http/1.1 {} {}",
            status.to_u16(),
            status.status_comment().unwrap()
        ));
    }

    fn write_content_type(&mut self, public_path: &str) {
        let mime = mime_guess::guess_mime_type(public_path).to_string();
        self.push(format!("content-type: {}; charset=utf-8", mime));
    }

    fn write_content_length(&mut self, size: usize) {
        self.push(format!("content-length: {}", size));
    }

    fn write_content_encoding(&mut self) {
        self.push(format!("content-encoding: {}", GZIP));
    }
}

#[cfg(test)]
mod write_response_headers_test {
    use super::*;

    #[test]
    fn write_http_status_line() {
        let mut headers = ResponseHeaders::new();
        headers.write_http_status_line(StatusCode::Ok);
        assert_eq!("http/1.1 200 OK", headers[0]);
    }

    #[test]
    fn write_content_type_with_html() {
        let mut headers = ResponseHeaders::new();
        headers.write_content_type("hoge.html");
        assert_eq!("content-type: text/html; charset=utf-8", headers[0]);
    }

    #[test]
    #[ignore]
    fn write_content_type_with_png() {
        let mut headers = ResponseHeaders::new();
        headers.write_content_type("hoge.png");
        assert_eq!("content-type: image/png", headers[0]);
    }

    #[test]
    fn write_content_length() {
        let mut headers = ResponseHeaders::new();
        headers.write_content_length(12);
        assert_eq!("content-length: 12", headers[0]);
    }

    #[test]
    fn write_content_encoding() {
        let mut headers = ResponseHeaders::new();
        headers.write_content_encoding();
        assert_eq!("content-encoding: gzip", headers[0]);
    }
}

pub fn response<W: Write>(request: Request, stream: W) -> Result<(), Error> {
    let public_path = public_path(&request.uri);

    let (public_path, status) = valid_file_path(&public_path);

    let data = read_data(&request, &public_path)?;

    let headers = create_response_headers(&request, status, &public_path, &data);

    write_response(stream, headers, data);

    Ok(())
}

fn public_path(path: &str) -> String {
    let sep = if path.starts_with("/") { "" } else { "/" };
    if path.ends_with("/") {
        vec![ROOT_DIR, sep, path, "index.html"].concat()
    } else {
        vec![ROOT_DIR, sep, path].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_path_with_slash() {
        assert_eq!("public/toc.html", public_path("/toc.html"));
    }

    #[test]
    fn file_path_without_slash() {
        assert_eq!("public/toc.html", public_path("toc.html"));
    }

    #[test]
    fn dir_path_with_slash() {
        assert_eq!("public/image/index.html", public_path("/image/"));
    }

    #[test]
    fn dir_path_without_slash() {
        assert_eq!("public/image/index.html", public_path("image/"));
    }
}

fn valid_file_path(path_str: &str) -> (String, StatusCode) {
    let path = Path::new(&path_str);

    if path.exists() && path.is_file() {
        (path_str.to_string(), StatusCode::Ok)
    } else {
        (public_path("/404.html"), StatusCode::NotFound)
    }
}

#[cfg(test)]
mod valid_file_path_test {
    use super::*;

    #[test]
    fn valid_file_path_return_ok() {
        let (path, status) = valid_file_path("public/index.html");
        assert_eq!("public/index.html", path);
        assert_eq!(StatusCode::Ok, status);
    }

    #[test]
    fn valid_file_path_return_not_found() {
        let (path, status) = valid_file_path("public/hoge.html");
        assert_eq!("public/404.html", path);
        assert_eq!(StatusCode::NotFound, status);
    }
}

fn read_data(request: &Request, public_path: &str) -> Result<Vec<u8>, Error> {
    let data = fs::read(&public_path)?;
    match request.headers.get(ACCEPT_ENCODING) {
        Some(keys) if keys.contains(GZIP) => {
            // It needs to use a more accurate match method.
            let mut e = GzEncoder::new(Vec::new(), Compression::default());

            e.write(&data).unwrap();
            e.finish()
        }
        _ => Ok(data),
    }
}

fn create_response_headers(
    request: &Request,
    status: StatusCode,
    public_path: &str,
    data: &Vec<u8>,
) -> ResponseHeaders {
    let mut headers = ResponseHeaders::new();
    headers.write_http_status_line(status);
    headers.write_content_type(&public_path);
    headers.write_content_length(data.len());
    if let Some(keys) = request.headers.get(ACCEPT_ENCODING) {
        if keys.contains(GZIP) {
            headers.write_content_encoding()
        }
    }
    headers
}

fn write_response<W: Write>(stream: W, headers: ResponseHeaders, data: Vec<u8>) {
    let mut stream = BufWriter::new(stream);

    for header in headers {
        writeln!(stream, "{}", header).unwrap();
    }
    writeln!(stream).unwrap();

    stream.write(&data).unwrap();
}
