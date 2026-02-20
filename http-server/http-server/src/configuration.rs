use std::path::PathBuf;

use http_server_codegen::FromHashMap;

#[derive(Debug, FromHashMap)]
pub struct Configuration {
    pub path: PathBuf,
    pub port: u16,
}
