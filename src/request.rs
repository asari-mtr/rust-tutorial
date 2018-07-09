pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
}

impl Request {
    pub fn debug_request(&self) {
        println!("method: {}", self.method);
        println!("uri: {}", self.uri);
        println!("version: {}", self.version);
    }
}
