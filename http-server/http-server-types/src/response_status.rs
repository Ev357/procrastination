#[derive(Debug)]
pub enum ResponseStatus {
    Ok,
    NotFound,
    InternalServerError,
}

impl From<ResponseStatus> for &str {
    fn from(status: ResponseStatus) -> Self {
        match status {
            ResponseStatus::Ok => "200 OK",
            ResponseStatus::NotFound => "404 Not Found",
            ResponseStatus::InternalServerError => "500 Internal Server Error",
        }
    }
}
