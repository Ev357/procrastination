use std::{
    io::Write,
    net::TcpStream,
    path::{Path, PathBuf},
};

use http_server_types::{
    error::Error, method::Method, request::Request, response::Response,
    response_status::ResponseStatus, result::Result,
};

use crate::{configuration::Configuration, utils::read_file::read_file};

pub fn send_response(
    stream: &mut TcpStream,
    request: &Request,
    configuration: &Configuration,
) -> Result<()> {
    let response: Vec<u8> = get_response(request, configuration)?.into();

    stream.write_all(&response)?;

    Ok(())
}

fn get_response(request: &Request, configuration: &Configuration) -> Result<Response> {
    match request.method {
        Method::Get => get_get_response(request, configuration),
        Method::Unknown => get_error_response(&Error::InvalidMethod),
    }
}

fn get_content_path(path: &Path, root: &Path) -> Result<PathBuf> {
    let relative_path = path.strip_prefix("/")?;

    let file_path = if relative_path.is_dir() {
        &relative_path.join("index.html")
    } else {
        relative_path
    };

    let content_path = root.join(file_path);

    Ok(content_path)
}

fn get_get_response(request: &Request, configuration: &Configuration) -> Result<Response> {
    let content_path = get_content_path(&request.path, &configuration.path)?;

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
