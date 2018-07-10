use std::str::SplitWhitespace;

pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
}

impl Request {
    pub fn create_request(iter: &mut SplitWhitespace) -> Request {
        Request {
            method: iter.next().unwrap().to_string(),
            uri: iter.next().unwrap().to_string(),
            version: iter.next().unwrap().to_string(),
        }
    }

    pub fn debug_request(&self) {
        println!("method: {}", self.method);
        println!("uri: {}", self.uri);
        println!("version: {}", self.version);
    }
}
