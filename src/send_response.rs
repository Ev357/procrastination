use std::{
    io::Write,
    net::TcpStream,
    path::{Path, PathBuf},
};

use crate::{
    types::{
        error::Error, method::Method, request::Request, response::Response,
        response_status::ResponseStatus, result::Result,
    },
    utils::read_file::read_file,
};

pub fn send_response(stream: &mut TcpStream, request: &Request) -> Result<()> {
    let response: Vec<u8> = get_response(request)?.into();

    stream.write_all(&response)?;

    Ok(())
}

fn get_response(request: &Request) -> Result<Response> {
    match request.method {
        Method::Get => get_get_response(request),
        Method::Unknown => get_error_response(&Error::InvalidMethod),
    }
}

fn get_content_path(path: &Path) -> Result<PathBuf> {
    let prefix = Path::new("./dist");

    if path == Path::new("/") {
        return Ok(prefix.join("index.html"));
    }

    let content_path = prefix.join(path.strip_prefix("/")?);

    Ok(content_path)
}

fn get_get_response(request: &Request) -> Result<Response> {
    let content_path = get_content_path(&request.path)?;

    let body = read_file(&content_path);

    if let Err(error) = body {
        return get_error_response(&error);
    }

    let (body, content_type) = body?;

    Ok(Response {
        status: ResponseStatus::Ok,
        content_type,
        body,
    })
}

fn get_error_response(error: &Error) -> Result<Response> {
    match error {
        Error::NotFound(_) => {
            let (error_page, content_type) = read_file(Path::new("./dist/404.html"))?;

            Ok(Response {
                status: ResponseStatus::NotFound,
                content_type,
                body: error_page,
            })
        }
        _ => {
            let (body, content_type) = read_file(Path::new("./dist/500.html"))?;

            Ok(Response {
                status: ResponseStatus::InternalServerError,
                content_type,
                body,
            })
        }
    }
}
