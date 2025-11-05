use crate::types::response_status::ResponseStatus;

#[derive(Debug)]
pub struct Response {
    pub status: ResponseStatus,
    pub content_type: String,
    pub body: Vec<u8>,
}

impl From<Response> for Vec<u8> {
    fn from(response: Response) -> Self {
        let status: &str = response.status.into();
        let header = format!("HTTP/1.1 {}\r\n", status);

        let content_type = format!("Content-Type: {}\r\n", response.content_type);

        let content_length = format!("Content-Length: {}\r\n", response.body.len());

        const SERVER: &str = "Server: Small Rust HTTP server 🦀\r\n";

        const CONNECTION: &str = "Connection: close\r\n";

        let headers = format!(
            "{}{}{}{}{}\r\n",
            header, content_type, content_length, SERVER, CONNECTION
        );

        let mut vector = Vec::new();
        vector.extend_from_slice(headers.as_bytes());
        vector.extend_from_slice(&response.body);

        vector
    }
}
