#[derive(Debug)]
pub enum Method {
    Get,
    Unknown,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "GET" => Method::Get,
            _ => Method::Unknown,
        }
    }
}
