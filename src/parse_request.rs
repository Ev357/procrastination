use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    path::PathBuf,
};

use crate::{
    Error, Method,
    types::{request::Request, result::Result},
};

pub fn parse_request(request: &TcpStream) -> Result<Request> {
    let buffer = BufReader::new(request);

    let mut request = Option::<Request>::None;

    for (index, line) in buffer.lines().enumerate() {
        let line = line?;
        if line.is_empty() {
            break;
        }

        if index == 0 {
            let mut parts = line.split_whitespace();
            let method = Method::from(parts.next().ok_or(Error::InvalidMethod)?);
            let path = PathBuf::from(parts.next().ok_or(Error::InvalidPath)?);
            let version = parts.next().ok_or(Error::InvalidVersion)?.to_string();

            request = Some(Request {
                method,
                path,
                version,
            });
        }
    }

    request.ok_or(Error::ProtocolError)
}
