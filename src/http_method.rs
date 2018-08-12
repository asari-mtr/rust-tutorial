#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

impl HttpMethod {
    pub fn from_str(s: &str) -> Option<HttpMethod> {
        match s {
            "GET" => Some(HttpMethod::GET),
            "HEAD" => Some(HttpMethod::HEAD),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::DELETE),
            "CONNECT" => Some(HttpMethod::CONNECT),
            "OPTIONS" => Some(HttpMethod::OPTIONS),
            "TRACE" => Some(HttpMethod::TRACE),
            "PATCH" => Some(HttpMethod::PATCH),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::HEAD => "HEAD",
            HttpMethod::POST => "POST",
            HttpMethod::PUT => "PUT",
            HttpMethod::DELETE => "DELETE",
            HttpMethod::CONNECT => "CONNECT",
            HttpMethod::OPTIONS => "OPTIONS",
            HttpMethod::TRACE => "TRACE",
            HttpMethod::PATCH => "PATCH",
        }
    }

    pub fn is_get(&self) -> bool {
        self.to_str() == "GET"
    }

    pub fn is_head(&self) -> bool {
        self.to_str() == "HEAD"
    }

    pub fn is_post(&self) -> bool {
        self.to_str() == "POST"
    }

    pub fn is_put(&self) -> bool {
        self.to_str() == "PUT"
    }

    pub fn is_delete(&self) -> bool {
        self.to_str() == "DELETE"
    }

    pub fn is_connect(&self) -> bool {
        self.to_str() == "CONNECT"
    }

    pub fn is_options(&self) -> bool {
        self.to_str() == "OPTIONS"
    }

    pub fn is_trace(&self) -> bool {
        self.to_str() == "TRACE"
    }

    pub fn is_patch(&self) -> bool {
        self.to_str() == "PATCH"
    }
}
