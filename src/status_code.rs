#[derive(Debug)]
#[derive(PartialEq)]
pub enum StatusCode {
    // 200 Ok
    Ok,
    // 404 Not Found
    NotFound,
    // Unregistered
    Unregistered(u16)
}

impl StatusCode {
    pub fn from_u16(n: u16) -> StatusCode {
        match n {
            200 => StatusCode::Ok,
            404 => StatusCode::NotFound,
            _ => StatusCode::Unregistered(n)
        }
    }

    pub fn to_u16(&self) -> u16 {
        match *self {
            StatusCode::Ok => 200,
            StatusCode::NotFound => 404,
            StatusCode::Unregistered(n) => n
        }
    }

    pub fn status_comment(&self) -> Option<&'static str> {
        match *self {
            StatusCode::Ok => Some("OK"),
            StatusCode::NotFound => Some("Not Found"),
            StatusCode::Unregistered(..) => None
        }
    }
}
