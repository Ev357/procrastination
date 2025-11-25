use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use http_server_types::{error::Error, result::Result};

pub fn read_file(path: &Path) -> Result<(Vec<u8>, String)> {
    if path.is_dir() || !path.exists() {
        return Err(Error::NotFound(path.to_string_lossy().to_string()));
    }

    let file = File::open(path);

    let mut reader = BufReader::new(file?);

    let mut contents = Vec::new();

    reader.read_to_end(&mut contents)?;

    let content_type = get_content_type(path);

    Ok((contents, content_type))
}

fn get_content_type(path: &Path) -> String {
    match path.extension().and_then(|s| s.to_str()) {
        Some("html") => "text/html; charset=UTF-8",
        Some("css") => "text/css; charset=UTF-8",
        Some("js") => "application/javascript; charset=UTF-8",
        Some("json") => "application/json; charset=UTF-8",
        Some("png") => "image/png",
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("svg") => "image/svg+xml",
        Some("ico") => "image/x-icon",
        Some("webp") => "image/webp",
        Some("pdf") => "application/pdf",
        Some("txt") => "text/plain; charset=UTF-8",
        _ => "application/octet-stream",
    }
    .to_string()
}
